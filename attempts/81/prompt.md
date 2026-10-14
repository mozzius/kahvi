# Ticket B-set-prototype-values: Set.prototype.values

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

- `crates/kahvi-builtins/src/set/prototype_values.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Set.prototype.values ( )
  id: sec-set.prototype.values   type: built-in function

This method performs the following steps when called:

1. Let _set_ be the *this* value.
2. Return ? CreateSetIterator(_set_, ~value~).

--- referenced: CreateSetIterator ( _set_: an ECMAScript language value, _kind_: ~key+value~ or ~value~, ): either a normal completion containing a Generator or a throw completion
  id: sec-createsetiterator

description: It is used to create iterator objects for Set methods that return such iterators.

1. Perform ? RequireInternalSlot(_set_, [[SetData]]).
2. Let _closure_ be a new Abstract Closure with no parameters that captures _set_ and _kind_ and performs the following steps when called:
   a. Let _index_ be 0.
   b. Let _entries_ be _set_.[[SetData]].
   c. Let _entriesCount_ be the number of elements in _entries_.
   d. Repeat, while _index_ < _entriesCount_,
      i. Let _entry_ be _entries_[_index_].
      ii. Set _index_ to _index_ + 1.
      iii. If _entry_ is not ~empty~, then
         1. If _kind_ is ~key+value~, then
            a. Let _result_ be CreateArrayFromList(« _entry_, _entry_ »).
            b. Perform ? GeneratorYield(CreateIteratorResultObject(_result_, *false*)).
         2. Else,
            a. Assert: _kind_ is ~value~.
            b. Perform ? GeneratorYield(CreateIteratorResultObject(_entry_, *false*)).
         3. NOTE: The number of elements in _entries_ may have increased while execution of this abstract operation was paused by GeneratorYield.
         4. Set _entriesCount_ to the number of elements in _entries_.
   e. Return NormalCompletion(~unused~).
3. Return CreateIteratorFromClosure(_closure_, *"%SetIteratorPrototype%"*, %SetIteratorPrototype%).


## Done when

- Every test262 run under built-ins/Set/prototype/values/ passes (check with `./t262 built-ins/Set/prototype/values/`). 8 pass on the main branch right now.

