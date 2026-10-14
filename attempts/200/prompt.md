# Ticket B-reg-exp-prototype-compile: RegExp.prototype.compile

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

## Files you may change

- `crates/kahvi-builtins/src/reg_exp/prototype_compile.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

RegExp.prototype.compile ( _pattern_, _flags_ )
  id: sec-regexp.prototype.compile   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. Perform ? RequireInternalSlot(_obj_, [[RegExpMatcher]]).
3. If _pattern_ is an Object and _pattern_ has a [[RegExpMatcher]] internal slot, then
   a. If _flags_ is not *undefined*, throw a *TypeError* exception.
   b. Set _flags_ to _pattern_.[[OriginalFlags]].
   c. Set _pattern_ to _pattern_.[[OriginalSource]].
4. Return ? RegExpInitialize(_obj_, _pattern_, _flags_).

NOTE:
This method completely reinitializes the *this* value RegExp with a new pattern and flags. An implementation may interpret use of this method as an assertion that the resulting RegExp object will be used multiple times and hence is a candidate for extra optimization.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.


## Done when

- Every test262 run under annexB/built-ins/RegExp/prototype/compile/ passes (check with `./t262 annexB/built-ins/RegExp/prototype/compile/`). 6 pass on the main branch right now.

