# Ticket B-iterator-prototype-set-constructor: set Iterator.prototype.constructor

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

- `crates/kahvi-builtins/src/iterator/prototype_set_constructor.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

set Iterator.prototype.constructor
  id: sec-set-iterator.prototype.constructor   type: built-in function

The value of the [[Setter]] attribute is a built-in function that takes an argument _v_. It performs the following steps when called:

1. Perform ? SetterThatIgnoresPrototypeProperties(*this* value, %Iterator.prototype%, *"constructor"*, _v_).
2. Return *undefined*.

--- referenced: SetterThatIgnoresPrototypeProperties ( _thisValue_: an ECMAScript language value, _home_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-SetterThatIgnoresPrototypeProperties

1. If _thisValue_ is not an Object, then
   a. Throw a *TypeError* exception.
2. If SameValue(_thisValue_, _home_) is *true*, then
   a. NOTE: Throwing here emulates assignment to a non-writable data property on the _home_ object in strict mode code.
   b. Throw a *TypeError* exception.
3. Let _propertyDesc_ be ? _thisValue_.[[GetOwnProperty]](_propertyKey_).
4. If _propertyDesc_ is *undefined*, then
   a. Perform ? CreateDataPropertyOrThrow(_thisValue_, _propertyKey_, _value_).
5. Else,
   a. Perform ? Set(_thisValue_, _propertyKey_, _value_, *true*).
6. Return ~unused~.


## Done when

- Every test262 run under built-ins/Iterator/prototype/constructor/ passes (check with `./t262 built-ins/Iterator/prototype/constructor/`). 0 pass on the main branch right now.

## Previous attempts

- attempt 483: rejected by gates: no new test262 passes in target
- attempt 490: worker reported BLOCKED but changed files: `table.rs` link loop (lines 901-902) overwrites the Iterator.prototype.constructor accessor with a data property; it must skip the Iterator constructor link so the accessor from `generated.rs` survives.
- attempt 490: rejected by gates: no new test262 passes in target
- attempt 490: escalated from haiku to sonnet

