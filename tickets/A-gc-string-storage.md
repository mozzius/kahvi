+++
id = "A-gc-string-storage"
title = "Store each interned string once"
kind = "infra"
tier = "architect"
scope = [
    "crates/kahvi-core/src/agent.rs",
    "crates/kahvi-core/src/hash.rs",
    "crates/kahvi-core/src/gc.rs",
]
read = ["docs/GC.md"]
target = ["built-ins/String/"]
check = [
    "KAHVI_GC_STRESS=1 ./t262 built-ins/String/prototype/ --engine $KAHVI_BIN --baseline conformance/expected-pass.txt --timeout 60",
]
priority = 0
+++
The intern table (`Agent::string_index`) keys on a second copy of every
string's code units, so string memory is doubled. Index the arena instead:
a table of handles hashed by content, probing with the arena's slice (no
second allocation), with the collector's sweep removing dead handles from it.

Done when: no regressions, and a program holding many large strings (e.g.
1e5 strings of 1 KiB) uses about half the string memory it does now.
