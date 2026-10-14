# Ticket B-symbol-for: Symbol.for

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

- `crates/kahvi-builtins/src/symbol/for.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Symbol.for ( _key_ )
  id: sec-symbol.for   type: built-in function

This function performs the following steps when called:

1. Let _stringKey_ be ? ToString(_key_).
2. Let _agentRecord_ be the Agent Record of the surrounding agent.
3. Let _globalSymbolRegistry_ be _agentRecord_.[[GlobalSymbolRegistry]].
4. For each element _element_ of _globalSymbolRegistry_, do
   a. If _element_.[[Key]] is _stringKey_, return _element_.[[Symbol]].
5. Assert: _globalSymbolRegistry_ does not currently contain an entry for _stringKey_.
6. Let _newSymbol_ be a new Symbol whose [[Description]] is _stringKey_.
7. Append the GlobalSymbolRegistry Record { [[Key]]: _stringKey_, [[Symbol]]: _newSymbol_ } to _globalSymbolRegistry_.
8. Return _newSymbol_.

--- referenced: ToString ( _arg_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-tostring

description: It converts _arg_ to a value of type String.

1. If _arg_ is a String, return _arg_.
2. If _arg_ is a Symbol, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *"undefined"*.
4. If _arg_ is *null*, return *"null"*.
5. If _arg_ is *true*, return *"true"*.
6. If _arg_ is *false*, return *"false"*.
7. If _arg_ is a Number, return Number::toString(_arg_, 10).
8. If _arg_ is a BigInt, return BigInt::toString(_arg_, 10).
9. Assert: _arg_ is an Object.
10. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~string~).
11. Assert: _primitiveValue_ is not an Object.
12. Return ? ToString(_primitiveValue_).


## Done when

- Every test262 run under built-ins/Symbol/for/ passes (check with `./t262 built-ins/Symbol/for/`). 8 pass on the main branch right now.

