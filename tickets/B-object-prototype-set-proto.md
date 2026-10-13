+++
id = "B-object-prototype-set-proto"
title = "set Object.prototype.__proto__"
kind = "builtin"
tier = "haiku"
scope = ["crates/kahvi-builtins/src/object/prototype_set_proto.rs"]
read = ["docs/API.md", "crates/kahvi-builtins/src/support/mod.rs", "crates/kahvi-builtins/src/array/prototype_map.rs", "crates/kahvi-builtins/src/object/define_property.rs"]
spec = ["sec-set-object.prototype.__proto__"]
target = ["built-ins/Object/prototype/__proto__/"]
priority = 10
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
