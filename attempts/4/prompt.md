# Ticket B-object-define-properties: Object.defineProperties

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

- `crates/kahvi-builtins/src/object/define_properties.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.defineProperties ( _obj_, _properties_ )
  id: sec-object.defineproperties   type: built-in function

This function adds own properties and/or updates the attributes of existing own properties of an object.
It performs the following steps when called:

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. Return ? ObjectDefineProperties(_obj_, _properties_).

[subclause sec-objectdefineproperties: ObjectDefineProperties (
_obj_: an Object,
_properties_: an ECMAScript language value,
): either a normal completion containing an Object or a throw completion]

--- referenced: ObjectDefineProperties ( _obj_: an Object, _properties_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-objectdefineproperties

1. Set _properties_ to ? ToObject(_properties_).
2. Let _keys_ be ? _properties_.[[OwnPropertyKeys]]().
3. Let _propertyDescs_ be a new empty List.
4. For each element _nextKey_ of _keys_, do
   a. Let _currentPropertyDesc_ be ? _properties_.[[GetOwnProperty]](_nextKey_).
   b. If _currentPropertyDesc_ is not *undefined* and _currentPropertyDesc_.[[Enumerable]] is *true*, then
      i. Let _propertyDescObj_ be ? Get(_properties_, _nextKey_).
      ii. Let _propertyDesc_ be ? ToPropertyDescriptor(_propertyDescObj_).
      iii. Append the Record { [[Key]]: _nextKey_, [[Descriptor]]: _propertyDesc_ } to _propertyDescs_.
5. For each element _property_ of _propertyDescs_, do
   a. Perform ? DefinePropertyOrThrow(_obj_, _property_.[[Key]], _property_.[[Descriptor]]).
6. Return _obj_.


## Done when

- Every test262 run under built-ins/Object/defineProperties/ passes (check with `./t262 built-ins/Object/defineProperties/`). 1208 pass on the main branch right now.

## Previous attempts

- attempt 1: worker reported BLOCKED but changed files: engine support for Date, RegExp, Object.preventExtensions, Function and Proxy (used by the defineProperties tests, not by this built-in)
- attempt 1: merged partial progress. Still failing:
built-ins/Object/defineProperties/15.2.3.7-2-12.js:sloppy  Uncaught InternalError: not implemented: Date
built-ins/Object/defineProperties/15.2.3.7-2-12.js:strict  Uncaught InternalError: not implemented: Date
built-ins/Object/defineProperties/15.2.3.7-2-13.js:sloppy  Uncaught InternalError: not implemented: RegExp
built-ins/Object/defineProperties/15.2.3.7-2-13.js:strict  Uncaught InternalError: not implemented: RegExp
built-ins/Object/defineProperties/15.2.3.7-5-a-13.js:sloppy  Uncaught InternalError: not implemented: Date
built-ins/Object/defineProperties/15.2.3.7-5-a-13.js:strict  Uncaught InternalError: not implemented: Date
built-ins/Object/defineProperties/15.2.3.7-5-a-14.js:sloppy  Uncaught InternalError: not implemented: RegExp
built-ins/Object/defineProperties/15.2.3.7-5-a-14.js:strict  Uncaught InternalError: not implemented: RegExp
built-ins/Object/defineProperties/15.2.3.7-5-b-103.js:sloppy  Uncaught InternalError: not implemented: Date
built-ins/Object/defineProperties/15.2.3.7-5-b-103.js:strict  Uncaught InternalError: not implemented: Date

