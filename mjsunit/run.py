#!/usr/bin/env python3
"""mjsunit: V8's test/mjsunit suite run under the kahvi engine CLI.

V8's tests are fetched at the commit pinned in mjsunit/v8.lock into
mjsunit/.cache/ (a sparse, blob-filtered clone of test/mjsunit only). Each test
runs as V8's own runner runs it (test/mjsunit/testcfg.py): mjsunit.js first,
then any `// Files:` it names, then the test, as a script (or as a module for
.mjs). It passes when the engine exits 0, or, for a test with `--throws` in its
`// Flags:`, when it ends in an uncaught exception.

mjsunit is written for d8, so some tests cannot mean anything here. They are
skipped, each with its reason, never run and counted as failing:

- directories for wasm, the V8 sandbox, d8 itself and other V8-only tooling;
- tests mjsunit.status marks SKIP or FAIL for every V8 configuration;
- tests whose `// Flags:` change JS semantics (staged proposals, exposed
  internals, mock allocators, ...); flags that only tune V8's tiers or GC are
  ignored, as they do not change what a correct program prints;
- tests that call a V8 runtime function (`%Foo(...)`) other than the
  optimization hints below, or assert V8 tiering state (assertOptimized, ...);
- tests that use d8 host APIs (d8.*, Realm, Worker, load, quit, ...);
- anything listed with a reason in mjsunit/skip.txt.

The optimization hints (%PrepareFunctionForOptimization,
%OptimizeFunctionOnNextCall, %NeverOptimizeFunction, ...) only steer V8's
compilers and have no effect a program can observe, so the prepared copy of the
suite rewrites `%Name(` to `__v8_natives.Name(` and shim.js defines those as
functions that do nothing. %ArrayBufferDetach maps to the spec's
DetachArrayBuffer through $262, and `--expose-gc` defines gc() from $262.gc.

expected-pass.txt is the ratchet: a listed test that fails is a regression,
and the run exits 1. Other failures are reported, grouped by cause.

  ./mjs [PREFIX...]               run tests under these paths (all by default)
  ./mjs --engine PATH             use this kahvi binary (default: release build)
  ./mjs --gc-stress               collect at every safe point
  ./mjs --update-expected         rewrite expected-pass.txt from this run
  ./mjs --json FILE               also write per-test results as JSON
  ./mjs --skipped                 list skipped tests with their reasons
  ./mjs --markdown FILE           also write the README summary table
"""

import argparse
import fnmatch
import json
import os
import re
import resource
import shutil
import subprocess
import sys
import time
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
# Outside target/, which CI caches and prunes (rust-cache).
CACHE = HERE / ".cache"
EXPECTED = HERE / "expected-pass.txt"
SKIP_FILE = HERE / "skip.txt"
SHIM = HERE / "shim.js"
# Bump when rewrite_natives or rewrite_specifiers changes, to re-prepare the copy.
PREPARE_VERSION = 3

# Directories that test V8 itself rather than JavaScript.
SKIP_DIRS = {
    "wasm": "WebAssembly",
    "sandbox": "V8 sandbox",
    "d8": "d8 shell",
    "tools": "V8 tools (profiler, tick processor)",
    "async-hooks": "d8 async_hooks",
    "type-profile": "V8 type profiler",
    "lower_limits_mode": "V8 build mode",
    "shared-memory": "V8 shared structs and shared heap",
}

# Runtime functions that only steer V8's compilers. They return undefined and
# change nothing a program can observe; shim.js defines them as no-ops.
HINT_NATIVES = {
    "PrepareFunctionForOptimization", "OptimizeFunctionOnNextCall",
    "OptimizeMaglevOnNextCall", "NeverOptimizeFunction", "OptimizeOsr",
    "DeoptimizeFunction", "DeoptimizeNow", "EnsureFeedbackVectorForFunction",
    "ClearFunctionFeedback", "CompileBaseline", "BaselineOsr",
    "DisableOptimizationFinalization", "FinalizeOptimization",
    "WaitForBackgroundOptimization", "PretenureAllocationSite",
}
# Runtime functions with a spec meaning the CLI already exposes through $262.
SPEC_NATIVES = {"ArrayBufferDetach"}

# mjsunit.js helpers that read V8's optimization status.
TIERING_HELPERS = re.compile(
    r"\b(assertOptimized|assertUnoptimized|assertMaglevved|assertNotMaglevved|"
    r"isNeverOptimize\w*|isLazy|isInterpreted|isBaseline|isUnoptimized|"
    r"isOptimized|willBeMaglevved|willBeTurbofanned|isMaglevved|isTurboFanned|"
    r"topFrameIs\w+|V8OptimizationStatus)\b")

# d8's own globals, which a JS engine does not define.
D8_API = re.compile(
    r"\b(d8|Realm)\s*\.|\bnew\s+Worker\b|(?<![.\w$])(load|quit|readline|readbuffer|"
    r"read|writeFile|version|setTimeout|testRunner)\s*\(")

# Flags that change what a correct program does. Anything else in `// Flags:`
# tunes V8's tiers, GC or logging and is ignored.
SEMANTIC_FLAGS = [
    re.compile(p) for p in (
        r"--(no-?)?harmony.*", r"--(no-?)?js-.*", r"--es-staging", r"--(no-?)?experimental.*",
        r"--expose-(?!gc$).*", r"--(no-?)?sandbox.*", r"--(no-?)?wasm.*", r"--liftoff",
        r"--mock-arraybuffer-allocator", r"--multi-mapped-mock-allocator",
        r"--shared-string-table", r"--shared-heap", r"--(no-?)?enable-inspector",
        r"--use-strict", r"--icu-timezone-data", r"--memory-corruption.*",
        r"--(no-?)?enable-experimental-regexp.*", r"--default-to-experimental-regexp.*",
        r"--disallow-.*", r"--allow-(?!natives-syntax$).*", r"--bundle", r"--fuzzing",
        r"--(no-?)?jit-fuzzing", r"--stress-runs.*", r"--no-fail",
    )
]

FLAGS_LINE = re.compile(r"^//\s*Flags:(.*)$", re.M)
FILES_LINE = re.compile(r"//\s+Files:(.*)")
ENV_LINE = re.compile(r"//\s+Environment Variables:(.*)")
NO_HARNESS = re.compile(r"^// NO HARNESS$", re.M)
NATIVE_CALL = re.compile(r"%_?([A-Z][A-Za-z0-9_]*)\s*\(")  # %Name( and the %_Name( intrinsic form


def lock():
    fields = dict(line.split("=", 1) for line in (HERE / "v8.lock").read_text().split() if "=" in line)
    return fields["repo"], fields["commit"]


def fetch():
    """Sparse, blob-filtered clone of V8's test/mjsunit at the pinned commit."""
    repo, commit = lock()
    src = CACHE / "v8"
    if (src / ".git").is_dir():
        have = subprocess.run(["git", "rev-parse", "HEAD"], cwd=src, capture_output=True, text=True).stdout.strip()
        if have == commit and (src / "test" / "mjsunit" / "mjsunit.js").is_file():
            return src, commit
    else:
        src.mkdir(parents=True, exist_ok=True)
        git = lambda *a: subprocess.run(["git", *a], cwd=src, check=True, stdout=subprocess.DEVNULL)
        git("init", "-q")
        git("remote", "add", "origin", repo)
        git("sparse-checkout", "set", "test/mjsunit")
    print(f"mjs: fetching V8 test/mjsunit at {commit[:12]}", flush=True)
    git = lambda *a: subprocess.run(["git", *a], cwd=src, check=True, stdout=subprocess.DEVNULL)
    git("remote", "set-url", "origin", repo)
    git("fetch", "-q", "--depth", "1", "--filter=blob:none", "origin", commit)
    git("checkout", "-q", "-f", "--detach", commit)
    return src, commit


def rewrite_natives(text):
    """Turn `%Name(` into `__v8_natives.Name(` for the natives shim.js defines.

    Only those names are rewritten: tests calling any other runtime function
    are skipped, and no test takes the modulo of a call to a function named
    like an optimization hint, so `%PrepareFunctionForOptimization(` is never
    the modulo operator.
    """
    return NATIVE_CALL.sub(
        lambda m: f"__v8_natives.{m.group(1)}(" if m.group(1) in HINT_NATIVES | SPEC_NATIVES else m.group(0), text)


IMPORT_SPEC = re.compile(r"""((?:\bfrom|\bimport)\s*\(?\s*)(['"])(?![./])([^'"\n]+\.m?js)\2""")


def rewrite_specifiers(text):
    """d8 resolves `import 'x.mjs'` against the importing file; kahvi's CLI does
    that for `./x.mjs`. Spell such specifiers the way the CLI resolves them."""
    return IMPORT_SPEC.sub(lambda m: f"{m.group(1)}{m.group(2)}./{m.group(3)}{m.group(2)}", text)


def prepare(src, commit):
    """Copy test/mjsunit to mjsunit/.cache/prepared-COMMIT with natives rewritten."""
    dst = CACHE / f"prepared-{commit[:12]}-v{PREPARE_VERSION}"
    if (dst / ".done").exists():
        return dst
    if dst.exists():
        shutil.rmtree(dst)
    base = src / "test" / "mjsunit"
    if not (base / "mjsunit.js").is_file():
        sys.exit(f"mjs: {base} has no mjsunit.js; delete {CACHE} and run again")
    dst.mkdir(parents=True)
    for p in base.rglob("*"):
        if not p.is_file():
            continue
        q = dst / p.relative_to(base)
        q.parent.mkdir(parents=True, exist_ok=True)
        if p.suffix in (".js", ".mjs") and p.name != "mjsunit.js":
            text = rewrite_natives(p.read_text(errors="surrogateescape"))
            if p.suffix == ".mjs" or "import" in text:
                text = rewrite_specifiers(text)
            q.write_text(text, errors="surrogateescape")
        else:
            shutil.copyfile(p, q)
    (dst / ".done").write_text(commit + "\n")
    return dst


def status_always(src):
    """Globs mjsunit.status marks SKIP or FAIL (without PASS) in its ALWAYS section."""
    text = (src / "test" / "mjsunit" / "mjsunit.status").read_text()
    names = defaultdict(lambda: None)
    class Names(dict):  # outcome names (PASS, FAIL, SKIP, ...) evaluate to themselves
        def __missing__(self, key):
            return key
    sections = eval(text, {"__builtins__": {}}, Names())  # V8's status files are Python literals
    out = []
    for cond, entries in sections:
        if cond != "ALWAYS":
            continue
        for glob, outcomes in entries.items():
            outcomes = outcomes if isinstance(outcomes, list) else [outcomes]
            if "SKIP" in outcomes:
                out.append((glob, "mjsunit.status: SKIP"))
            elif "FAIL" in outcomes and "PASS" not in outcomes:
                out.append((glob, "mjsunit.status: FAIL in V8 too"))
    return out


def manual_skips():
    skips = {}
    if SKIP_FILE.exists():
        for line in SKIP_FILE.read_text().splitlines():
            line = line.strip()
            if line and not line.startswith("#"):
                name, _, why = line.partition(" ")
                skips[name] = why.strip().lstrip("#").strip() or "skip.txt"
    return skips


def collect(prepared, src, prefixes):
    """Every test with its run spec, or the reason it is skipped."""
    status = status_always(src)
    manual = manual_skips()
    original = src / "test" / "mjsunit"
    tests = []
    for p in sorted(prepared.rglob("*")):
        if p.suffix not in (".js", ".mjs") or not p.is_file():
            continue
        name = p.relative_to(prepared).as_posix()
        if name in ("mjsunit.js", "mjsunit_numfuzz.js"):
            continue
        if prefixes and not any(name.startswith(x) for x in prefixes):
            continue
        tests.append((name, p, classify(name, p, original / name, status, manual)))
    return tests


def classify(name, path, original, status, manual):
    stem = name.rsplit(".", 1)[0]
    if name in manual:
        return {"skip": manual[name]}
    top = name.split("/")[0] if "/" in name else None
    if top in SKIP_DIRS:
        return {"skip": f"directory: {SKIP_DIRS[top]}"}
    if name.startswith("regress/wasm/"):
        return {"skip": "directory: WebAssembly"}
    for glob, why in status:
        if fnmatch.fnmatchcase(stem, glob):
            return {"skip": why}
    source = original.read_text(errors="surrogateescape")
    flags = [f for line in FLAGS_LINE.findall(source) for f in line.split()]
    for f in flags:
        flag = f.split("=", 1)[0]
        if any(rx.fullmatch(flag) for rx in SEMANTIC_FLAGS):
            return {"skip": f"flag {flag}"}
    # The `// Files:` helpers run with the test, so they count as its code.
    code = source
    for f in (f for line in FILES_LINE.findall(source) for f in line.split()):
        helper = original.parents[len(Path(name).parts) - 1].parent.parent / f
        if helper.is_file():
            code += "\n" + helper.read_text(errors="surrogateescape")
    natives = set(NATIVE_CALL.findall(code)) - HINT_NATIVES - SPEC_NATIVES
    if natives:
        return {"skip": f"V8 internal %{sorted(natives)[0]}"}
    if TIERING_HELPERS.search(code):
        return {"skip": "asserts V8 tiering state"}
    if re.search(r"\bWebAssembly\b|wasm-module-builder", code):
        return {"skip": "uses WebAssembly"}
    m = D8_API.search(re.sub(r"//.*", "", code))
    if m:
        return {"skip": f"d8 host API: {m.group(0).strip()}"}
    files = [f for line in FILES_LINE.findall(source) for f in line.split()]
    extra = []
    for f in files:
        if not f.startswith("test/mjsunit/"):
            return {"skip": f"needs {f} from outside test/mjsunit"}
        extra.append(f[len("test/mjsunit/"):])
    env = dict(pair.split("=", 1) for line in ENV_LINE.findall(source) for pair in line.split())
    return {
        "harness": not NO_HARNESS.search(source),
        "files": extra,
        "module": name.endswith(".mjs"),
        "expose_gc": "--expose-gc" in flags,
        "throws": "--throws" in flags,
        "env": env,
    }


# Address-space cap per test, so a runaway allocation fails that test instead of
# getting the whole run killed by the OOM killer. This caps virtual memory, which
# runs well above resident memory: es6/big-weakmap.js peaks at about 2 GB RSS
# and needs more than 3 GB of address space. Runaways reach far past 4 GB.
MEMORY_LIMIT = 4 << 30


def limit_memory():
    resource.setrlimit(resource.RLIMIT_AS, (MEMORY_LIMIT, MEMORY_LIMIT))


def run_test(engine, prepared, name, path, spec, gc_stress, timeout):
    cmd = [engine, "--prelude", str(SHIM)]
    if spec["expose_gc"]:
        cmd += ["--prelude", str(HERE / "expose-gc.js")]
    if spec["harness"]:
        cmd += ["--prelude", str(prepared / "mjsunit.js")]
    for f in spec["files"]:
        cmd += ["--prelude", str(prepared / f)]
    if spec["module"]:
        cmd.append("--module")
    cmd.append(str(path))
    env = {"PATH": os.environ.get("PATH", ""), "TZ": "UTC", **spec["env"]}
    if gc_stress:
        env["KAHVI_GC_STRESS"] = "1"
    start = time.monotonic()
    try:
        p = subprocess.run(cmd, cwd=path.parent, env=env, capture_output=True, timeout=timeout,
                           preexec_fn=limit_memory)
        err, status = p.stderr.decode(errors="replace"), p.returncode
    except subprocess.TimeoutExpired:
        err, status = f"timeout after {timeout}s", None
    secs = round(time.monotonic() - start, 2)
    uncaught = next((l for l in err.splitlines() if l.startswith("Uncaught")), None)
    if spec["throws"]:
        ok = status == 1 and uncaught is not None
    else:
        ok = status == 0
    r = {"name": name, "pass": ok, "secs": secs}
    if not ok:
        if status is None:
            reason = err
        elif status == 0:
            reason = "exited 0 but the test expects an uncaught exception (--throws)"
        else:
            first = uncaught or (err.strip().splitlines() or [f"exit {status}"])[0]
            reason = f"crash (exit {status}): {first}" if status >= 101 or status < 0 else first
        r.update(status=status, reason=reason[:300])
    return r


def cause(reason):
    """Group failures: assertion text varies per test, the error kind does not."""
    if reason.startswith("Uncaught MjsUnitAssertionError: Failure (Error message)"):
        return "error message text differs from V8's (not a spec requirement)"
    if reason.startswith("Uncaught MjsUnitAssertionError"):
        return "Uncaught MjsUnitAssertionError (an assertion failed)"
    if "not implemented:" in reason:
        return reason
    if reason.startswith("timeout"):
        return "timeout"
    if reason.startswith("crash"):
        return re.sub(r"(panicked at [^:]+:\d+).*", r"\1", reason)[:160]
    return reason[:120]


def markdown_summary(commit, results, passed, skipped):
    """The README table: how many of the run tests pass, and how many are skipped."""
    rows = ["| mjsunit | |", "|---|--:|",
            f"| **Passing** | **{len(passed)} / {len(results)}** |",
            f"| Skipped | {len(skipped)} |"]
    note = (f"V8's mjsunit tests at [{commit[:12]}](https://chromium.googlesource.com/v8/v8/+/{commit}/test/mjsunit), "
            "run under the `kahvi` CLI; tests that need d8 or V8 internals are skipped with a reason "
            "([mjsunit/](mjsunit/run.py), `./mjs`).")
    return note + "\n\n" + "\n".join(rows) + "\n"


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--engine", help="kahvi binary (default: build target/release/kahvi)")
    ap.add_argument("--gc-stress", action="store_true", help="collect at every safe point")
    ap.add_argument("--timeout", type=int, default=60, help="seconds per test (default 60)")
    ap.add_argument("-j", "--jobs", type=int, default=os.cpu_count() or 4)
    ap.add_argument("--update-expected", action="store_true", help="rewrite expected-pass.txt")
    ap.add_argument("--json", metavar="FILE", help="write per-test results to FILE")
    ap.add_argument("--markdown", metavar="FILE", help="write the README summary table to FILE")
    ap.add_argument("--skipped", action="store_true", help="list skipped tests and their reasons")
    ap.add_argument("prefixes", nargs="*")
    args = ap.parse_args()

    src, commit = fetch()
    prepared = prepare(src, commit)
    tests = collect(prepared, src, args.prefixes)
    skipped = [(n, s["skip"]) for n, _, s in tests if "skip" in s]
    runnable = [(n, p, s) for n, p, s in tests if "skip" not in s]
    if args.skipped:
        for n, why in skipped:
            print(f"{n}  {why}")
        return

    engine = args.engine
    if not engine:
        subprocess.run(["cargo", "build", "-q", "--release", "-p", "kahvi-cli"], cwd=ROOT, check=True)
        engine = str(ROOT / "target" / "release" / "kahvi")
    engine = str(Path(engine).resolve())

    expected = set(EXPECTED.read_text().split()) if EXPECTED.exists() else set()
    print(f"mjs: V8 {commit[:12]}, {len(runnable)} tests to run, {len(skipped)} skipped", flush=True)
    results = []
    with ThreadPoolExecutor(args.jobs) as pool:
        futs = [pool.submit(run_test, engine, prepared, n, p, s, args.gc_stress, args.timeout) for n, p, s in runnable]
        for i, f in enumerate(futs, 1):
            r = f.result()
            results.append(r)
            if not r["pass"] and r["name"] in expected:
                print(f"REGRESSION {r['name']}: {r['reason']}", flush=True)
            if i % 500 == 0:
                print(f"  {i}/{len(futs)}", flush=True)

    passed = sorted(r["name"] for r in results if r["pass"])
    failed = [r for r in results if not r["pass"]]
    groups = defaultdict(list)
    for r in failed:
        groups[cause(r["reason"])].append(r["name"])
    if groups:
        print("\nfailures by cause (top 40):")
        for reason, names in sorted(groups.items(), key=lambda kv: (-len(kv[1]), kv[0]))[:40]:
            print(f"  {len(names):5}  {reason}  [{', '.join(names[:3])}{', ...' if len(names) > 3 else ''}]")
    print("\nskipped by reason:")
    for why, n in Counter(re.sub(r"(%|: ).*", r"\1*", w) for _, w in skipped).most_common():
        print(f"  {n:5}  {why}")
    regressions = sorted(r["name"] for r in failed if r["name"] in expected)
    new_passes = sorted(set(passed) - expected)
    print(f"\n{len(passed)}/{len(results)} passed ({len(skipped)} skipped)")
    if new_passes:
        print(f"newly passing (not in expected-pass.txt): {len(new_passes)}")
    if regressions:
        print(f"REGRESSIONS (in expected-pass.txt): {', '.join(regressions)}")

    if args.json:
        Path(args.json).write_text(json.dumps({"engine": engine, "v8": commit, "gc_stress": args.gc_stress,
                                               "results": results, "skipped": dict(skipped)}, indent=1) + "\n")
    if args.markdown and not args.prefixes:
        Path(args.markdown).write_text(markdown_summary(commit, results, passed, skipped))
    if args.update_expected:
        if args.prefixes:
            sys.exit("mjs: --update-expected needs a run of every test")
        EXPECTED.write_text("".join(n + "\n" for n in passed))
        print(f"wrote {EXPECTED.relative_to(ROOT)}")
    elif regressions:
        sys.exit(1)


if __name__ == "__main__":
    main()
