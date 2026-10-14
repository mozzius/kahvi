//! TypedArray, ArrayBuffer and DataView abstract operations shared by the
//! built-ins (§23.2, §25.1, §25.3). The internal slots, buffer operations and
//! exotic internal methods are in `kahvi_core::object::typed_array`.
#![allow(dead_code)] // helpers for the TypedArray, ArrayBuffer and DataView tickets

use kahvi_core::object::typed_array::{
    DataViewData, ElementType, TaRecord, TypedArrayData, allocate_array_buffer,
    array_buffer_byte_length, array_buffer_data, data_view_data, get_value_from_buffer,
    is_detached_buffer, is_fixed_length_array_buffer, is_typed_array_out_of_bounds, make_ta_record,
    set_value_in_buffer, typed_array_data, typed_array_data_mut, typed_array_length,
};
use kahvi_core::prelude::*;

#[allow(unused_imports)] // for the tickets
pub(crate) use kahvi_core::object::typed_array::{
    clone_array_buffer, detach_array_buffer, is_typed_array_fixed_length, is_valid_integer_index,
    typed_array_byte_length, typed_array_get_element, typed_array_set_element,
};

/// The %TypedArray.prototype% intrinsic name for an element type, e.g. "%Int8Array.prototype%".
fn prototype_name(kind: ElementType) -> String {
    format!("%{}.prototype%", kind.name())
}

/// The constructor intrinsic for an element type, e.g. %Int8Array%.
pub(crate) fn constructor_of(agent: &mut Agent, kind: ElementType) -> Object {
    agent.intrinsic(&format!("%{}%", kind.name()))
}

/// The element type of a TypedArray, if `v` is one.
pub(crate) fn element_type(agent: &Agent, v: Value) -> Option<ElementType> {
    match v {
        Value::Object(o) => typed_array_data(agent, o).map(|d| d.kind),
        _ => None,
    }
}

/// The [[ViewedArrayBuffer]] of a TypedArray.
pub(crate) fn viewed_buffer(agent: &Agent, o: Object) -> Object {
    typed_array_data(agent, o)
        .and_then(|d| d.buffer)
        .expect("an initialized TypedArray")
}

/// [[ByteOffset]] of a TypedArray.
pub(crate) fn byte_offset(agent: &Agent, o: Object) -> usize {
    typed_array_data(agent, o)
        .expect("a TypedArray")
        .byte_offset
}

/// MakeTypedArrayWithBufferWitnessRecord(O, order)
pub(crate) fn ta_record(agent: &Agent, o: Object) -> TaRecord {
    make_ta_record(agent, o)
}

/// IsTypedArrayOutOfBounds(taRecord)
pub(crate) fn out_of_bounds(agent: &Agent, rec: TaRecord) -> bool {
    is_typed_array_out_of_bounds(agent, rec)
}

/// TypedArrayLength(taRecord)
pub(crate) fn length(agent: &Agent, rec: TaRecord) -> usize {
    typed_array_length(agent, rec)
}

/// ValidateTypedArray(O, order) (§23.2.4.4): the TypedArray With Buffer Witness Record.
pub(crate) fn validate_typed_array(agent: &mut Agent, o: Value) -> JsResult<TaRecord> {
    // 1. Perform ? RequireInternalSlot(O, [[TypedArrayName]]).
    let Value::Object(o) = o else {
        return Err(agent.throw_type_error("not a TypedArray"));
    };
    if typed_array_data(agent, o).is_none() {
        return Err(agent.throw_type_error("not a TypedArray"));
    }
    // 3. Return ? ValidateTypedArrayBounds(O, order): out of bounds (or detached) is a TypeError.
    let rec = make_ta_record(agent, o);
    if is_typed_array_out_of_bounds(agent, rec) {
        return Err(agent.throw_type_error("TypedArray is out of bounds or detached"));
    }
    Ok(rec)
}

/// GetArrayBufferMaxByteLengthOption(options) (§25.1.3.7)
pub(crate) fn get_array_buffer_max_byte_length_option(
    agent: &mut Agent,
    options: Value,
) -> JsResult<Option<f64>> {
    // 1. If options is not an Object, return empty.
    let Value::Object(o) = options else {
        return Ok(None);
    };
    // 2. Let maxByteLength be ? Get(options, "maxByteLength").
    let key = agent.key("maxByteLength");
    let max = ops::get(agent, o, key)?;
    // 3. If maxByteLength is undefined, return empty.
    if max.is_undefined() {
        return Ok(None);
    }
    // 4. Return ? ToIndex(maxByteLength).
    Ok(Some(ops::to_index(agent, max)?))
}

/// AllocateArrayBuffer(constructor, byteLength [, maxByteLength]) (§25.1.3.1)
pub(crate) fn allocate_buffer(
    agent: &mut Agent,
    ctor: Object,
    byte_length: f64,
    max: Option<f64>,
) -> JsResult<Object> {
    allocate_array_buffer(agent, ctor, byte_length, max)
}

// ---- Creating TypedArrays (§23.2.5.1) ---------------------------------------------------

/// AllocateTypedArray(constructorName, newTarget, defaultProto [, length]) (§23.2.5.1.1)
pub(crate) fn allocate_typed_array(
    agent: &mut Agent,
    kind: ElementType,
    new_target: Object,
    length: Option<f64>,
) -> JsResult<Object> {
    // 1. Let proto be ? GetPrototypeFromConstructor(newTarget, defaultProto).
    let proto = ops::get_prototype_from_constructor(agent, new_target, &prototype_name(kind))?;
    // 2. Let obj be TypedArrayCreate(proto).
    // 3-7. [[TypedArrayName]], [[ContentType]], [[ByteLength]] 0, [[ByteOffset]] 0, [[ArrayLength]] 0.
    let obj = agent.object_with_kind(
        Some(proto),
        ObjectKind::TypedArray(Box::new(TypedArrayData {
            kind,
            buffer: None,
            byte_offset: 0,
            byte_length: Some(0),
            array_length: Some(0),
        })),
    );
    // 8. If length is present, perform ? AllocateTypedArrayBuffer(obj, length).
    if let Some(len) = length {
        allocate_typed_array_buffer(agent, obj, len)?;
    }
    // 9. Return obj.
    Ok(obj)
}

/// AllocateTypedArrayBuffer(O, length) (§23.2.5.1.6)
pub(crate) fn allocate_typed_array_buffer(
    agent: &mut Agent,
    o: Object,
    length: f64,
) -> JsResult<()> {
    let kind = typed_array_data(agent, o).expect("a TypedArray").kind;
    // 2-3. Let byteLength be elementSize × length.
    let byte_length = kind.size() as f64 * length;
    // 4. Let data be ? AllocateArrayBuffer(%ArrayBuffer%, byteLength).
    let ctor = agent.intrinsic("%ArrayBuffer%");
    let data = allocate_array_buffer(agent, ctor, byte_length, None)?;
    // 5-8. Set [[ViewedArrayBuffer]], [[ByteLength]], [[ByteOffset]] and [[ArrayLength]].
    let d = typed_array_data_mut(agent, o).expect("a TypedArray");
    d.buffer = Some(data);
    d.byte_length = Some(byte_length as usize);
    d.byte_offset = 0;
    d.array_length = Some(length as usize);
    Ok(())
}

/// InitializeTypedArrayFromTypedArray(O, srcArray) (§23.2.5.1.2)
fn initialize_from_typed_array(agent: &mut Agent, o: Object, src: Object) -> JsResult<()> {
    let src_data = typed_array_data(agent, src).expect("a TypedArray");
    let kind = typed_array_data(agent, o).expect("a TypedArray").kind;
    // 7-8. Let srcRecord be MakeTypedArrayWithBufferWitnessRecord(srcArray, seq-cst);
    //      if IsTypedArrayOutOfBounds(srcRecord) is true, throw a TypeError exception.
    let rec = make_ta_record(agent, src);
    if is_typed_array_out_of_bounds(agent, rec) {
        return Err(agent.throw_type_error("source TypedArray is out of bounds or detached"));
    }
    // 9-10. Let elementLength be TypedArrayLength(srcRecord); byteLength be elementSize × elementLength.
    let element_length = typed_array_length(agent, rec);
    let byte_length = kind.size() * element_length;
    // 11. If elementType is srcType, then let data be ? CloneArrayBuffer(srcData, srcByteOffset, byteLength).
    let src_buffer = src_data.buffer.expect("initialized");
    let data = if src_data.kind == kind {
        clone_array_buffer(agent, src_buffer, src_data.byte_offset, byte_length)?
    } else {
        // 12.a. Let data be ? AllocateArrayBuffer(%ArrayBuffer%, byteLength).
        let ctor = agent.intrinsic("%ArrayBuffer%");
        let data = allocate_array_buffer(agent, ctor, byte_length as f64, None)?;
        // 12.b. If srcArray.[[ContentType]] is not O.[[ContentType]], throw a TypeError exception.
        if src_data.kind.is_bigint() != kind.is_bigint() {
            return Err(agent.throw_type_error("cannot mix BigInt and Number typed arrays"));
        }
        // 12.c-g. Copy each element, converting it.
        for i in 0..element_length {
            let v = get_value_from_buffer(
                agent,
                src_buffer,
                src_data.byte_offset + i * src_data.kind.size(),
                src_data.kind,
                true,
            );
            set_value_in_buffer(agent, data, i * kind.size(), kind, v, true);
        }
        data
    };
    // 13-16. Set O's [[ViewedArrayBuffer]], [[ByteLength]], [[ByteOffset]] and [[ArrayLength]].
    let d = typed_array_data_mut(agent, o).expect("a TypedArray");
    d.buffer = Some(data);
    d.byte_length = Some(byte_length);
    d.byte_offset = 0;
    d.array_length = Some(element_length);
    Ok(())
}

/// InitializeTypedArrayFromArrayBuffer(O, buffer, byteOffset, length) (§23.2.5.1.3)
fn initialize_from_array_buffer(
    agent: &mut Agent,
    o: Object,
    buffer: Object,
    byte_offset: Value,
    length: Value,
) -> JsResult<()> {
    let kind = typed_array_data(agent, o).expect("a TypedArray").kind;
    let element_size = kind.size();
    // 2. Let offset be ? ToIndex(byteOffset).
    let offset = ops::to_index(agent, byte_offset)?;
    // 3. If offset modulo elementSize ≠ 0, throw a RangeError exception.
    if offset % element_size as f64 != 0.0 {
        return Err(agent.throw_range_error("start offset must be a multiple of the element size"));
    }
    // 4. Let bufferIsFixedLength be IsFixedLengthArrayBuffer(buffer).
    let fixed = is_fixed_length_array_buffer(agent, buffer);
    // 5. If length is not undefined, let newLength be ? ToIndex(length).
    let new_length = if length.is_undefined() {
        None
    } else {
        Some(ops::to_index(agent, length)?)
    };
    // 6. If IsDetachedBuffer(buffer) is true, throw a TypeError exception.
    if is_detached_buffer(agent, buffer) {
        return Err(agent.throw_type_error("ArrayBuffer is detached"));
    }
    // 7. Let bufferByteLength be ArrayBufferByteLength(buffer, seq-cst).
    let buffer_byte_length = array_buffer_byte_length(agent, buffer) as f64;
    let (byte_length, array_length) = match new_length {
        // 8. If length is undefined and bufferIsFixedLength is false, then
        None if !fixed => {
            // a. If offset > bufferByteLength, throw a RangeError exception.
            if offset > buffer_byte_length {
                return Err(agent.throw_range_error("start offset is outside the buffer"));
            }
            // b-c. [[ByteLength]] and [[ArrayLength]] are auto.
            (None, None)
        }
        None => {
            // 9.a.i. If bufferByteLength modulo elementSize ≠ 0, throw a RangeError exception.
            if buffer_byte_length % element_size as f64 != 0.0 {
                return Err(
                    agent.throw_range_error("buffer length must be a multiple of the element size")
                );
            }
            // ii-iii. Let newByteLength be bufferByteLength - offset; if < 0, throw a RangeError exception.
            let new_byte_length = buffer_byte_length - offset;
            if new_byte_length < 0.0 {
                return Err(agent.throw_range_error("start offset is outside the buffer"));
            }
            (
                Some(new_byte_length as usize),
                Some(new_byte_length as usize / element_size),
            )
        }
        Some(n) => {
            // 9.b.i-ii. Let newByteLength be newLength × elementSize; if offset + newByteLength > bufferByteLength, throw a RangeError.
            let new_byte_length = n * element_size as f64;
            if offset + new_byte_length > buffer_byte_length {
                return Err(agent.throw_range_error("length is outside the buffer"));
            }
            (Some(new_byte_length as usize), Some(n as usize))
        }
    };
    // 10-12. Set [[ViewedArrayBuffer]], [[ByteOffset]] (and the lengths above).
    let d = typed_array_data_mut(agent, o).expect("a TypedArray");
    d.buffer = Some(buffer);
    d.byte_length = byte_length;
    d.byte_offset = offset as usize;
    d.array_length = array_length;
    Ok(())
}

/// InitializeTypedArrayFromList(O, values) (§23.2.5.1.4)
fn initialize_from_list(agent: &mut Agent, o: Object, values: Vec<Value>) -> JsResult<()> {
    // 1-2. Let len be the number of elements in values; perform ? AllocateTypedArrayBuffer(O, len).
    allocate_typed_array_buffer(agent, o, values.len() as f64)?;
    // 3-4. For each k, perform ? Set(O, Pk, kValue, true).
    for (k, v) in values.into_iter().enumerate() {
        let key = agent.key_from_index(k as f64);
        ops::set(agent, o, key, v, true)?;
    }
    Ok(())
}

/// InitializeTypedArrayFromArrayLike(O, arrayLike) (§23.2.5.1.5)
fn initialize_from_array_like(agent: &mut Agent, o: Object, array_like: Object) -> JsResult<()> {
    // 1. Let len be ? LengthOfArrayLike(arrayLike).
    let len = ops::length_of_array_like(agent, array_like)?;
    // 2. Perform ? AllocateTypedArrayBuffer(O, len).
    allocate_typed_array_buffer(agent, o, len)?;
    // 3-4. For each k: Perform ? Set(O, Pk, ? Get(arrayLike, Pk), true).
    let mut k = 0.0;
    while k < len {
        let key = agent.key_from_index(k);
        let v = ops::get(agent, array_like, key)?;
        ops::set(agent, o, key, v, true)?;
        k += 1.0;
    }
    Ok(())
}

/// The TypedArray constructors, _TypedArray_(...args) (§23.2.5.1)
pub(crate) fn typed_array_constructor(
    agent: &mut Agent,
    args: &[Value],
    new_target: Option<Object>,
    kind: ElementType,
) -> JsResult<Value> {
    // 1. If NewTarget is undefined, throw a TypeError exception.
    let Some(new_target) = new_target else {
        let msg = format!("Constructor {} requires 'new'", kind.name());
        return Err(agent.throw_type_error(&msg));
    };
    // 4-5. If numberOfArgs = 0, return ? AllocateTypedArray(constructorName, NewTarget, proto, 0).
    let Some(&first) = args.first() else {
        return Ok(Value::Object(allocate_typed_array(
            agent,
            kind,
            new_target,
            Some(0.0),
        )?));
    };
    // 6.b. If firstArgument is an Object, then
    if let Value::Object(first) = first {
        // i. Let O be ? AllocateTypedArray(constructorName, NewTarget, proto).
        let o = allocate_typed_array(agent, kind, new_target, None)?;
        if typed_array_data(agent, first).is_some() {
            // ii. If firstArgument has a [[TypedArrayName]] internal slot, InitializeTypedArrayFromTypedArray.
            initialize_from_typed_array(agent, o, first)?;
        } else if array_buffer_data(agent, first).is_some() {
            // iii. Else if firstArgument has an [[ArrayBufferData]] internal slot, InitializeTypedArrayFromArrayBuffer.
            initialize_from_array_buffer(
                agent,
                o,
                first,
                args.get_or_undefined(1),
                args.get_or_undefined(2),
            )?;
        } else {
            // iv.1-2. Let usingIterator be ? GetMethod(firstArgument, %Symbol.iterator%).
            let iterator_key = PropertyKey::Symbol(agent.symbols_wk.iterator);
            let using_iterator = ops::get_method(agent, Value::Object(first), iterator_key)?;
            if !using_iterator.is_undefined() {
                // 3. Let values be ? IteratorToList(? GetIteratorFromMethod(firstArgument, usingIterator)).
                let mut record =
                    ops::get_iterator_from_method(agent, Value::Object(first), using_iterator)?;
                let values = ops::iterator_to_list(agent, &mut record)?;
                // 4. Perform ? InitializeTypedArrayFromList(O, values).
                initialize_from_list(agent, o, values)?;
            } else {
                // 5. Else, perform ? InitializeTypedArrayFromArrayLike(O, firstArgument).
                initialize_from_array_like(agent, o, first)?;
            }
        }
        // v. Return O.
        return Ok(Value::Object(o));
    }
    // 6.c.i-ii. Let elementLength be ? ToIndex(firstArgument).
    let element_length = ops::to_index(agent, first)?;
    // iii. Return ? AllocateTypedArray(constructorName, NewTarget, proto, elementLength).
    Ok(Value::Object(allocate_typed_array(
        agent,
        kind,
        new_target,
        Some(element_length),
    )?))
}

/// TypedArrayCreateFromConstructor(constructor, argumentList) (§23.2.4.2)
pub(crate) fn typed_array_create_from_constructor(
    agent: &mut Agent,
    ctor: Object,
    args: &[Value],
) -> JsResult<Object> {
    // 1. Let newTypedArray be ? Construct(constructor, argumentList).
    let ta = ops::construct(agent, ctor, args, None)?;
    // 2. Let taRecord be ? ValidateTypedArray(newTypedArray, seq-cst).
    let rec = validate_typed_array(agent, Value::Object(ta))?;
    // 4. If the number of elements in argumentList is 1 and argumentList[0] is a Number, then
    if let [Value::Number(n)] = args {
        // a. If IsTypedArrayOutOfBounds(taRecord) is true, throw a TypeError exception.
        // b-c. If TypedArrayLength(taRecord) < ℝ(argumentList[0]), throw a TypeError exception.
        if (typed_array_length(agent, rec) as f64) < *n {
            return Err(agent.throw_type_error("TypedArray is too short"));
        }
    }
    // 5. Return newTypedArray.
    Ok(rec.object)
}

/// TypedArraySpeciesCreate(exemplar, argumentList) (§23.2.4.1)
pub(crate) fn typed_array_species_create(
    agent: &mut Agent,
    exemplar: Object,
    args: &[Value],
) -> JsResult<Object> {
    let kind = typed_array_data(agent, exemplar)
        .expect("a TypedArray")
        .kind;
    // 1. Let defaultConstructor be the intrinsic object associated with exemplar.[[TypedArrayName]].
    let default = constructor_of(agent, kind);
    // 2. Let constructor be ? SpeciesConstructor(exemplar, defaultConstructor).
    let ctor = ops::species_constructor(agent, exemplar, default)?;
    // 3. Let result be ? TypedArrayCreateFromConstructor(constructor, argumentList).
    let result = typed_array_create_from_constructor(agent, ctor, args)?;
    // 4. If result.[[ContentType]] is not exemplar.[[ContentType]], throw a TypeError exception.
    if typed_array_data(agent, result)
        .expect("a TypedArray")
        .kind
        .is_bigint()
        != kind.is_bigint()
    {
        return Err(agent.throw_type_error("species constructor made the wrong kind of TypedArray"));
    }
    // 5. Return result.
    Ok(result)
}

/// TypedArrayCreateSameType(exemplar, length) (§23.2.4.3)
pub(crate) fn typed_array_create_same_type(
    agent: &mut Agent,
    exemplar: Object,
    length: f64,
) -> JsResult<Object> {
    let kind = typed_array_data(agent, exemplar)
        .expect("a TypedArray")
        .kind;
    // 1. Let constructor be the intrinsic object associated with exemplar.[[TypedArrayName]].
    let ctor = constructor_of(agent, kind);
    // 2. Let result be ? TypedArrayCreateFromConstructor(constructor, « 𝔽(length) »).
    typed_array_create_from_constructor(agent, ctor, &[Value::Number(length)])
}

// ---- DataView (§25.3.1) -------------------------------------------------------------

/// RequireInternalSlot(view, [[DataView]]): the view's slots.
pub(crate) fn require_data_view(
    agent: &mut Agent,
    view: Value,
) -> JsResult<(Object, DataViewData)> {
    match view {
        Value::Object(o) => match data_view_data(agent, o) {
            Some(d) => Ok((o, d)),
            None => Err(agent.throw_type_error("not a DataView")),
        },
        _ => Err(agent.throw_type_error("not a DataView")),
    }
}

/// IsViewOutOfBounds(MakeDataViewWithBufferWitnessRecord(view, order)) (§25.3.1.3)
pub(crate) fn is_view_out_of_bounds(agent: &Agent, d: DataViewData) -> bool {
    // 3. If IsDetachedBuffer(view.[[ViewedArrayBuffer]]) is true, return true.
    if is_detached_buffer(agent, d.buffer) {
        return true;
    }
    let buffer_byte_length = array_buffer_byte_length(agent, d.buffer);
    // 5-7. byteOffsetStart and byteOffsetEnd.
    let start = d.byte_offset;
    let end = match d.byte_length {
        None => buffer_byte_length,
        Some(len) => start + len,
    };
    // 9. If byteOffsetStart > bufferByteLength or byteOffsetEnd > bufferByteLength, return true.
    start > buffer_byte_length || end > buffer_byte_length
}

/// GetViewByteLength(viewRecord) (§25.3.1.2)
pub(crate) fn get_view_byte_length(agent: &Agent, d: DataViewData) -> usize {
    // 3. If view.[[ByteLength]] is not auto, return view.[[ByteLength]].
    if let Some(len) = d.byte_length {
        return len;
    }
    // 5-8. Return byteLength - byteOffset.
    array_buffer_byte_length(agent, d.buffer) - d.byte_offset
}

/// GetViewValue(view, requestIndex, isLittleEndian, type) (§25.3.1.5)
pub(crate) fn get_view_value(
    agent: &mut Agent,
    view: Value,
    request_index: Value,
    little_endian: Value,
    ty: ElementType,
) -> JsResult<Value> {
    // 1-2. Perform ? RequireInternalSlot(view, [[DataView]]).
    let (o, _) = require_data_view(agent, view)?;
    // 3. Let getIndex be ? ToIndex(requestIndex).
    let get_index = ops::to_index(agent, request_index)?;
    // 4. Set isLittleEndian to ToBoolean(isLittleEndian).
    let little_endian = ops::to_boolean(agent, little_endian);
    // 5-8. If IsViewOutOfBounds(viewRecord) is true, throw a TypeError exception.
    let d = data_view_data(agent, o).expect("a DataView");
    if is_view_out_of_bounds(agent, d) {
        return Err(agent.throw_type_error("DataView is out of bounds or detached"));
    }
    // 9-11. If getIndex + elementSize > viewSize, throw a RangeError exception.
    let view_size = get_view_byte_length(agent, d);
    if get_index + ty.size() as f64 > view_size as f64 {
        return Err(agent.throw_range_error("offset is outside the bounds of the DataView"));
    }
    // 12-13. Return GetValueFromBuffer(view.[[ViewedArrayBuffer]], getIndex + viewOffset, type, false, unordered, isLittleEndian).
    let index = get_index as usize + d.byte_offset;
    Ok(get_value_from_buffer(
        agent,
        d.buffer,
        index,
        ty,
        little_endian,
    ))
}

/// SetViewValue(view, requestIndex, isLittleEndian, type, value) (§25.3.1.6)
pub(crate) fn set_view_value(
    agent: &mut Agent,
    view: Value,
    request_index: Value,
    little_endian: Value,
    ty: ElementType,
    value: Value,
) -> JsResult<Value> {
    // 1-2. Perform ? RequireInternalSlot(view, [[DataView]]).
    let (o, _) = require_data_view(agent, view)?;
    // 3. Let getIndex be ? ToIndex(requestIndex).
    let get_index = ops::to_index(agent, request_index)?;
    // 4-5. If IsBigIntElementType(type) is true, let numberValue be ? ToBigInt(value); else ? ToNumber(value).
    let number = if ty.is_bigint() {
        Value::BigInt(ops::bigint::to_bigint(agent, value)?)
    } else {
        Value::Number(ops::to_number(agent, value)?)
    };
    // 6. Set isLittleEndian to ToBoolean(isLittleEndian).
    let little_endian = ops::to_boolean(agent, little_endian);
    // 7-10. If IsViewOutOfBounds(viewRecord) is true, throw a TypeError exception.
    let d = data_view_data(agent, o).expect("a DataView");
    if is_view_out_of_bounds(agent, d) {
        return Err(agent.throw_type_error("DataView is out of bounds or detached"));
    }
    // 11-13. If getIndex + elementSize > viewSize, throw a RangeError exception.
    let view_size = get_view_byte_length(agent, d);
    if get_index + ty.size() as f64 > view_size as f64 {
        return Err(agent.throw_range_error("offset is outside the bounds of the DataView"));
    }
    // 14-15. Perform SetValueInBuffer(view.[[ViewedArrayBuffer]], getIndex + viewOffset, type, numberValue, false, unordered, isLittleEndian).
    let index = get_index as usize + d.byte_offset;
    set_value_in_buffer(agent, d.buffer, index, ty, number, little_endian);
    // 16. Return undefined.
    Ok(Value::Undefined)
}

/// The DataView constructor (§25.3.2.1)
pub(crate) fn data_view_constructor(
    agent: &mut Agent,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let (buffer, byte_offset, byte_length) = (
        args.get_or_undefined(0),
        args.get_or_undefined(1),
        args.get_or_undefined(2),
    );
    // 1. If NewTarget is undefined, throw a TypeError exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor DataView requires 'new'"));
    };
    // 2. Perform ? RequireInternalSlot(buffer, [[ArrayBufferData]]).
    let buffer = match buffer {
        Value::Object(b) if array_buffer_data(agent, b).is_some() => b,
        _ => return Err(agent.throw_type_error("DataView needs an ArrayBuffer")),
    };
    // 3. Let offset be ? ToIndex(byteOffset).
    let offset = ops::to_index(agent, byte_offset)?;
    // 4. If IsDetachedBuffer(buffer) is true, throw a TypeError exception.
    if is_detached_buffer(agent, buffer) {
        return Err(agent.throw_type_error("ArrayBuffer is detached"));
    }
    // 5-6. If offset > bufferByteLength, throw a RangeError exception.
    let buffer_byte_length = array_buffer_byte_length(agent, buffer) as f64;
    if offset > buffer_byte_length {
        return Err(agent.throw_range_error("start offset is outside the buffer"));
    }
    // 7. Let bufferIsFixedLength be IsFixedLengthArrayBuffer(buffer).
    let fixed = is_fixed_length_array_buffer(agent, buffer);
    // 8-9. The view byte length.
    let view_byte_length = if byte_length.is_undefined() {
        if fixed {
            Some(buffer_byte_length - offset)
        } else {
            None
        }
    } else {
        let len = ops::to_index(agent, byte_length)?;
        if offset + len > buffer_byte_length {
            return Err(agent.throw_range_error("length is outside the buffer"));
        }
        Some(len)
    };
    // 10. Let O be ? OrdinaryCreateFromConstructor(NewTarget, "%DataView.prototype%", slots).
    let o = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%DataView.prototype%",
        ObjectKind::DataView(Box::new(DataViewData {
            buffer,
            byte_length: view_byte_length.map(|l| l as usize),
            byte_offset: offset as usize,
        })),
    )?;
    // 11. If IsDetachedBuffer(buffer) is true, throw a TypeError exception.
    if is_detached_buffer(agent, buffer) {
        return Err(agent.throw_type_error("ArrayBuffer is detached"));
    }
    // 12-13. If offset > bufferByteLength (after a possible resize), throw a RangeError exception.
    let buffer_byte_length = array_buffer_byte_length(agent, buffer) as f64;
    if offset > buffer_byte_length {
        return Err(agent.throw_range_error("start offset is outside the buffer"));
    }
    // 14. If byteLength is not undefined, then if offset + viewByteLength > bufferByteLength, throw a RangeError exception.
    if !byte_length.is_undefined()
        && let Some(len) = view_byte_length
        && offset + len > buffer_byte_length
    {
        return Err(agent.throw_range_error("length is outside the buffer"));
    }
    // 15-18. Return O.
    Ok(Value::Object(o))
}
