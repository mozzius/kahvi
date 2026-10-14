# Ticket B-set-iterator-prototype-next: SetIteratorPrototype.next

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

- `crates/kahvi-builtins/src/set_iterator_prototype/next.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

%SetIteratorPrototype%.next ( )
  id: sec-%setiteratorprototype%.next   type: built-in function

1. Return ? GeneratorResume(*this* value, ~empty~, *"%SetIteratorPrototype%"*).

--- referenced: GeneratorResume ( _gen_: an ECMAScript language value, _value_: an ECMAScript language value or ~empty~, _genBrand_: a String or ~empty~, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-generatorresume

1. Let _state_ be ? GeneratorValidate(_gen_, _genBrand_).
2. If _state_ is ~completed~, return CreateIteratorResultObject(*undefined*, *true*).
3. Assert: _state_ is either ~suspended-start~ or ~suspended-yield~.
4. Let _genContext_ be _gen_.[[GeneratorContext]].
5. Set _gen_.[[GeneratorState]] to ~executing~.
6. Return ? RunSuspendedContext(_genContext_, NormalCompletion(_value_)).


## Done when

- Every test262 run under built-ins/SetIteratorPrototype/next/ passes (check with `./t262 built-ins/SetIteratorPrototype/next/`). 4 pass on the main branch right now.

