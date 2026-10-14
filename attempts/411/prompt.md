# Ticket B-json-raw-json: JSON.rawJSON

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

- `crates/kahvi-builtins/src/json/raw_json.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

JSON.rawJSON ( _text_ )
  id: sec-json.rawjson   type: built-in function

This function returns an object representing raw JSON text of a string, number, boolean, or null value.

1. Let _jsonString_ be ? ToString(_text_).
2. If _jsonString_ is the empty String, throw a *SyntaxError* exception.
3. If the first code unit of _jsonString_ is not either an ASCII lowercase letter code unit (0x0061 through 0x007A, inclusive), an ASCII digit code unit (0x0030 through 0x0039, inclusive), 0x0022 (QUOTATION MARK), or 0x002D (HYPHEN-MINUS), throw a *SyntaxError* exception.
4. If the last code unit of _jsonString_ is not either an ASCII lowercase letter code unit (0x0061 through 0x007A, inclusive), an ASCII digit code unit (0x0030 through 0x0039, inclusive), or 0x0022 (QUOTATION MARK), throw a *SyntaxError* exception.
5. Let _parseResult_ be ? ParseJSON(_jsonString_).
6. Assert: _parseResult_.[[Value]] is either a String, a Number, a Boolean, or *null*.
7. Let _internalSlotsList_ be « [[IsRawJSON]] ».
8. Let _obj_ be OrdinaryObjectCreate(*null*, _internalSlotsList_).
9. Perform ! CreateDataPropertyOrThrow(_obj_, *"rawJSON"*, _jsonString_).
10. Perform ! SetIntegrityLevel(_obj_, ~frozen~).
11. Return _obj_.

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

--- referenced: OrdinaryObjectCreate ( _proto_: an Object or *null*, optional _additionalInternalSlotsList_: a List of names of internal slots, ): an Object
  id: sec-ordinaryobjectcreate

description: It is used to specify the runtime creation of new ordinary objects. _additionalInternalSlotsList_ contains the names of additional internal slots that must be defined as part of the object, beyond [[Prototype]] and [[Extensible]]. If _additionalInternalSlotsList_ is not provided, a new empty List is used.

1. Let _internalSlotsList_ be « [[Prototype]], [[Extensible]] ».
2. If _additionalInternalSlotsList_ is present, set _internalSlotsList_ to the list-concatenation of _internalSlotsList_ and _additionalInternalSlotsList_.
3. Let _obj_ be MakeBasicObject(_internalSlotsList_).
4. Set _obj_.[[Prototype]] to _proto_.
5. Return _obj_.

NOTE:
Although OrdinaryObjectCreate does little more than call MakeBasicObject, its use communicates the intention to create an ordinary object, and not an exotic one. Thus, within this specification, it is not called by any algorithm that subsequently modifies the internal methods of the object in ways that would make the result non-ordinary. Operations that create exotic objects invoke MakeBasicObject directly.

--- referenced: CreateDataPropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-createdatapropertyorthrow

description: It is used to create a new own property of an object. It throws a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? CreateDataProperty(_obj_, _propertyKey_, _value_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.

NOTE:
This abstract operation creates a property whose attributes are set to the same defaults used for properties created by the ECMAScript language assignment operator. Normally, the property will not already exist. If it does exist and is not configurable or if _obj_ is not extensible, [[DefineOwnProperty]] will return *false* causing this operation to throw a *TypeError* exception.

--- referenced: SetIntegrityLevel ( _obj_: an Object, _level_: ~sealed~ or ~frozen~, ): either a normal completion containing a Boolean or a throw completion
  id: sec-setintegritylevel

description: It is used to fix the set of own properties of an object.

1. Let _status_ be ? _obj_.[[PreventExtensions]]().
2. If _status_ is *false*, return *false*.
3. Let _keys_ be ? _obj_.[[OwnPropertyKeys]]().
4. If _level_ is ~sealed~, then
   a. For each element _key_ of _keys_, do
      i. Perform ? DefinePropertyOrThrow(_obj_, _key_, PropertyDescriptor { [[Configurable]]: *false* }).
5. Else,
   a. Assert: _level_ is ~frozen~.
   b. For each element _key_ of _keys_, do
      i. Let _currentDesc_ be ? _obj_.[[GetOwnProperty]](_key_).
      ii. If _currentDesc_ is not *undefined*, then
         1. If IsAccessorDescriptor(_currentDesc_) is *true*, then
            a. Let _propertyDesc_ be the PropertyDescriptor { [[Configurable]]: *false* }.
         2. Else,
            a. Let _propertyDesc_ be the PropertyDescriptor { [[Configurable]]: *false*, [[Writable]]: *false* }.
         3. Perform ? DefinePropertyOrThrow(_obj_, _key_, _propertyDesc_).
6. Return *true*.


## Done when

- Every test262 run under built-ins/JSON/rawJSON/ passes (check with `./t262 built-ins/JSON/rawJSON/`). 10 pass on the main branch right now.

