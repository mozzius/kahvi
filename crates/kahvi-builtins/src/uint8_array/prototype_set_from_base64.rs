//! Uint8Array.prototype.setFromBase64 ( _string_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-uint8array.prototype.setfrombase64

use kahvi_core::object::typed_array::{ElementType, set_value_in_buffer};
use kahvi_core::prelude::*;

use super::from_base64::{
    from_base64_bytes, get_alphabet_option, get_last_chunk_handling_option, get_options_object,
};
use crate::support::typed_array::{
    byte_offset, element_type, length, out_of_bounds, ta_record, viewed_buffer,
};

/// Uint8Array.prototype.setFromBase64
pub(crate) fn prototype_set_from_base64(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _into_ be the *this* value.
    // 2. Perform ? ValidateUint8Array(_into_).
    // ValidateUint8Array: 1. Perform ? RequireInternalSlot(_ta_, [[TypedArrayName]]).
    // 2. If _ta_.[[TypedArrayName]] is not *"Uint8Array"*, throw a *TypeError* exception.
    let into = match (this, element_type(agent, this)) {
        (Value::Object(o), Some(ElementType::Uint8)) => o,
        _ => return Err(agent.throw_type_error("this is not a Uint8Array")),
    };
    // 3. If _string_ is not a String, throw a *TypeError* exception.
    let Value::String(string) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Uint8Array.prototype.setFromBase64 requires a string"));
    };
    // 4. Let _optionsObj_ be ? GetOptionsObject(_options_).
    let options_obj = get_options_object(agent, args.get_or_undefined(1))?;
    // 5-7. Let _alphabet_ be ? Get(_optionsObj_, *"alphabet"*), defaulting to *"base64"*; throw a *TypeError* if invalid.
    let alphabet = get_alphabet_option(agent, options_obj)?;
    // 8-10. Let _lastChunkHandling_ be ? Get(_optionsObj_, *"lastChunkHandling"*), defaulting to *"loose"*; throw a *TypeError* if invalid.
    let last_chunk_handling = get_last_chunk_handling_option(agent, options_obj)?;
    // 11. Let _taRecord_ be ? ValidateTypedArrayBounds(_into_, ~seq-cst~).
    let ta_rec = ta_record(agent, into);
    if out_of_bounds(agent, ta_rec) {
        return Err(agent.throw_type_error("TypedArray is out of bounds or detached"));
    }
    // 12. Let _byteLength_ be TypedArrayLength(_taRecord_).
    let byte_length = length(agent, ta_rec);
    // 13. Let _result_ be FromBase64(_string_, _alphabet_, _lastChunkHandling_, _byteLength_).
    let (read, bytes, error) = from_base64_bytes(
        agent,
        string,
        alphabet,
        last_chunk_handling,
        Some(byte_length),
    );
    // 14. Let _bytes_ be _result_.[[Bytes]].
    // 15. Let _written_ be the number of elements in _bytes_.
    let written = bytes.len();
    // 16. NOTE: FromBase64 does not invoke any user code, so the ArrayBuffer backing _into_ cannot have been detached or shrunk.
    // 17. Assert: _written_ ≤ _byteLength_.
    debug_assert!(written <= byte_length);
    // 18. Perform SetUint8ArrayBytes(_into_, _bytes_).
    // SetUint8ArrayBytes: 1. Let _offset_ be _into_.[[ByteOffset]].
    let offset = byte_offset(agent, into);
    let buffer = viewed_buffer(agent, into);
    // 2-4. For each _index_ < the number of elements in _bytes_, perform
    // SetValueInBuffer(_into_.[[ViewedArrayBuffer]], _index_ + _offset_, ~uint8~, 𝔽(_byte_), *true*, ~unordered~).
    for (index, byte) in bytes.into_iter().enumerate() {
        set_value_in_buffer(
            agent,
            buffer,
            index + offset,
            ElementType::Uint8,
            Value::Number(f64::from(byte)),
            true,
        );
    }
    // 19. If _result_.[[Error]] is not ~none~, then
    if let Some(error) = error {
        // a. Throw _result_.[[Error]].
        return Err(error);
    }
    // 20. Let _resultObj_ be OrdinaryObjectCreate(%Object.prototype%).
    let object_prototype = agent.intrinsic("%Object.prototype%");
    let result_obj = agent.ordinary_object_create(Some(object_prototype));
    // 21. Perform ! CreateDataPropertyOrThrow(_resultObj_, *"read"*, 𝔽(_result_.[[Read]])).
    let read_key = agent.key("read");
    ops::create_data_property_or_throw(agent, result_obj, read_key, Value::Number(read as f64))?;
    // 22. Perform ! CreateDataPropertyOrThrow(_resultObj_, *"written"*, 𝔽(_written_)).
    let written_key = agent.key("written");
    ops::create_data_property_or_throw(
        agent,
        result_obj,
        written_key,
        Value::Number(written as f64),
    )?;
    // 23. Return _resultObj_.
    Ok(Value::Object(result_obj))
}
