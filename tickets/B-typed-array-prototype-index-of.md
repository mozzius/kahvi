+++
id = "B-typed-array-prototype-index-of"
title = "TypedArray.prototype.indexOf"
kind = "builtin"
tier = "haiku"
scope = ["crates/kahvi-builtins/src/typed_array/prototype_index_of.rs"]
read = ["docs/API.md", "crates/kahvi-builtins/src/support/mod.rs", "crates/kahvi-builtins/src/array/prototype_map.rs", "crates/kahvi-builtins/src/object/define_property.rs", "crates/kahvi-builtins/src/support/typed_array.rs", "crates/kahvi-core/src/object/typed_array.rs", "crates/kahvi-builtins/src/typed_array/prototype_get_length.rs"]
spec = ["sec-%typedarray%.prototype.indexof"]
target = ["built-ins/TypedArray/prototype/indexOf/"]
priority = 76
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
