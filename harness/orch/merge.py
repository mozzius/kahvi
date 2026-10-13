"""The merge bot: replays an accepted attempt onto the tip of the integration
branch, runs the full suite, and advances the branch only if nothing that
passed before fails now. The baseline (conformance/expected-pass.txt) is
rewritten in the same commit, so the pass count can only ratchet up."""

from pathlib import Path

from common import ROOT, WORK, git, log, run, write_json
from gates import cargo, read_results, run_t262

MERGE_DIR = WORK / "merge"


def prepare(tip):
    if not (MERGE_DIR / ".git").exists():
        git("worktree", "add", "--detach", str(MERGE_DIR), tip)
    git("cherry-pick", "--abort", cwd=MERGE_DIR, check=False)
    git("checkout", "--detach", "--force", tip, cwd=MERGE_DIR)
    git("clean", "-fdx", "-e", "target", cwd=MERGE_DIR)


def advance_branch(cfg, old, new):
    current = git("rev-parse", "--abbrev-ref", "HEAD")
    if current == cfg.branch:
        # Keep the main checkout's working tree in sync with the branch.
        git("merge", "--ff-only", "-q", new)
    else:
        git("update-ref", f"refs/heads/{cfg.branch}", new, old)
    if cfg.push:
        for delay in (0, 2, 4, 8, 16):
            if delay:
                import time
                time.sleep(delay)
            if run(["git", "push", cfg.remote, f"{new}:refs/heads/{cfg.branch}"], check=False).returncode == 0:
                break
        else:
            log(f"push of {cfg.branch} failed; continuing locally")


def merge(cfg, ticket, attempt_id, head, verdict, adir):
    """Returns a dict: merged (bool), reason, new_passes, target_complete, remaining."""
    adir = Path(adir)
    tip = git("rev-parse", cfg.branch)
    prepare(tip)
    pick = run(["git", "cherry-pick", "--no-commit", head], cwd=MERGE_DIR, check=False)
    if pick.returncode != 0:
        git("cherry-pick", "--abort", cwd=MERGE_DIR, check=False)
        git("reset", "--hard", "-q", tip, cwd=MERGE_DIR, check=False)
        return {"merged": False, "reason": "conflicts with newer work on the branch"}

    target_dir = WORK / "merge-target"
    build = cargo(["build", "-q", "-p", "kahvi-cli"], MERGE_DIR, target_dir)
    if build.returncode != 0:
        return {"merged": False, "reason": "does not build on top of the current branch:\n" + build.stderr[-2000:]}
    engine = target_dir / "debug" / "kahvi"
    baseline = MERGE_DIR / "conformance" / "expected-pass.txt"
    jobs = cfg.t262.get("jobs") or 0
    extra = ["--timeout", str(cfg.t262.get("timeout", 10))] + (["--jobs", str(jobs)] if jobs else [])

    log(f"{ticket.id}: full test262 run for merge")
    _, s = run_t262([], engine, adir, "full", baseline=baseline, write_baseline=baseline, extra=extra)
    if s is None or not s["canaries_ok"]:
        return {"merged": False, "reason": "full run invalid (canaries or runner failure)"}
    regressions = s["regressions"]
    if regressions:
        # One re-run of just the regressed files to rule out flakes.
        files = sorted({r.rsplit(":", 1)[0] for r in regressions})
        _, s2 = run_t262(files, engine, adir, "recheck", baseline=baseline, write_baseline=baseline, extra=extra)
        still = regressions if s2 is None else s2["regressions"]
        if still:
            git("checkout", "--", "conformance", cwd=MERGE_DIR, check=False)
            return {"merged": False, "reason": f"{len(still)} regressions in the full suite", "regressions": still[:50]}

    new_passes = len(s["new_passes"])
    git("add", "-A", cwd=MERGE_DIR)
    message = (
        f"{ticket.id}: {ticket.title} (+{new_passes} test262)\n\n"
        f"Attempt {attempt_id}, verifier verdict: {verdict}.\n"
        f"test262: {s['pass']}/{s['runs']} runs passing."
    )
    run(["git", "-c", "user.name=kahvi-merge-bot", "-c", "user.email=merge-bot@kahvi.invalid",
         "commit", "-q", "--no-verify", "-m", message], cwd=MERGE_DIR)
    new = git("rev-parse", "HEAD", cwd=MERGE_DIR)
    advance_branch(cfg, tip, new)
    (WORK / "last-full.jsonl").write_text((adir / "full.jsonl").read_text())
    write_json(WORK / "last-full-summary.json", s)

    results = read_results(adir / "full.jsonl")
    in_target = [r for r in results if any(r["id"].startswith(t) for t in ticket.target)]
    remaining = [f"{r['id']}  {r['reason'][:140]}" for r in in_target if r["status"] != "pass"]
    return {
        "merged": True,
        "commit": new,
        "new_passes": new_passes,
        "total_pass": s["pass"],
        "target_complete": not remaining,
        "remaining": remaining,
    }
