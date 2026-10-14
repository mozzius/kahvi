+++
id = "A-gc-collect-in-callbacks"
title = "GC: collect inside callbacks of built-ins and iterator steps"
kind = "infra"
tier = "architect"
scope = [
    "crates/kahvi-builtins/src/array/prototype_for_each.rs",
    "crates/kahvi-builtins/src/array/prototype_map.rs",
    "crates/kahvi-builtins/src/array/prototype_filter.rs",
    "crates/kahvi-builtins/src/array/prototype_some.rs",
    "crates/kahvi-builtins/src/array/prototype_every.rs",
    "crates/kahvi-builtins/src/array/prototype_find.rs",
    "crates/kahvi-builtins/src/array/prototype_find_index.rs",
    "crates/kahvi-builtins/src/array/prototype_find_last.rs",
    "crates/kahvi-builtins/src/array/prototype_find_last_index.rs",
    "crates/kahvi-builtins/src/array/prototype_reduce.rs",
    "crates/kahvi-builtins/src/array/prototype_reduce_right.rs",
    "crates/kahvi-builtins/src/array/prototype_flat_map.rs",
    "crates/kahvi-builtins/src/array/prototype_sort.rs",
    "crates/kahvi-builtins/src/array/prototype_to_sorted.rs",
    "crates/kahvi-builtins/src/array/from.rs",
    "crates/kahvi-builtins/src/map/prototype_for_each.rs",
    "crates/kahvi-builtins/src/set/prototype_for_each.rs",
    "crates/kahvi-builtins/src/string/prototype_replace.rs",
    "crates/kahvi-builtins/src/string/prototype_replace_all.rs",
    "crates/kahvi-builtins/src/json/parse.rs",
    "crates/kahvi-builtins/src/json/stringify.rs",
    "crates/kahvi-core/src/vm/mod.rs",
    "crates/kahvi-core/src/ops/iteration.rs",
]
read = ["docs/GC.md"]
target = [
    "built-ins/Array/prototype/",
    "built-ins/Map/prototype/forEach/",
    "built-ins/Set/prototype/forEach/",
    "built-ins/String/prototype/replace/",
    "built-ins/JSON/",
    "language/statements/for-of/",
]
check = [
    "KAHVI_GC_STRESS=1 ./t262 built-ins/Array/prototype/ built-ins/JSON/ language/statements/for-of/ --engine $KAHVI_BIN --baseline conformance/expected-pass.txt --timeout 60",
]
priority = 0
+++
Rust code that calls into JavaScript holds the collector (`gc_holds`), so
garbage made inside a `forEach`/`map`/`sort`/`replace`/`JSON.parse` reviver
callback is not collected until the built-in returns, and a generator body
driven by `for-of` (whose `next()` the interpreter calls through
`ops::call`) cannot collect while it runs. A long callback loop can grow the
heap without bound.

For each built-in in scope: keep every handle it uses after a callback
rooted (`agent.push_root` / `agent.pop_roots`, or values already reachable
from rooted objects) and call with `ops::call_rooted`. The interpreter's
iterator steps (IteratorNext, IteratorStep, spread, destructuring) can call
`next()` rooted: the iterator record is on the frame's iterator stack.

Done when: the targets do not regress, the stress-mode check passes, and a
callback that allocates in a loop (e.g. `new Array(1e6).fill(0).forEach(()
=> ({a: [1, 2]}))`) stays at a flat peak RSS.
