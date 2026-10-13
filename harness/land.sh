#!/usr/bin/env bash
# Lands an architect branch onto the swarm's integration branch:
#   harness/land.sh [BRANCH]   (default: arch)
# Stops the swarm (drains in-flight attempts), merges BRANCH, runs the full
# test262 suite against the pre-merge baseline, and refuses on regressions
# (the merge is undone). Otherwise writes the new baseline, commits, pushes
# and restarts the swarm. Run from the main checkout.
set -euo pipefail
cd "${KAHVI_ROOT:-$(dirname "$0")/..}"
BRANCH="${1:-arch}"
WORK="${KAHVI_WORK:-$PWD/work}"
current=$(git rev-parse --abbrev-ref HEAD)

if [ -f "$WORK/orch.pid" ] && ps -p "$(cat "$WORK/orch.pid")" >/dev/null 2>&1; then
  ./orch stop
  while ps -p "$(cat "$WORK/orch.pid")" >/dev/null 2>&1; do sleep 5; done
fi

before=$(git rev-parse HEAD)
# vendor/ is protected: never land a branch that changes it.
if ! git diff --quiet "$before" "$BRANCH" -- vendor; then
  echo "land: $BRANCH changes vendor/; refusing" >&2
  exit 1
fi
git show HEAD:conformance/expected-pass.txt > "$WORK/land-baseline.txt"
git merge --no-edit -q "$BRANCH"
cargo build -q -p kahvi-cli
if ! ./t262 built-ins/ language/ annexB/ intl402/ staging/ harness/ --show 20 \
     --baseline "$WORK/land-baseline.txt" --write-baseline conformance/expected-pass.txt > "$WORK/land.log" 2>&1; then
  grep -E "pass |regress" "$WORK/land.log" || true
  echo "land: regressions; undoing the merge" >&2
  git checkout -q -- conformance
  git reset -q --hard "$before"
  git submodule update --init -q
  exit 1
fi
grep -E "pass |baseline" "$WORK/land.log"
if ! git diff --quiet -- conformance; then
  git add conformance/expected-pass.txt
  git commit -q -m "Baseline after merging $BRANCH

$(grep -E 'pass ' "$WORK/land.log" | head -1 | sed 's/^ *//')"
fi
git push -q origin "$current"
nohup ./orch run > "$WORK/orch.log" 2>&1 &
echo $! > "$WORK/orch.pid"
echo "land: $BRANCH merged into $current and pushed; swarm restarted"
