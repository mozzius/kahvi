//! JSON.parse ( _text_ [ , _reviver_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-json.parse

use kahvi_core::ops::EnumerableKind;
use kahvi_core::prelude::*;

/// JSON.parse
pub(crate) fn parse(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function parses a JSON text (a JSON-formatted String) and produces an ECMAScript language value. The JSON format represents literals, arrays, and objects with a syntax similar to the syntax for ECMAScript literals, Array Initializers, and Object Initializers. After parsing, JSON objects are realized as ECMAScript objects. JSON arrays are realized as ECMAScript Array instances. JSON strings, numbers, booleans, and null are realized as ECMAScript Strings, Numbers, Booleans, and *null*.
    // The optional _reviver_ parameter is a function that can filter and transform the results. For each value produced by the parse, _reviver_ is called with three arguments (the associated property key, the value, and a context object). If the property is unmodified and its value is primitive, the provided context object has a *"source"* property containing source text of the corresponding Parse Node. If the call returns *undefined*, the property is deleted. Otherwise, the property is redefined to use the return value.
    //
    // 1. Let _jsonString_ be ? ToString(_text_).
    let json_string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 2. Let _parseResult_ be ? ParseJSON(_jsonString_).
    let text = agent.str(json_string).to_vec();
    let parse_result = parse_json(agent, &text)?;
    // 3. Let _unfiltered_ be _parseResult_.[[Value]].
    let unfiltered = parse_result.value;
    // 4. If IsCallable(_reviver_) is *false*, return _unfiltered_.
    let reviver = args.get_or_undefined(1);
    if !ops::is_callable(agent, reviver) {
        return Ok(unfiltered);
    }
    // 5. Let _root_ be OrdinaryObjectCreate(%Object.prototype%).
    let object_proto = agent.intrinsic("%Object.prototype%");
    let root = agent.ordinary_object_create(Some(object_proto));
    // 6. Let _rootName_ be the empty String.
    let root_name = agent.intern(&[]);
    // 7. Perform ! CreateDataPropertyOrThrow(_root_, _rootName_, _unfiltered_).
    let root_key = ops::to_property_key(agent, Value::String(root_name))?;
    ops::create_data_property_or_throw(agent, root, root_key, unfiltered)?;
    // 8. Let _snapshot_ be CreateJSONParseRecord(_parseResult_.[[ParseNode]], _rootName_, _unfiltered_).
    // The JSON Parse Record of every value is built while parsing, so the record for the
    // root of the parse result is the snapshot.
    // 9. Return ? InternalizeJSONProperty(_root_, _rootName_, _reviver_, _snapshot_).
    internalize_json_property(agent, root, root_name, reviver, Some(&parse_result))
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
}

/// A JSON Parse Record (sec-json-parse-record), built while parsing.
///
/// [[ParseNode]] is represented by `source` (the source text of a primitive),
/// [[Elements]] by `elements` and [[Entries]] by `entries`.
struct JsonParseRecord {
    source: Vec<u16>,
    value: Value,
    elements: Vec<JsonParseRecord>,
    entries: Vec<(Vec<u16>, JsonParseRecord)>,
}

/// ParseJSON ( _text_ )
///
/// Parses `text` as a JSON text and returns its JSON Parse Record. Malformed
/// input throws a SyntaxError.
fn parse_json(agent: &mut Agent, text: &[u16]) -> JsResult<JsonParseRecord> {
    // 1. Let parseResult be ParseText(text, JSONText).
    // 2. If parseResult is a non-empty List of errors, throw a SyntaxError exception.
    // 3. Return the Record { [[ParseNode]]: parseResult, [[Value]]: the JSON value }.
    let mut parser = JsonParser {
        agent,
        text,
        pos: 0,
    };
    let result = parser.parse_value()?;
    parser.skip_ws();
    if parser.pos != text.len() {
        return parser.fail();
    }
    Ok(result)
}

/// Recursive-descent parser for the JSON grammar over UTF-16 code units.
struct JsonParser<'a> {
    agent: &'a mut Agent,
    text: &'a [u16],
    pos: usize,
}

impl JsonParser<'_> {
    fn peek(&self) -> Option<u16> {
        self.text.get(self.pos).copied()
    }

    fn slice(&self, start: usize, end: usize) -> &[u16] {
        self.text.get(start..end).unwrap_or(&[])
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(0x09 | 0x0A | 0x0D | 0x20)) {
            self.pos += 1;
        }
    }

    fn skip_digits(&mut self) {
        while matches!(self.peek(), Some(0x30..=0x39)) {
            self.pos += 1;
        }
    }

    fn require_digits(&mut self) -> JsResult<()> {
        if !matches!(self.peek(), Some(0x30..=0x39)) {
            return self.fail();
        }
        self.skip_digits();
        Ok(())
    }

    fn fail<T>(&mut self) -> JsResult<T> {
        Err(self.agent.throw_syntax_error("Unexpected token in JSON"))
    }

    fn primitive(&self, start: usize, value: Value) -> JsonParseRecord {
        JsonParseRecord {
            source: self.slice(start, self.pos).to_vec(),
            value,
            elements: Vec::new(),
            entries: Vec::new(),
        }
    }

    /// JSONValue : JSONNullLiteral | JSONBooleanLiteral | JSONString | JSONNumber | JSONObject | JSONArray
    fn parse_value(&mut self) -> JsResult<JsonParseRecord> {
        self.skip_ws();
        let start = self.pos;
        let value = match self.peek() {
            Some(0x22) => {
                let units = self.parse_string()?;
                Value::String(self.agent.intern(&units))
            }
            Some(0x7B) => return self.parse_object(),
            Some(0x5B) => return self.parse_array(),
            Some(0x2D | 0x30..=0x39) => Value::Number(self.parse_number()?),
            Some(0x74) => self.parse_literal("true", Value::Boolean(true))?,
            Some(0x66) => self.parse_literal("false", Value::Boolean(false))?,
            Some(0x6E) => self.parse_literal("null", Value::Null)?,
            _ => return self.fail(),
        };
        Ok(self.primitive(start, value))
    }

    fn parse_literal(&mut self, word: &str, value: Value) -> JsResult<Value> {
        for unit in word.bytes() {
            if self.peek() != Some(u16::from(unit)) {
                return self.fail();
            }
            self.pos += 1;
        }
        Ok(value)
    }

    /// JSONNumber : -? ( 0 | [1-9][0-9]* ) ( . [0-9]+ )? ( [eE] [+-]? [0-9]+ )?
    fn parse_number(&mut self) -> JsResult<f64> {
        let start = self.pos;
        if self.peek() == Some(0x2D) {
            self.pos += 1;
        }
        match self.peek() {
            Some(0x30) => self.pos += 1,
            Some(0x31..=0x39) => self.skip_digits(),
            _ => return self.fail(),
        }
        if self.peek() == Some(0x2E) {
            self.pos += 1;
            self.require_digits()?;
        }
        if matches!(self.peek(), Some(0x65 | 0x45)) {
            self.pos += 1;
            if matches!(self.peek(), Some(0x2B | 0x2D)) {
                self.pos += 1;
            }
            self.require_digits()?;
        }
        let literal: String = self
            .slice(start, self.pos)
            .iter()
            .map(|&unit| char::from(unit as u8))
            .collect();
        match literal.parse::<f64>() {
            Ok(number) => Ok(number),
            Err(_) => self.fail(),
        }
    }

    /// JSONString : " JSONStringCharacter* "
    fn parse_string(&mut self) -> JsResult<Vec<u16>> {
        // The opening quote has been checked by the caller.
        self.pos += 1;
        let mut units = Vec::new();
        loop {
            let Some(unit) = self.peek() else {
                return self.fail();
            };
            self.pos += 1;
            match unit {
                0x22 => return Ok(units),
                0x5C => {
                    let Some(escape) = self.peek() else {
                        return self.fail();
                    };
                    self.pos += 1;
                    let decoded = match escape {
                        0x22 | 0x5C | 0x2F => escape,
                        0x62 => 0x08,
                        0x66 => 0x0C,
                        0x6E => 0x0A,
                        0x72 => 0x0D,
                        0x74 => 0x09,
                        0x75 => self.parse_hex4()?,
                        _ => return self.fail(),
                    };
                    units.push(decoded);
                }
                0x00..=0x1F => return self.fail(),
                _ => units.push(unit),
            }
        }
    }

    /// The four hex digits of a `\u` escape.
    fn parse_hex4(&mut self) -> JsResult<u16> {
        let mut value: u16 = 0;
        for _ in 0..4 {
            let Some(digit) = self.peek().and_then(hex_digit_value) else {
                return self.fail();
            };
            self.pos += 1;
            value = value * 16 + digit;
        }
        Ok(value)
    }

    /// JSONArray : [ ] | [ ElementList ]
    fn parse_array(&mut self) -> JsResult<JsonParseRecord> {
        // The opening bracket is the current unit.
        self.pos += 1;
        let mut values = Vec::new();
        let mut elements = Vec::new();
        self.skip_ws();
        if self.peek() == Some(0x5D) {
            self.pos += 1;
        } else {
            loop {
                let record = self.parse_value()?;
                values.push(record.value);
                elements.push(record);
                self.skip_ws();
                match self.peek() {
                    Some(0x2C) => self.pos += 1,
                    Some(0x5D) => {
                        self.pos += 1;
                        break;
                    }
                    _ => return self.fail(),
                }
            }
        }
        let array = ops::create_array_from_list(self.agent, &values);
        Ok(JsonParseRecord {
            source: Vec::new(),
            value: Value::Object(array),
            elements,
            entries: Vec::new(),
        })
    }

    /// JSONObject : { } | { JSONMemberList }
    fn parse_object(&mut self) -> JsResult<JsonParseRecord> {
        // The opening brace is the current unit.
        self.pos += 1;
        let object_proto = self.agent.intrinsic("%Object.prototype%");
        let object = self.agent.ordinary_object_create(Some(object_proto));
        let mut entries: Vec<(Vec<u16>, JsonParseRecord)> = Vec::new();
        self.skip_ws();
        if self.peek() == Some(0x7D) {
            self.pos += 1;
        } else {
            loop {
                self.skip_ws();
                if self.peek() != Some(0x22) {
                    return self.fail();
                }
                let name = self.parse_string()?;
                self.skip_ws();
                if self.peek() != Some(0x3A) {
                    return self.fail();
                }
                self.pos += 1;
                let record = self.parse_value()?;
                let name_string = self.agent.intern(&name);
                let key = ops::to_property_key(self.agent, Value::String(name_string))?;
                ops::create_data_property_or_throw(self.agent, object, key, record.value)?;
                // A repeated name replaces the earlier member, keeping its position.
                match entries.iter_mut().find(|entry| entry.0 == name) {
                    Some(entry) => entry.1 = record,
                    None => entries.push((name, record)),
                }
                self.skip_ws();
                match self.peek() {
                    Some(0x2C) => self.pos += 1,
                    Some(0x7D) => {
                        self.pos += 1;
                        break;
                    }
                    _ => return self.fail(),
                }
            }
        }
        Ok(JsonParseRecord {
            source: Vec::new(),
            value: Value::Object(object),
            elements: Vec::new(),
            entries,
        })
    }
}

fn hex_digit_value(unit: u16) -> Option<u16> {
    match unit {
        0x30..=0x39 => Some(unit - 0x30),
        0x41..=0x46 => Some(unit - 0x41 + 10),
        0x61..=0x66 => Some(unit - 0x61 + 10),
        _ => None,
    }
}

/// InternalizeJSONProperty ( _holder_, _name_, _reviver_, _parseRecord_ )
fn internalize_json_property(
    agent: &mut Agent,
    holder: Object,
    name: JsString,
    reviver: Value,
    parse_record: Option<&JsonParseRecord>,
) -> JsResult<Value> {
    // 1. Let val be ? Get(holder, name).
    let name_key = ops::to_property_key(agent, Value::String(name))?;
    let val = ops::get(agent, holder, name_key)?;
    // 2. If val is an Object, then
    if let Value::Object(val_obj) = val {
        // a. Let isArray be ? IsArray(val).
        // b. If isArray is true, then
        if ops::is_array(agent, val)? {
            // i. Let len be ? LengthOfArrayLike(val).
            let len = ops::length_of_array_like(agent, val_obj)?;
            // ii. Let I be 0.
            let mut i = 0.0;
            // iii. Repeat, while I < len,
            while i < len {
                // 1. Let prop be ! ToString(𝔽(I)).
                let prop = ops::to_string(agent, Value::Number(i))?;
                // 2. Let elementRecord be empty, or the element record of parseRecord
                //    when parseRecord.[[Value]] is SameValue to val.
                let element_record = match parse_record {
                    Some(record) if ops::same_value(agent, record.value, val) => {
                        record.elements.get(i as usize)
                    }
                    _ => None,
                };
                // 3. Let newElement be ? InternalizeJSONProperty(val, prop, reviver, elementRecord).
                // 4. If newElement is undefined, perform ? val.[[Delete]](prop).
                // 5. Else, perform ? CreateDataProperty(val, prop, newElement).
                internalize_child(agent, val_obj, prop, reviver, element_record)?;
                // 6. Set I to I + 1.
                i += 1.0;
            }
        } else {
            // c. Else,
            // i. Let keys be ? EnumerableOwnProperties(val, key).
            let keys = ops::enumerable_own_properties(agent, val_obj, EnumerableKind::Key)?;
            // ii. For each String P of keys, do
            for key in keys {
                let prop = ops::to_string(agent, key)?;
                // 1. Let entryRecord be empty, or the entry record of parseRecord whose key is P
                //    when parseRecord.[[Value]] is SameValue to val.
                let entry_record = match parse_record {
                    Some(record) if ops::same_value(agent, record.value, val) => record
                        .entries
                        .iter()
                        .find(|entry| entry.0.as_slice() == agent.str(prop))
                        .map(|entry| &entry.1),
                    _ => None,
                };
                // 2. Let newElement be ? InternalizeJSONProperty(val, P, reviver, entryRecord).
                // 3. If newElement is undefined, perform ? val.[[Delete]](P).
                // 4. Else, perform ? CreateDataProperty(val, P, newElement).
                internalize_child(agent, val_obj, prop, reviver, entry_record)?;
            }
        }
    }
    // 3. Let context be OrdinaryObjectCreate(%Object.prototype%).
    let object_proto = agent.intrinsic("%Object.prototype%");
    let context = agent.ordinary_object_create(Some(object_proto));
    // 4. If parseRecord is not empty and val is not an Object and SameValue(parseRecord.[[Value]], val) is true, then
    if let Some(record) = parse_record {
        if !matches!(val, Value::Object(_)) && ops::same_value(agent, record.value, val) {
            // a. Let sourceText be the source text matched by the JSONText production that produced val.
            let source_text = agent.intern(&record.source);
            // b. Perform ! CreateDataPropertyOrThrow(context, "source", sourceText).
            let source_key = agent.key("source");
            ops::create_data_property_or_throw(
                agent,
                context,
                source_key,
                Value::String(source_text),
            )?;
        }
    }
    // 5. Return ? Call(_reviver_, _holder_, « _name_, _val_, _context_ »).
    ops::call(
        agent,
        reviver,
        Value::Object(holder),
        &[Value::String(name), val, Value::Object(context)],
    )
}

/// Steps 2 to 5 of InternalizeJSONProperty for one child property `prop` of `val`.
fn internalize_child(
    agent: &mut Agent,
    val: Object,
    prop: JsString,
    reviver: Value,
    parse_record: Option<&JsonParseRecord>,
) -> JsResult<()> {
    // 1. Let newElement be ? InternalizeJSONProperty(val, prop, reviver, parseRecord).
    let new_element = internalize_json_property(agent, val, prop, reviver, parse_record)?;
    let prop_key = ops::to_property_key(agent, Value::String(prop))?;
    // 2. If newElement is undefined, then
    if matches!(new_element, Value::Undefined) {
        // a. Perform ? val.[[Delete]](prop).
        val.internal_delete(agent, prop_key)?;
    } else {
        // a. Perform ? CreateDataProperty(val, prop, newElement).
        ops::create_data_property(agent, val, prop_key, new_element)?;
    }
    Ok(())
}
