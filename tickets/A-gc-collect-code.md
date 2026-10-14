+++
id = "A-gc-collect-code"
title = "GC: collect compiled code and the strings it pins"
kind = "infra"
tier = "architect"
scope = [
    "crates/kahvi-core/src/gc.rs",
    "crates/kahvi-core/src/agent.rs",
    "crates/kahvi-core/src/compiler/mod.rs",
    "crates/kahvi-core/src/compiler/bytecode.rs",
    "crates/kahvi-core/src/vm/eval.rs",
    "crates/kahvi-core/src/vm/dynamic_function.rs",
    "crates/kahvi-core/src/vm/mod.rs",
]
read = ["docs/GC.md"]
target = ["language/eval-code/", "built-ins/Function/", "built-ins/eval/"]
check = [
    "KAHVI_GC_STRESS=1 ./t262 language/eval-code/ built-ins/Function/ --engine $KAHVI_BIN --baseline conformance/expected-pass.txt --timeout 60",
]
priority = 0
+++
Compiled code is never freed: every string and BigInt the compiler interns
is pinned, and script records (including every `eval` and `new Function`)
are kept forever. A program that evaluates generated source in a loop
(`eval("x" + i)`, templates compiled at run time) grows without bound.

Trace code instead of pinning: when a `FunctionCode` is finished, record the
handles in its constants (strings in instructions, property keys, template
strings, BigInt literals, nested functions); trace them from the function
objects, frames, scripts and modules that own the code; drop the pinning
mode (keep pinning only the agent's common names and well-known symbols).
Script records of eval and dynamic functions should not keep their code
alive once nothing references it.

Done when: the targets and the stress-mode check do not regress, and
`for (let i = 0; i < 1e6; i++) eval("'s" + i + "'")` stays at a flat peak RSS.
