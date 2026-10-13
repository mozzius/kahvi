//! JSON.parse ( _text_ [ , _reviver_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-json.parse

use kahvi_core::prelude::*;

/// JSON.parse
pub(crate) fn parse(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function parses a JSON text (a JSON-formatted String) and produces an ECMAScript language value. The JSON format represents literals, arrays, and objects with a syntax similar to the syntax for ECMAScript literals, Array Initializers, and Object Initializers. After parsing, JSON objects are realized as ECMAScript objects. JSON arrays are realized as ECMAScript Array instances. JSON strings, numbers, booleans, and null are realized as ECMAScript Strings, Numbers, Booleans, and *null*.
    // The optional _reviver_ parameter is a function that can filter and transform the results. For each value produced by the parse, _reviver_ is called with three arguments (the associated property key, the value, and a context object). If the property is unmodified and its value is primitive, the provided context object has a *"source"* property containing source text of the corresponding Parse Node. If the call returns *undefined*, the property is deleted. Otherwise, the property is redefined to use the return value.
    //
    // 1. Let _jsonString_ be ? ToString(_text_).
    // 2. Let _parseResult_ be ? ParseJSON(_jsonString_).
    // 3. Let _unfiltered_ be _parseResult_.[[Value]].
    // 4. If IsCallable(_reviver_) is *false*, return _unfiltered_.
    // 5. Let _root_ be OrdinaryObjectCreate(%Object.prototype%).
    // 6. Let _rootName_ be the empty String.
    // 7. Perform ! CreateDataPropertyOrThrow(_root_, _rootName_, _unfiltered_).
    // 8. Let _snapshot_ be CreateJSONParseRecord(_parseResult_.[[ParseNode]], _rootName_, _unfiltered_).
    // 9. Return ? InternalizeJSONProperty(_root_, _rootName_, _reviver_, _snapshot_).
    //
    // The *"length"* property of this function is *2*_𝔽.
    // [subclause sec-ParseJSON: ParseJSON (
    // _text_: a String,
    // ): either a normal completion containing a Record with fields [[ParseNode]] (a Parse Node) and [[Value]] (an ECMAScript language value), or a throw completion]
    // [subclause sec-json-parse-record: JSON Parse Record]
    // [subclause sec-createjsonparserecord: CreateJSONParseRecord (
    // _parseNode_: a Parse Node,
    // _key_: a property name,
    // _value_: an ECMAScript language value,
    // ): a JSON Parse Record]
    // [subclause sec-internalizejsonproperty: InternalizeJSONProperty (
    // _holder_: an Object,
    // _name_: a String,
    // _reviver_: a function object,
    // _parseRecord_: a JSON Parse Record or ~empty~,
    // ): either a normal completion containing an ECMAScript language value or a throw completion]
    // [subclause sec-shallowestcontainedjsonvalue: Static Semantics: ShallowestContainedJSONValue (
    // _root_: a Parse Node,
    // ): a Parse Node or ~empty~]
    // [subclause sec-static-semantics-jsonarrayliteralcontentnodes: Static Semantics: JSONArrayLiteralContentNodes ( ): a List of Parse Nodes]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("JSON.parse"))
}
