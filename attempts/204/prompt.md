# Ticket B-aggregate-error-constructor: AggregateError

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

- `crates/kahvi-builtins/src/aggregate_error/constructor.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

AggregateError ( _errors_, _message_ [ , _options_ ] )
  id: sec-aggregate-error   type: built-in function

This function performs the following steps when called:

1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
2. Let _obj_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%AggregateError.prototype%"*, « [[ErrorData]] »).
3. If _message_ is not *undefined*, then
   a. Let _messageString_ be ? ToString(_message_).
   b. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"message"*, _messageString_).
4. Perform ? InstallErrorCause(_obj_, _options_).
5. Let _errorsList_ be ? IteratorToList(? GetIterator(_errors_, ~sync~)).
6. Perform ! DefinePropertyOrThrow(_obj_, *"errors"*, PropertyDescriptor { [[Configurable]]: *true*, [[Enumerable]]: *false*, [[Writable]]: *true*, [[Value]]: CreateArrayFromList(_errorsList_) }).
7. Return _obj_.

--- referenced: OrdinaryCreateFromConstructor ( _ctor_: a function object, _intrinsicDefaultProto_: a String, optional _internalSlotsList_: a List of names of internal slots, ): either a normal completion containing an Object or a throw completion
  id: sec-ordinarycreatefromconstructor

description: It creates an ordinary object whose [[Prototype]] value is retrieved from a constructor's *"prototype"* property, if it exists. Otherwise the intrinsic named by _intrinsicDefaultProto_ is used for [[Prototype]]. _internalSlotsList_ contains the names of additional internal slots that must be defined as part of the object. If _internalSlotsList_ is not provided, a new empty List is used.

1. Assert: _intrinsicDefaultProto_ is this specification's name of an intrinsic object. The corresponding object must be an intrinsic that is intended to be used as the [[Prototype]] value of an object.
2. Let _proto_ be ? GetPrototypeFromConstructor(_ctor_, _intrinsicDefaultProto_).
3. If _internalSlotsList_ is present, let _slots_ be _internalSlotsList_.
4. Else, let _slots_ be a new empty List.
5. Return OrdinaryObjectCreate(_proto_, _slots_).

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

--- referenced: CreateNonEnumerableDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): ~unused~
  id: sec-createnonenumerabledatapropertyorthrow

description: It is used to create a new non-enumerable own property of an ordinary object.

1. Assert: _obj_ is an ordinary, extensible object with no non-configurable properties.
2. Let _newDesc_ be the PropertyDescriptor { [[Value]]: _value_, [[Writable]]: *true*, [[Enumerable]]: *false*, [[Configurable]]: *true* }.
3. Perform ! DefinePropertyOrThrow(_obj_, _propertyKey_, _newDesc_).
4. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator except it is not enumerable. Normally, the property will not already exist. If it does exist, DefinePropertyOrThrow is guaranteed to complete normally.

--- referenced: InstallErrorCause ( _obj_: an Object, _options_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-installerrorcause

description: It is used to create a *"cause"* property on _obj_ when a *"cause"* property is present on _options_.

1. If _options_ is an Object and ? HasProperty(_options_, *"cause"*) is *true*, then
   a. Let _cause_ be ? Get(_options_, *"cause"*).
   b. Perform CreateNonEnumerableDataPropertyOrThrow(_obj_, *"cause"*, _cause_).
2. Return ~unused~.

--- referenced: IteratorToList ( _iteratorRecord_: an Iterator Record, ): either a normal completion containing a List of ECMAScript language values or a throw completion
  id: sec-iteratortolist

1. Let _values_ be a new empty List.
2. Repeat,
   a. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
   b. If _next_ is ~done~, then
      i. Return _values_.
   c. Append _next_ to _values_.

--- referenced: GetIterator ( _obj_: an ECMAScript language value, _kind_: ~sync~ or ~async~, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiterator

1. If _kind_ is ~async~, then
   a. Let _method_ be ? GetMethod(_obj_, %Symbol.asyncIterator%).
   b. If _method_ is *undefined*, then
      i. Let _syncMethod_ be ? GetMethod(_obj_, %Symbol.iterator%).
      ii. If _syncMethod_ is *undefined*, throw a *TypeError* exception.
      iii. Let _syncIteratorRecord_ be ? GetIteratorFromMethod(_obj_, _syncMethod_).
      iv. Return CreateAsyncFromSyncIterator(_syncIteratorRecord_).
2. Else,
   a. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
3. If _method_ is *undefined*, throw a *TypeError* exception.
4. Return ? GetIteratorFromMethod(_obj_, _method_).

--- referenced: DefinePropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _propertyDesc_: a Property Descriptor, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-definepropertyorthrow

description: It is used to call the [[DefineOwnProperty]] internal method of an object in a manner that will throw a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? _obj_.[[DefineOwnProperty]](_propertyKey_, _propertyDesc_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

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

- Every test262 run under built-ins/AggregateError/ passes (check with `./t262 built-ins/AggregateError/`). 20 pass on the main branch right now.

