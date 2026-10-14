+++
id = "B-uint8-array-from-base64"
title = "Uint8Array.fromBase64"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/uint8_array/from_base64.rs"]
read = ["docs/API.md", "crates/kahvi-builtins/src/support/mod.rs", "crates/kahvi-builtins/src/array/prototype_map.rs", "crates/kahvi-builtins/src/object/define_property.rs", "crates/kahvi-builtins/src/support/typed_array.rs"]
spec = ["sec-uint8array.frombase64"]
target = ["built-ins/Uint8Array/fromBase64/"]
priority = 44
+++
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

Write FromBase64 (sec-frombase64) and the option helpers (GetUint8ArrayBytes is not needed here) as `pub(crate)` functions in this file: Uint8Array.prototype.setFromBase64 calls them.
