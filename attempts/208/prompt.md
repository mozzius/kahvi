# Ticket B-object-values: Object.values

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

- `crates/kahvi-builtins/src/object/values.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.values ( _obj_ )
  id: sec-object.values   type: built-in function

This function performs the following steps when called:

1. Let _coerced_ be ? ToObject(_obj_).
2. Let _valueList_ be ? EnumerableOwnProperties(_coerced_, ~value~).
3. Return CreateArrayFromList(_valueList_).

--- referenced: ToObject ( _arg_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-toobject

description: It converts _arg_ to a value of type Object.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. If _arg_ is a Boolean, return a new Boolean object whose [[BooleanData]] internal slot is set to _arg_. See [sec-boolean-objects] for a description of Boolean objects.
3. If _arg_ is a Number, return a new Number object whose [[NumberData]] internal slot is set to _arg_. See [sec-number-objects] for a description of Number objects.
4. If _arg_ is a String, return a new String object whose [[StringData]] internal slot is set to _arg_. See [sec-string-objects] for a description of String objects.
5. If _arg_ is a Symbol, return a new Symbol object whose [[SymbolData]] internal slot is set to _arg_. See [sec-symbol-objects] for a description of Symbol objects.
6. If _arg_ is a BigInt, return a new BigInt object whose [[BigIntData]] internal slot is set to _arg_. See [sec-bigint-objects] for a description of BigInt objects.
7. Assert: _arg_ is an Object.
8. Return _arg_.

--- referenced: EnumerableOwnProperties ( _obj_: an Object, _kind_: ~key~, ~value~, or ~key+value~, ): either a normal completion containing a List of ECMAScript language values or a throw completion
  id: sec-enumerableownproperties

1. Let _ownKeys_ be ? _obj_.[[OwnPropertyKeys]]().
2. Let _results_ be a new empty List.
3. For each element _key_ of _ownKeys_, do
   a. If _key_ is a String, then
      i. Let _propertyDesc_ be ? _obj_.[[GetOwnProperty]](_key_).
      ii. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Enumerable]] is *true*, then
         1. If _kind_ is ~key~, then
            a. Append _key_ to _results_.
         2. Else,
            a. Let _value_ be ? Get(_obj_, _key_).
            b. If _kind_ is ~value~, then
               i. Append _value_ to _results_.
            c. Else,
               i. Assert: _kind_ is ~key+value~.
               ii. Let _entry_ be CreateArrayFromList(« _key_, _value_ »).
               iii. Append _entry_ to _results_.
4. Return _results_.

--- referenced: CreateArrayFromList ( _elements_: a List of ECMAScript language values, ): an Array
  id: sec-createarrayfromlist

description: It is used to create an Array whose elements are provided by _elements_.

1. Let _array_ be ! ArrayCreate(0).
2. Let _n_ be 0.
3. For each element _element_ of _elements_, do
   a. Perform ! CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_n_)), _element_).
   b. Set _n_ to _n_ + 1.
4. Return _array_.


## Done when

- Every test262 run under built-ins/Object/values/ passes (check with `./t262 built-ins/Object/values/`). 8 pass on the main branch right now.

