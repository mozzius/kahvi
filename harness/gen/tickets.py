#!/usr/bin/env python3
"""Writes one ticket per unimplemented built-in stub (tickets/B-*.md).

    python3 harness/gen/tickets.py [--counts FILE]

Only groups whose functions can be implemented in a single file with the
existing helpers get tickets; groups that first need new internal-slot types
or engine support (Map, Promise, RegExp, typed arrays, ...) are listed in
ARCHITECT_FIRST and wait for the architect.

--counts is a JSON object mapping not-implemented names (as the runner
reports them, e.g. "Array.prototype.concat") to how many test262 runs they
block; it sets each ticket's priority.
"""

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "harness"))
import importlib.util  # noqa: E402

import spec  # noqa: E402

# harness/gen/builtins.py (its module name would clash with Python's builtins).
_spec = importlib.util.spec_from_file_location("gen_builtins", ROOT / "harness" / "gen" / "builtins.py")
gen = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(gen)

TEST = ROOT / "vendor" / "test262" / "test"
TICKETS = ROOT / "tickets"
SRC = gen.OUT

# Groups whose built-ins workers can implement now.
WORKER_GROUPS = {
    "object", "array", "array_iterator_prototype", "string", "number", "boolean", "symbol", "math", "json",
    "reflect", "global", "error", "aggregate_error", "suppressed_error", "function", "promise",
    "map", "set", "weak_map", "weak_set", "map_iterator_prototype", "set_iterator_prototype",
    "string_iterator_prototype",
}
# Individual functions that need the architect first, with the reason.
ARCHITECT_FIRST = {
    "Function": "CreateDynamicFunction needs parser entry points for parameters and bodies",
    "String.prototype.normalize": "needs Unicode normalization tables",
    "Promise.try": "needs the Promise.try proposal semantics reviewed first",
}

# Installed as aliases of another built-in (see ALIASES in table.rs): the stub
# is never reachable, so no ticket.
ALIASED = {
    "Array.prototype[Symbol.iterator]", "Map.prototype[Symbol.iterator]", "Set.prototype[Symbol.iterator]",
    "Set.prototype.keys", "String.prototype.trimLeft", "String.prototype.trimRight",
    "Date.prototype.toGMTString", "Number.parseFloat", "Number.parseInt",
    "%TypedArray%.prototype[Symbol.iterator]",
}

STUB_MARKER = "let _ = (this, args, new_target);"

READ = [
    "docs/API.md",
    "crates/kahvi-builtins/src/support/mod.rs",
    "crates/kahvi-builtins/src/array/prototype_map.rs",
    "crates/kahvi-builtins/src/object/define_property.rs",
]

# Extra reading for some groups.
EXTRA_READ = {
    "promise": ["crates/kahvi-core/src/ops/promise.rs", "crates/kahvi-builtins/src/promise/prototype_then.rs"],
}
for _g in ("map", "set", "weak_map", "weak_set", "map_iterator_prototype", "set_iterator_prototype"):
    EXTRA_READ[_g] = [
        "crates/kahvi-builtins/src/support/collections.rs",
        "crates/kahvi-builtins/src/map/prototype_set.rs",
        "crates/kahvi-builtins/src/map_iterator_prototype/next.rs",
    ]
EXTRA_READ["string_iterator_prototype"] = ["crates/kahvi-builtins/src/support/string_iterator.rs"]
EXTRA_READ["string"] = ["crates/kahvi-builtins/src/support/string_iterator.rs"]

NOTES = """\
Implement this built-in in its file, following the spec steps already in
the stub: keep each step as a comment directly above the code for it.

- Abstract operations are in `kahvi_core::ops` (see docs/API.md); use them
  rather than re-implementing. If a helper you need is missing and other
  built-ins would need it too, write it in your file and say so in your
  final report; do not edit other files.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing. Never return
  a made-up value.
- No new Rust tests: the test262 target is the measure.
"""


def test_dir(fn):
    """The test262 directory for a built-in, or None."""
    owner = fn.owner.strip("%")
    key = f"Symbol.{fn.sym}" if fn.sym else fn.key
    if fn.constructor:
        if fn.key in gen.NATIVE_ERRORS:
            candidates = [f"built-ins/NativeErrors/{fn.key}/"]
        else:
            candidates = [f"built-ins/{fn.key}/"]
    elif owner == "GlobalObject":
        candidates = [f"built-ins/{key}/"]
    else:
        parts = owner.split(".")
        base = {
            "GeneratorPrototype": "GeneratorPrototype",
            "ArrayIteratorPrototype": "ArrayIteratorPrototype",
        }.get(parts[0], parts[0])
        path = "/".join([base] + parts[1:] + ([key] if key else []))
        candidates = [f"built-ins/{path}/"]
    for c in candidates + [f"annexB/{c}" for c in candidates]:
        if (TEST / c).is_dir():
            return c
    return None


def ticket_id(fn):
    return "B-" + re.sub(r"[^a-z0-9]+", "-", f"{fn.group}-{fn.file}".lower()).strip("-")


def main(argv):
    counts = {}
    if "--counts" in argv:
        counts = json.loads(Path(argv[argv.index("--counts") + 1]).read_text())
    fns = gen.collect(spec.load())
    written, skipped = 0, []
    for fn in fns:
        if fn.group not in WORKER_GROUPS:
            continue
        stub = SRC / fn.group / f"{fn.file}.rs"
        if not stub.exists() or STUB_MARKER not in stub.read_text():
            continue  # already implemented
        if fn.display in ALIASED:
            stale = TICKETS / f"{ticket_id(fn)}.md"
            if stale.exists():
                stale.unlink()
            continue
        if fn.display in ARCHITECT_FIRST:
            skipped.append(f"{fn.display}: {ARCHITECT_FIRST[fn.display]}")
            continue
        target = test_dir(fn)
        if not target:
            skipped.append(f"{fn.display}: no test262 directory found")
            continue
        tid = ticket_id(fn)
        scope = f"crates/kahvi-builtins/src/{fn.group}/{fn.file}.rs"
        priority = counts.get(fn.display, 0)
        meta = [
            f'id = "{tid}"',
            f"title = {json.dumps(fn.display)}",
            'kind = "builtin"',
            'tier = "haiku"',
            f"scope = [{json.dumps(scope)}]",
            "read = [" + ", ".join(json.dumps(r) for r in READ + EXTRA_READ.get(fn.group, [])) + "]",
            f"spec = [{json.dumps(fn.id)}]",
            f"target = [{json.dumps(target)}]",
            f"priority = {priority}",
        ]
        (TICKETS / f"{tid}.md").write_text("+++\n" + "\n".join(meta) + "\n+++\n" + NOTES)
        written += 1
    print(f"{written} tickets written")
    for s in skipped:
        print(f"  skipped {s}")


if __name__ == "__main__":
    main(sys.argv[1:])
