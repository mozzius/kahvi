# Ticket B-date-prototype-symbol-to-primitive: Date.prototype[Symbol.toPrimitive]

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

- `crates/kahvi-builtins/src/date/prototype_symbol_to_primitive.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/date.rs`
- `crates/kahvi-builtins/src/date/constructor.rs`

## Spec

Date.prototype [ %Symbol.toPrimitive% ] ( _hint_ )
  id: sec-date.prototype-%symbol.toprimitive%   type: built-in function

This method is called by ECMAScript language operators to convert a Date to a primitive value. The allowed values for _hint_ are *"default"*, *"number"*, and *"string"*. Dates are unique among built-in ECMAScript object in that they treat *"default"* as being equivalent to *"string"*, All other built-in ECMAScript objects treat *"default"* as being equivalent to *"number"*.
It performs the following steps when called:

1. Let _obj_ be the *this* value.
2. If _obj_ is not an Object, throw a *TypeError* exception.
3. If _hint_ is either *"string"* or *"default"*, then
   a. Let _tryFirst_ be ~string~.
4. Else if _hint_ is *"number"*, then
   a. Let _tryFirst_ be ~number~.
5. Else,
   a. Throw a *TypeError* exception.
6. Return ? OrdinaryToPrimitive(_obj_, _tryFirst_).

This property has the attributes { [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *true* }.
The value of the *"name"* property of this method is *"[Symbol.toPrimitive]"*.

--- referenced: OrdinaryToPrimitive ( _obj_: an Object, _hint_: ~string~ or ~number~, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-ordinarytoprimitive

1. If _hint_ is ~string~, then
   a. Let _methodNames_ be « *"toString"*, *"valueOf"* ».
2. Else,
   a. Let _methodNames_ be « *"valueOf"*, *"toString"* ».
3. For each element _name_ of _methodNames_, do
   a. Let _method_ be ? Get(_obj_, _name_).
   b. If IsCallable(_method_) is *true*, then
      i. Let _result_ be ? Call(_method_, _obj_).
      ii. If _result_ is not an Object, return _result_.
4. Throw a *TypeError* exception.


## Done when

- Every test262 run under built-ins/Date/prototype/Symbol.toPrimitive/ passes (check with `./t262 built-ins/Date/prototype/Symbol.toPrimitive/`). 6 pass on the main branch right now.

