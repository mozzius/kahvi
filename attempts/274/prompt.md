# Ticket B-symbol-key-for: Symbol.keyFor

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

- `crates/kahvi-builtins/src/symbol/key_for.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Symbol.keyFor ( _symbol_ )
  id: sec-symbol.keyfor   type: built-in function

This function performs the following steps when called:

1. If _symbol_ is not a Symbol, throw a *TypeError* exception.
2. Return KeyForSymbol(_symbol_).

--- referenced: KeyForSymbol ( _symbol_: a Symbol, ): a String or *undefined*
  id: sec-keyforsymbol

description: If _symbol_ is in the surrounding agent's [[GlobalSymbolRegistry]] List, the String used to register _symbol_ will be returned.

1. Let _agentRecord_ be the Agent Record of the surrounding agent.
2. Let _globalSymbolRegistry_ be _agentRecord_.[[GlobalSymbolRegistry]].
3. For each element _element_ of _globalSymbolRegistry_, do
   a. If SameValue(_element_.[[Symbol]], _symbol_) is *true*, return _element_.[[Key]].
4. Assert: _globalSymbolRegistry_ does not currently contain an entry for _symbol_.
5. Return *undefined*.


## Done when

- Every test262 run under built-ins/Symbol/keyFor/ passes (check with `./t262 built-ins/Symbol/keyFor/`). 8 pass on the main branch right now.

