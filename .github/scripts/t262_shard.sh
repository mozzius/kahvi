#!/usr/bin/env bash
# Runs one shard of the full test262 suite against the baseline:
#   t262_shard.sh SHARD COUNT OUT_PREFIX [t262 run options]...
# Writes OUT_PREFIX-summary.json and OUT_PREFIX-results.jsonl (per-run
# statuses, which the merge step reads to tell timeouts from failures).
# KAHVI_ENGINE overrides the engine command, e.g. to set environment: the
# runner clears the environment of every engine process it starts.
# Files are dealt round-robin from the sorted list, so every file lands in
# exactly one shard and slow directories (decodeURI, RegExp property
# escapes) spread across all of them. Exit codes are t262's own: 1 on a
# regression, 3 when the canaries say the run is invalid.
set -euo pipefail
shard=$1 count=$2 out=$3
shift 3
t262=${T262:-target/release/t262}
engine=${KAHVI_ENGINE:-target/release/kahvi}

"$t262" run --engine "$engine" --list > t262-all-files.txt
awk -v s="$shard" -v n="$count" '(NR - 1) % n == s' t262-all-files.txt > t262-shard-files.txt
echo "shard $shard/$count: $(wc -l < t262-shard-files.txt) of $(wc -l < t262-all-files.txt) files"
mapfile -t files < t262-shard-files.txt
# No filters would mean the whole suite, so an empty shard is an error.
if [ "${#files[@]}" -eq 0 ]; then
  echo "shard $shard/$count is empty; lower the shard count" >&2
  exit 2
fi
exec "$t262" run --engine "$engine" --summary-json "$out-summary.json" --json "$out-results.jsonl" "$@" "${files[@]}"
