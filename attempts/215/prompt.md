# Ticket B-symbol-prototype-symbol-to-primitive: Symbol.prototype[Symbol.toPrimitive]

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

- `crates/kahvi-builtins/src/symbol/prototype_symbol_to_primitive.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Symbol.prototype [ %Symbol.toPrimitive% ] ( _hint_ )
  id: sec-symbol.prototype-%symbol.toprimitive%   type: built-in function

This method is called by ECMAScript language operators to convert a Symbol object to a primitive value.
It performs the following steps when called:

1. Return ? ThisSymbolValue(*this* value).

NOTE:
The argument is ignored.
This property has the attributes { [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *true* }.
The value of the *"name"* property of this method is *"[Symbol.toPrimitive]"*.

--- referenced: ThisSymbolValue ( _arg_: an ECMAScript language value, ): either a normal completion containing a Symbol or a throw completion
  id: sec-thissymbolvalue

1. If _arg_ is a Symbol, return _arg_.
2. If _arg_ is an Object and _arg_ has a [[SymbolData]] internal slot, then
   a. Let _symbol_ be _arg_.[[SymbolData]].
   b. Assert: _symbol_ is a Symbol.
   c. Return _symbol_.
3. Throw a *TypeError* exception.


## Done when

- Every test262 run under built-ins/Symbol/prototype/Symbol.toPrimitive/ passes (check with `./t262 built-ins/Symbol/prototype/Symbol.toPrimitive/`). 8 pass on the main branch right now.

