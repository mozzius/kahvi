+++
id = "C-proxy-has-property"
title = "Proxy [[HasProperty]]"
kind = "builtin"
tier = "haiku"
scope = ["crates/kahvi-core/src/object/proxy/has_property.rs"]
read = ["docs/API.md", "crates/kahvi-core/src/object/proxy/mod.rs", "crates/kahvi-core/src/object/internal.rs"]
spec = ["sec-proxy-object-internal-methods-and-internal-slots-hasproperty-p"]
target = ["built-ins/Proxy/has/"]
priority = 58
+++
Implement this Proxy internal method in its file, following the spec steps
already in the stub: keep each step as a comment directly above the code.

- `p.validate_non_revoked(agent)?` is ValidateNonRevokedProxy and gives
  (target, handler). Call the target's internal methods directly
  (`target.internal_get(agent, key, receiver)`); a property key becomes a
  value for the trap call with `agent.key_to_value(key)`.
- Property descriptors: `ops::from_property_descriptor`,
  `ops::to_property_descriptor`, `ops::complete_property_descriptor`; the
  invariant checks use IsCompatiblePropertyDescriptor, which is
  `kahvi_core::object::internal::validate_and_apply_property_descriptor`
  called with `None` as the object.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))`. No new Rust tests.
