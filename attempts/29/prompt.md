# Ticket B-json-parse: JSON.parse

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

- `crates/kahvi-builtins/src/json/parse.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

JSON.parse ( _text_ [ , _reviver_ ] )
  id: sec-json.parse   type: built-in function

This function parses a JSON text (a JSON-formatted String) and produces an ECMAScript language value. The JSON format represents literals, arrays, and objects with a syntax similar to the syntax for ECMAScript literals, Array Initializers, and Object Initializers. After parsing, JSON objects are realized as ECMAScript objects. JSON arrays are realized as ECMAScript Array instances. JSON strings, numbers, booleans, and null are realized as ECMAScript Strings, Numbers, Booleans, and *null*.
The optional _reviver_ parameter is a function that can filter and transform the results. For each value produced by the parse, _reviver_ is called with three arguments (the associated property key, the value, and a context object). If the property is unmodified and its value is primitive, the provided context object has a *"source"* property containing source text of the corresponding Parse Node. If the call returns *undefined*, the property is deleted. Otherwise, the property is redefined to use the return value.

1. Let _jsonString_ be ? ToString(_text_).
2. Let _parseResult_ be ? ParseJSON(_jsonString_).
3. Let _unfiltered_ be _parseResult_.[[Value]].
4. If IsCallable(_reviver_) is *false*, return _unfiltered_.
5. Let _root_ be OrdinaryObjectCreate(%Object.prototype%).
6. Let _rootName_ be the empty String.
7. Perform ! CreateDataPropertyOrThrow(_root_, _rootName_, _unfiltered_).
8. Let _snapshot_ be CreateJSONParseRecord(_parseResult_.[[ParseNode]], _rootName_, _unfiltered_).
9. Return ? InternalizeJSONProperty(_root_, _rootName_, _reviver_, _snapshot_).

The *"length"* property of this function is *2*_𝔽.
[subclause sec-ParseJSON: ParseJSON (
_text_: a String,
): either a normal completion containing a Record with fields [[ParseNode]] (a Parse Node) and [[Value]] (an ECMAScript language value), or a throw completion]
[subclause sec-json-parse-record: JSON Parse Record]
[subclause sec-createjsonparserecord: CreateJSONParseRecord (
_parseNode_: a Parse Node,
_key_: a property name,
_value_: an ECMAScript language value,
): a JSON Parse Record]
[subclause sec-internalizejsonproperty: InternalizeJSONProperty (
_holder_: an Object,
_name_: a String,
_reviver_: a function object,
_parseRecord_: a JSON Parse Record or ~empty~,
): either a normal completion containing an ECMAScript language value or a throw completion]
[subclause sec-shallowestcontainedjsonvalue: Static Semantics: ShallowestContainedJSONValue (
_root_: a Parse Node,
): a Parse Node or ~empty~]
[subclause sec-static-semantics-jsonarrayliteralcontentnodes: Static Semantics: JSONArrayLiteralContentNodes ( ): a List of Parse Nodes]

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

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

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


## Done when

- Every test262 run under built-ins/JSON/parse/ passes (check with `./t262 built-ins/JSON/parse/`). 8 pass on the main branch right now.

