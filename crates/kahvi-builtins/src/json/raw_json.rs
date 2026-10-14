//! JSON.rawJSON ( _text_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-json.rawjson

use kahvi_core::prelude::*;

/// The [[IsRawJSON]] internal slot. The object carries no other state.
#[derive(Debug)]
pub(crate) struct RawJsonData;

impl kahvi_core::gc::Trace for RawJsonData {
    fn trace(&self, _t: &mut kahvi_core::gc::Tracer) {}
}

/// JSON.rawJSON
pub(crate) fn raw_json(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns an object representing raw JSON text of a string, number, boolean, or null value.
    //
    // 1. Let _jsonString_ be ? ToString(_text_).
    let json_string = ops::to_string(agent, args.get_or_undefined(0))?;
    // 2. If _jsonString_ is the empty String, throw a *SyntaxError* exception.
    let first = agent.str(json_string).first().copied();
    let Some(first) = first else {
        return Err(agent.throw_syntax_error("JSON.rawJSON: text must not be empty"));
    };
    // 3. If the first code unit of _jsonString_ is not either an ASCII lowercase letter code unit (0x0061 through 0x007A, inclusive), an ASCII digit code unit (0x0030 through 0x0039, inclusive), 0x0022 (QUOTATION MARK), or 0x002D (HYPHEN-MINUS), throw a *SyntaxError* exception.
    if !matches!(first, 0x61..=0x7A | 0x30..=0x39 | 0x22 | 0x2D) {
        return Err(agent.throw_syntax_error("JSON.rawJSON: invalid first character"));
    }
    // 4. If the last code unit of _jsonString_ is not either an ASCII lowercase letter code unit (0x0061 through 0x007A, inclusive), an ASCII digit code unit (0x0030 through 0x0039, inclusive), or 0x0022 (QUOTATION MARK), throw a *SyntaxError* exception.
    let last = agent.str(json_string).last().copied().unwrap_or(0);
    if !matches!(last, 0x61..=0x7A | 0x30..=0x39 | 0x22) {
        return Err(agent.throw_syntax_error("JSON.rawJSON: invalid last character"));
    }
    // 5. Let _parseResult_ be ? ParseJSON(_jsonString_).
    // ParseJSON is `parse_json` in json/parse.rs, which is private to that module, so it
    // cannot be called from here yet. Until it is exposed as pub(crate), this step throws.
    let _parse_result: () = Err(agent.not_implemented(
        "ParseJSON for JSON.rawJSON (json/parse.rs parse_json is not pub(crate))",
    ))?;
    // 6. Assert: _parseResult_.[[Value]] is either a String, a Number, a Boolean, or *null*.
    // 7. Let _internalSlotsList_ be « [[IsRawJSON]] ».
    // 8. Let _obj_ be OrdinaryObjectCreate(*null*, _internalSlotsList_).
    let obj = agent.object_with_kind(None, ObjectKind::Native(Box::new(RawJsonData)));
    agent.with_roots(&[Value::Object(obj), Value::String(json_string)], |agent| {
        // 9. Perform ! CreateDataPropertyOrThrow(_obj_, *"rawJSON"*, _jsonString_).
        let raw_json_key = agent.key("rawJSON");
        ops::create_data_property_or_throw(agent, obj, raw_json_key, Value::String(json_string))?;
        // 10. Perform ! SetIntegrityLevel(_obj_, ~frozen~).
        ops::set_integrity_level(agent, obj, ops::IntegrityLevel::Frozen)?;
        // 11. Return _obj_.
        Ok(Value::Object(obj))
    })
}
