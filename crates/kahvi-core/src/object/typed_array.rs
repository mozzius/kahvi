//! ArrayBuffer data blocks (§25.1), TypedArray exotic objects (§10.4.5,
//! §23.2) and DataView views (§25.3): the internal slots, the buffer
//! abstract operations and the exotic internal methods. The built-in
//! functions are in kahvi-builtins.

use super::{ObjectKind, PropertyDescriptor, PropertyKey};
use crate::agent::Agent;
use crate::error::JsResult;
use crate::ops;
use crate::value::{Object, Value};

/// The largest buffer the engine allocates (CreateByteDataBlock throws a
/// RangeError beyond it).
pub const MAX_BYTE_LENGTH: f64 = (1u64 << 32) as f64;

/// [[ArrayBufferData]], [[ArrayBufferByteLength]], [[ArrayBufferMaxByteLength]]
/// and [[ArrayBufferDetachKey]].
#[derive(Debug)]
pub struct ArrayBufferData {
    /// The data block; `None` once detached.
    pub data: Option<DataBlock>,
    /// The maximum byte length of a resizable buffer.
    pub max_byte_length: Option<usize>,
    pub detach_key: Value,
    /// The data block is a Shared Data Block (a SharedArrayBuffer). With one
    /// agent, a Shared Data Block holds bytes like any Data Block; it is
    /// never detached. A growable one's [[ArrayBufferByteLengthData]] is the
    /// data's length, and [[ArrayBufferMaxByteLength]] is `max_byte_length`.
    pub shared: bool,
}

/// A Data Block (§6.2.9): bytes the engine owns, or memory an embedder
/// lent it (JSI's `createArrayBuffer`), released when the block is dropped
/// (the buffer is detached or collected).
pub enum DataBlock {
    Owned(Vec<u8>),
    External(ExternalBlock),
}

/// Embedder memory backing an ArrayBuffer.
pub struct ExternalBlock {
    ptr: *mut u8,
    len: usize,
    release: Option<Box<dyn FnOnce() + Send>>,
}

impl ExternalBlock {
    /// # Safety
    /// `ptr` must be valid for reads and writes of `len` bytes, and not be
    /// used by anything else as a Rust reference, until `release` runs; and
    /// usable from whichever thread runs the agent (the agent may move
    /// between threads, never run on two).
    pub unsafe fn new(
        ptr: *mut u8,
        len: usize,
        release: Box<dyn FnOnce() + Send>,
    ) -> ExternalBlock {
        ExternalBlock {
            ptr,
            len,
            release: Some(release),
        }
    }
}

// SAFETY: ExternalBlock::new's contract: the memory goes with the agent.
unsafe impl Send for ExternalBlock {}

impl Drop for ExternalBlock {
    fn drop(&mut self) {
        if let Some(release) = self.release.take() {
            release();
        }
    }
}

impl std::fmt::Debug for DataBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataBlock::Owned(v) => write!(f, "DataBlock({} bytes)", v.len()),
            DataBlock::External(e) => write!(f, "DataBlock({} external bytes)", e.len),
        }
    }
}

impl From<Vec<u8>> for DataBlock {
    fn from(v: Vec<u8>) -> DataBlock {
        DataBlock::Owned(v)
    }
}

impl Clone for DataBlock {
    /// A copy of the bytes, owned by the engine.
    fn clone(&self) -> DataBlock {
        DataBlock::Owned(self.to_vec())
    }
}

impl std::ops::Deref for DataBlock {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        match self {
            DataBlock::Owned(v) => v,
            // SAFETY: ExternalBlock::new's contract.
            DataBlock::External(e) if e.len > 0 => unsafe {
                std::slice::from_raw_parts(e.ptr, e.len)
            },
            DataBlock::External(_) => &[],
        }
    }
}

impl std::ops::DerefMut for DataBlock {
    fn deref_mut(&mut self) -> &mut [u8] {
        match self {
            DataBlock::Owned(v) => v,
            // SAFETY: ExternalBlock::new's contract.
            DataBlock::External(e) if e.len > 0 => unsafe {
                std::slice::from_raw_parts_mut(e.ptr, e.len)
            },
            DataBlock::External(_) => &mut [],
        }
    }
}

impl DataBlock {
    /// Resizes the block in place (a resizable ArrayBuffer or growable
    /// SharedArrayBuffer; external blocks are neither, and would be copied).
    pub fn resize(&mut self, len: usize, value: u8) {
        if let DataBlock::External(_) = self {
            *self = DataBlock::Owned(self.to_vec());
        }
        if let DataBlock::Owned(v) = self {
            v.resize(len, value);
        }
    }
}

/// An Element Type (Table 71).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElementType {
    Int8,
    Uint8,
    Uint8C,
    Int16,
    Uint16,
    Int32,
    Uint32,
    BigInt64,
    BigUint64,
    Float16,
    Float32,
    Float64,
}

impl ElementType {
    /// The Element Size.
    pub fn size(self) -> usize {
        match self {
            ElementType::Int8 | ElementType::Uint8 | ElementType::Uint8C => 1,
            ElementType::Int16 | ElementType::Uint16 | ElementType::Float16 => 2,
            ElementType::Int32 | ElementType::Uint32 | ElementType::Float32 => 4,
            ElementType::BigInt64 | ElementType::BigUint64 | ElementType::Float64 => 8,
        }
    }

    /// IsBigIntElementType(type); a TypedArray's [[ContentType]] is bigint for these.
    pub fn is_bigint(self) -> bool {
        matches!(self, ElementType::BigInt64 | ElementType::BigUint64)
    }

    /// The TypedArray constructor name ([[TypedArrayName]]).
    pub fn name(self) -> &'static str {
        match self {
            ElementType::Int8 => "Int8Array",
            ElementType::Uint8 => "Uint8Array",
            ElementType::Uint8C => "Uint8ClampedArray",
            ElementType::Int16 => "Int16Array",
            ElementType::Uint16 => "Uint16Array",
            ElementType::Int32 => "Int32Array",
            ElementType::Uint32 => "Uint32Array",
            ElementType::BigInt64 => "BigInt64Array",
            ElementType::BigUint64 => "BigUint64Array",
            ElementType::Float16 => "Float16Array",
            ElementType::Float32 => "Float32Array",
            ElementType::Float64 => "Float64Array",
        }
    }

    pub fn from_name(name: &str) -> Option<ElementType> {
        ALL_ELEMENT_TYPES.iter().copied().find(|t| t.name() == name)
    }
}

pub const ALL_ELEMENT_TYPES: [ElementType; 12] = [
    ElementType::Int8,
    ElementType::Uint8,
    ElementType::Uint8C,
    ElementType::Int16,
    ElementType::Uint16,
    ElementType::Int32,
    ElementType::Uint32,
    ElementType::BigInt64,
    ElementType::BigUint64,
    ElementType::Float16,
    ElementType::Float32,
    ElementType::Float64,
];

/// A TypedArray's internal slots.
#[derive(Debug, Clone, Copy)]
pub struct TypedArrayData {
    /// [[TypedArrayName]] / [[ContentType]], as the element type.
    pub kind: ElementType,
    /// [[ViewedArrayBuffer]]; `None` only while AllocateTypedArray runs.
    pub buffer: Option<Object>,
    pub byte_offset: usize,
    /// [[ByteLength]]; `None` is ~auto~.
    pub byte_length: Option<usize>,
    /// [[ArrayLength]]; `None` is ~auto~.
    pub array_length: Option<usize>,
}

/// A DataView's internal slots.
#[derive(Debug, Clone, Copy)]
pub struct DataViewData {
    /// [[ViewedArrayBuffer]]
    pub buffer: Object,
    /// [[ByteLength]]; `None` is ~auto~.
    pub byte_length: Option<usize>,
    pub byte_offset: usize,
}

pub fn array_buffer_data(agent: &Agent, o: Object) -> Option<&ArrayBufferData> {
    match &agent.obj(o).kind {
        ObjectKind::ArrayBuffer(d) => Some(d),
        _ => None,
    }
}

pub fn array_buffer_data_mut(agent: &mut Agent, o: Object) -> Option<&mut ArrayBufferData> {
    match &mut agent.obj_mut(o).kind {
        ObjectKind::ArrayBuffer(d) => Some(d),
        _ => None,
    }
}

pub fn typed_array_data(agent: &Agent, o: Object) -> Option<TypedArrayData> {
    match &agent.obj(o).kind {
        ObjectKind::TypedArray(d) => Some(**d),
        _ => None,
    }
}

pub fn typed_array_data_mut(agent: &mut Agent, o: Object) -> Option<&mut TypedArrayData> {
    match &mut agent.obj_mut(o).kind {
        ObjectKind::TypedArray(d) => Some(d),
        _ => None,
    }
}

pub fn data_view_data(agent: &Agent, o: Object) -> Option<DataViewData> {
    match &agent.obj(o).kind {
        ObjectKind::DataView(d) => Some(**d),
        _ => None,
    }
}

// ---- ArrayBuffer abstract operations (§25.1.3) --------------------------------------

/// AllocateArrayBuffer(constructor, byteLength [, maxByteLength]) (§25.1.3.1)
pub fn allocate_array_buffer(
    agent: &mut Agent,
    constructor: Object,
    byte_length: f64,
    max_byte_length: Option<f64>,
) -> JsResult<Object> {
    // 3. If allocatingResizableBuffer is true and byteLength > maxByteLength, throw a RangeError exception.
    if let Some(max) = max_byte_length
        && byte_length > max
    {
        return Err(agent.throw_range_error("byte length exceeds the maximum byte length"));
    }
    // 4. Let obj be ? OrdinaryCreateFromConstructor(constructor, "%ArrayBuffer.prototype%", slots).
    let obj = ops::ordinary_create_from_constructor(
        agent,
        constructor,
        "%ArrayBuffer.prototype%",
        ObjectKind::ArrayBuffer(Box::new(ArrayBufferData {
            data: Some(DataBlock::Owned(Vec::new())),
            max_byte_length: None,
            detach_key: Value::Undefined,
            shared: false,
        })),
    )?;
    // 5. Let block be ? CreateByteDataBlock(byteLength).
    let block = create_byte_data_block(agent, byte_length)?;
    // 8. If allocatingResizableBuffer is true: if it is not possible to create a Data Block of
    //    maxByteLength bytes, throw a RangeError exception.
    let max = match max_byte_length {
        Some(m) if m > MAX_BYTE_LENGTH => {
            return Err(agent.throw_range_error("maximum byte length too large"));
        }
        Some(m) => Some(m as usize),
        None => None,
    };
    let data = array_buffer_data_mut(agent, obj).expect("an ArrayBuffer");
    // 6-7. Set obj.[[ArrayBufferData]] to block and obj.[[ArrayBufferByteLength]] to byteLength.
    data.data = Some(block.into());
    // 8.b. Set obj.[[ArrayBufferMaxByteLength]] to maxByteLength.
    data.max_byte_length = max;
    // 9. Return obj.
    Ok(obj)
}

/// AllocateSharedArrayBuffer(ctor, byteLength [, maxByteLength])
pub fn allocate_shared_array_buffer(
    agent: &mut Agent,
    ctor: Object,
    byte_length: f64,
    max_byte_length: Option<f64>,
) -> JsResult<Object> {
    // 2-3. If allocatingGrowableBuffer is true and byteLength > maxByteLength, throw a RangeError exception.
    if let Some(max) = max_byte_length
        && byte_length > max
    {
        return Err(agent.throw_range_error("byte length exceeds the maximum byte length"));
    }
    // 5. Let obj be ? OrdinaryCreateFromConstructor(ctor, "%SharedArrayBuffer.prototype%", slots).
    let obj = ops::ordinary_create_from_constructor(
        agent,
        ctor,
        "%SharedArrayBuffer.prototype%",
        ObjectKind::ArrayBuffer(Box::new(ArrayBufferData {
            data: Some(DataBlock::Owned(Vec::new())),
            max_byte_length: None,
            detach_key: Value::Undefined,
            shared: true,
        })),
    )?;
    // 6. If allocatingGrowableBuffer is true, let allocationLength be maxByteLength; else byteLength.
    // 7. Let block be ? CreateSharedByteDataBlock(allocationLength).
    //    (The block holds byteLength bytes and grows in place up to maxByteLength.)
    let allocation_length = max_byte_length.unwrap_or(byte_length);
    if allocation_length > MAX_BYTE_LENGTH {
        return Err(agent.throw_range_error("shared array buffer allocation failed"));
    }
    let block = create_byte_data_block(agent, byte_length)?;
    let data = array_buffer_data_mut(agent, obj).expect("a SharedArrayBuffer");
    // 8. Set obj.[[ArrayBufferData]] to block.
    // 9.b-d / 10.a. [[ArrayBufferByteLengthData]] or [[ArrayBufferByteLength]] is byteLength.
    data.data = Some(block.into());
    // 9.e. Set obj.[[ArrayBufferMaxByteLength]] to maxByteLength.
    data.max_byte_length = max_byte_length.map(|m| m as usize);
    // 11. Return obj.
    Ok(obj)
}

/// IsSharedArrayBuffer(obj): obj.[[ArrayBufferData]] is a Shared Data Block.
pub fn is_shared_array_buffer(agent: &Agent, obj: Object) -> bool {
    array_buffer_data(agent, obj).is_some_and(|d| d.shared)
}

/// CreateByteDataBlock(size) (§6.2.9.1)
pub fn create_byte_data_block(agent: &mut Agent, size: f64) -> JsResult<Vec<u8>> {
    // 1. If size > 2^53 - 1, throw a RangeError exception.
    // 2-3. If it is impossible to create such a Data Block, throw a RangeError exception.
    if size > MAX_BYTE_LENGTH {
        return Err(agent.throw_range_error("array buffer allocation failed"));
    }
    // 4. Set all of the bytes of db to 0.
    Ok(vec![0; size as usize])
}

/// IsDetachedBuffer(arrayBuffer) (§25.1.3.4)
pub fn is_detached_buffer(agent: &Agent, buffer: Object) -> bool {
    array_buffer_data(agent, buffer).is_some_and(|d| d.data.is_none())
}

/// DetachArrayBuffer(arrayBuffer [, key]) (§25.1.3.5)
pub fn detach_array_buffer(agent: &mut Agent, buffer: Object, key: Value) -> JsResult<()> {
    // 1. Assert: IsSharedArrayBuffer(arrayBuffer) is false. (A host can ask:
    //    a Shared Data Block is never detached, so that throws a TypeError.)
    if is_shared_array_buffer(agent, buffer) {
        return Err(agent.throw_type_error("a SharedArrayBuffer cannot be detached"));
    }
    // 3. If SameValue(arrayBuffer.[[ArrayBufferDetachKey]], key) is false, throw a TypeError exception.
    let detach_key = array_buffer_data(agent, buffer)
        .expect("an ArrayBuffer")
        .detach_key;
    if !ops::same_value(agent, detach_key, key) {
        return Err(agent.throw_type_error("ArrayBuffer detach key mismatch"));
    }
    // 4-5. Set [[ArrayBufferData]] to null and [[ArrayBufferByteLength]] to 0.
    array_buffer_data_mut(agent, buffer)
        .expect("an ArrayBuffer")
        .data = None;
    Ok(())
}

/// ArrayBufferByteLength(arrayBuffer, order) (§25.1.3.?): 0 once detached.
pub fn array_buffer_byte_length(agent: &Agent, buffer: Object) -> usize {
    array_buffer_data(agent, buffer)
        .and_then(|d| d.data.as_ref())
        .map_or(0, |d| d.len())
}

/// IsFixedLengthArrayBuffer(arrayBuffer) (§25.1.3.?)
pub fn is_fixed_length_array_buffer(agent: &Agent, buffer: Object) -> bool {
    array_buffer_data(agent, buffer).is_none_or(|d| d.max_byte_length.is_none())
}

/// CloneArrayBuffer(srcBuffer, srcByteOffset, srcLength) (§25.1.3.6)
pub fn clone_array_buffer(
    agent: &mut Agent,
    src: Object,
    offset: usize,
    length: usize,
) -> JsResult<Object> {
    // 2. Let targetBuffer be ? AllocateArrayBuffer(%ArrayBuffer%, srcLength).
    let ctor = agent.intrinsic("%ArrayBuffer%");
    let target = allocate_array_buffer(agent, ctor, length as f64, None)?;
    // 3-6. Copy srcLength bytes from srcBlock at srcByteOffset.
    let bytes = array_buffer_data(agent, src)
        .and_then(|d| d.data.as_ref())
        .expect("not detached")[offset..offset + length]
        .to_vec();
    array_buffer_data_mut(agent, target)
        .expect("an ArrayBuffer")
        .data = Some(bytes.into());
    Ok(target)
}

/// The bytes for `value` (already a Number or BigInt of the right type): NumericToRawBytes(type, value, isLittleEndian) (§25.1.3.17)
pub fn numeric_to_raw_bytes(
    agent: &Agent,
    ty: ElementType,
    value: Value,
    little_endian: bool,
) -> Vec<u8> {
    let mut bytes = match (ty, value) {
        (ElementType::Float16, Value::Number(n)) => f64_to_f16_bits(n).to_le_bytes().to_vec(),
        (ElementType::Float32, Value::Number(n)) => (n as f32).to_le_bytes().to_vec(),
        (ElementType::Float64, Value::Number(n)) => n.to_le_bytes().to_vec(),
        (ElementType::Int8, Value::Number(n)) => vec![ops::f64_to_int32(n) as i8 as u8],
        (ElementType::Uint8, Value::Number(n)) => vec![ops::f64_to_int32(n) as u8],
        (ElementType::Uint8C, Value::Number(n)) => vec![uint8_clamp(n)],
        (ElementType::Int16, Value::Number(n)) => {
            (ops::f64_to_int32(n) as i16).to_le_bytes().to_vec()
        }
        (ElementType::Uint16, Value::Number(n)) => {
            (ops::f64_to_int32(n) as u16).to_le_bytes().to_vec()
        }
        (ElementType::Int32, Value::Number(n)) => ops::f64_to_int32(n).to_le_bytes().to_vec(),
        (ElementType::Uint32, Value::Number(n)) => ops::f64_to_uint32(n).to_le_bytes().to_vec(),
        (ElementType::BigInt64 | ElementType::BigUint64, Value::BigInt(b)) => {
            agent.bigint(b).low_u64().to_le_bytes().to_vec()
        }
        _ => unreachable!("NumericToRawBytes gets a value of the element type's content type"),
    };
    if !little_endian {
        bytes.reverse();
    }
    bytes
}

/// ToUint8Clamp on a Number (§7.1.12)
fn uint8_clamp(n: f64) -> u8 {
    if n.is_nan() || n <= 0.0 {
        return 0;
    }
    if n >= 255.0 {
        return 255;
    }
    let f = n.floor();
    if f + 0.5 < n {
        return (f + 1.0) as u8;
    }
    if n < f + 0.5 {
        return f as u8;
    }
    // Exactly halfway: round to even.
    if (f as u8) % 2 == 1 {
        (f + 1.0) as u8
    } else {
        f as u8
    }
}

/// RawBytesToNumeric(type, rawBytes, isLittleEndian) (§25.1.3.15)
pub fn raw_bytes_to_numeric(
    agent: &mut Agent,
    ty: ElementType,
    raw: &[u8],
    little_endian: bool,
) -> Value {
    let mut b = raw.to_vec();
    if !little_endian {
        b.reverse();
    }
    let n = match ty {
        ElementType::Float16 => f16_bits_to_f64(u16::from_le_bytes([b[0], b[1]])),
        ElementType::Float32 => f32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64,
        ElementType::Float64 => f64::from_le_bytes(b[..8].try_into().expect("8 bytes")),
        ElementType::Int8 => b[0] as i8 as f64,
        ElementType::Uint8 | ElementType::Uint8C => b[0] as f64,
        ElementType::Int16 => i16::from_le_bytes([b[0], b[1]]) as f64,
        ElementType::Uint16 => u16::from_le_bytes([b[0], b[1]]) as f64,
        ElementType::Int32 => i32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64,
        ElementType::Uint32 => u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64,
        ElementType::BigInt64 => {
            let v = i64::from_le_bytes(b[..8].try_into().expect("8 bytes"));
            return Value::BigInt(ops::bigint::from_i128(agent, v as i128));
        }
        ElementType::BigUint64 => {
            let v = u64::from_le_bytes(b[..8].try_into().expect("8 bytes"));
            return Value::BigInt(ops::bigint::from_i128(agent, v as i128));
        }
    };
    Value::Number(n)
}

/// GetValueFromBuffer(arrayBuffer, byteIndex, type, isTypedArray, order [, isLittleEndian]) (§25.1.3.16)
pub fn get_value_from_buffer(
    agent: &mut Agent,
    buffer: Object,
    byte_index: usize,
    ty: ElementType,
    little_endian: bool,
) -> Value {
    // 1. Assert: IsDetachedBuffer(arrayBuffer) is false.
    // 5-6. Let rawValue be the elementSize bytes of block at byteIndex.
    let raw = array_buffer_data(agent, buffer)
        .and_then(|d| d.data.as_ref())
        .expect("not detached")[byte_index..byte_index + ty.size()]
        .to_vec();
    // 9. Return RawBytesToNumeric(type, rawValue, isLittleEndian).
    raw_bytes_to_numeric(agent, ty, &raw, little_endian)
}

/// SetValueInBuffer(arrayBuffer, byteIndex, type, value, isTypedArray, order [, isLittleEndian]) (§25.1.3.18)
pub fn set_value_in_buffer(
    agent: &mut Agent,
    buffer: Object,
    byte_index: usize,
    ty: ElementType,
    value: Value,
    little_endian: bool,
) {
    // 8. Let rawBytes be NumericToRawBytes(type, value, isLittleEndian).
    let bytes = numeric_to_raw_bytes(agent, ty, value, little_endian);
    // 9-10. Store the bytes in block at byteIndex.
    let data = array_buffer_data_mut(agent, buffer)
        .and_then(|d| d.data.as_mut())
        .expect("not detached");
    data[byte_index..byte_index + bytes.len()].copy_from_slice(&bytes);
}

// ---- Float16 -------------------------------------------------------------------

/// The IEEE 754-2019 binary16 value nearest `n` (roundTiesToEven), as bits.
pub fn f64_to_f16_bits(n: f64) -> u16 {
    let bits = n.to_bits();
    let sign = ((bits >> 48) & 0x8000) as u16;
    if n.is_nan() {
        return sign | 0x7E00;
    }
    let a = n.abs();
    if a.is_infinite() {
        return sign | 0x7C00;
    }
    // Largest finite half is 65504; values rounding past it become infinity.
    if a >= 65520.0 {
        return sign | 0x7C00;
    }
    // Smallest normal half is 2^-14; below it, multiples of 2^-24.
    if a < 6.103515625e-05 {
        // Subnormal: round a / 2^-24 to the nearest integer, ties to even.
        let q = a * 16777216.0;
        let r = round_ties_even(q) as u16;
        return sign | r;
    }
    let exp = ((bits >> 52) & 0x7FF) as i32 - 1023;
    let mant = bits & 0x000F_FFFF_FFFF_FFFF;
    // Keep 10 mantissa bits; round the 42 dropped bits ties-to-even.
    let mut m = (mant >> 42) as u32;
    let rest = mant & ((1u64 << 42) - 1);
    let half = 1u64 << 41;
    let mut e = exp + 15;
    if rest > half || (rest == half && (m & 1) == 1) {
        m += 1;
        if m == 0x400 {
            m = 0;
            e += 1;
        }
    }
    if e >= 31 {
        return sign | 0x7C00;
    }
    sign | ((e as u16) << 10) | m as u16
}

fn round_ties_even(x: f64) -> f64 {
    let f = x.floor();
    let d = x - f;
    if d > 0.5 || (d == 0.5 && f % 2.0 != 0.0) {
        f + 1.0
    } else {
        f
    }
}

/// A binary16 value as a Number.
pub fn f16_bits_to_f64(h: u16) -> f64 {
    let sign = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let e = ((h >> 10) & 0x1F) as i32;
    let m = (h & 0x3FF) as f64;
    match e {
        0 => sign * m * 2f64.powi(-24),
        31 => {
            if m == 0.0 {
                sign * f64::INFINITY
            } else {
                f64::NAN
            }
        }
        _ => sign * (1.0 + m / 1024.0) * 2f64.powi(e - 15),
    }
}

// ---- TypedArray With Buffer Witness Records (§10.4.5.9) ------------------------------

/// A TypedArray With Buffer Witness Record.
#[derive(Clone, Copy, Debug)]
pub struct TaRecord {
    pub object: Object,
    /// [[CachedBufferByteLength]]; `None` is ~detached~.
    pub cached_buffer_byte_length: Option<usize>,
}

/// MakeTypedArrayWithBufferWitnessRecord(obj, order) (§10.4.5.10)
pub fn make_ta_record(agent: &Agent, o: Object) -> TaRecord {
    let buffer = typed_array_data(agent, o)
        .and_then(|d| d.buffer)
        .expect("an initialized TypedArray");
    // 2-3. If IsDetachedBuffer(buffer) is true, let byteLength be detached; else ArrayBufferByteLength(buffer, order).
    let cached = if is_detached_buffer(agent, buffer) {
        None
    } else {
        Some(array_buffer_byte_length(agent, buffer))
    };
    TaRecord {
        object: o,
        cached_buffer_byte_length: cached,
    }
}

/// IsTypedArrayOutOfBounds(taRecord) (§10.4.5.15)
pub fn is_typed_array_out_of_bounds(agent: &Agent, rec: TaRecord) -> bool {
    let d = typed_array_data(agent, rec.object).expect("a TypedArray");
    // 2-3. If bufferByteLength is detached, return true.
    let Some(buffer_byte_length) = rec.cached_buffer_byte_length else {
        return true;
    };
    // 4-7. byteOffsetStart and byteOffsetEnd.
    let start = d.byte_offset;
    let end = match d.array_length {
        None => buffer_byte_length,
        Some(len) => start + len * d.kind.size(),
    };
    // 8. If byteOffsetStart > bufferByteLength or byteOffsetEnd > bufferByteLength, return true.
    start > buffer_byte_length || end > buffer_byte_length
}

/// TypedArrayLength(taRecord) (§10.4.5.12)
pub fn typed_array_length(agent: &Agent, rec: TaRecord) -> usize {
    let d = typed_array_data(agent, rec.object).expect("a TypedArray");
    // 3. If O.[[ArrayLength]] is not auto, return O.[[ArrayLength]].
    if let Some(len) = d.array_length {
        return len;
    }
    // 4-8. floor((byteLength - byteOffset) / elementSize).
    let byte_length = rec.cached_buffer_byte_length.expect("not out of bounds");
    (byte_length - d.byte_offset) / d.kind.size()
}

/// TypedArrayByteLength(taRecord) (§10.4.5.11)
pub fn typed_array_byte_length(agent: &Agent, rec: TaRecord) -> usize {
    // 1. If IsTypedArrayOutOfBounds(taRecord) is true, return 0.
    if is_typed_array_out_of_bounds(agent, rec) {
        return 0;
    }
    // 2-3. Let length be TypedArrayLength(taRecord); if length = 0, return 0.
    let length = typed_array_length(agent, rec);
    if length == 0 {
        return 0;
    }
    let d = typed_array_data(agent, rec.object).expect("a TypedArray");
    // 5. If O.[[ByteLength]] is not auto, return O.[[ByteLength]].
    if let Some(b) = d.byte_length {
        return b;
    }
    // 6-7. Return length × elementSize.
    length * d.kind.size()
}

/// IsTypedArrayFixedLength(O) (§10.4.5.16)
pub fn is_typed_array_fixed_length(agent: &Agent, o: Object) -> bool {
    let d = typed_array_data(agent, o).expect("a TypedArray");
    // 1. If O.[[ArrayLength]] is auto, return false.
    if d.array_length.is_none() {
        return false;
    }
    // 2. Let buffer be O.[[ViewedArrayBuffer]].
    let buffer = d.buffer.expect("initialized");
    // 3. If IsFixedLengthArrayBuffer(buffer) is false and IsSharedArrayBuffer(buffer) is false, return false.
    // 4. Return true.
    is_fixed_length_array_buffer(agent, buffer) || is_shared_array_buffer(agent, buffer)
}

/// IsValidIntegerIndex(O, index) (§10.4.5.17)
pub fn is_valid_integer_index(agent: &Agent, o: Object, index: f64) -> bool {
    let d = typed_array_data(agent, o).expect("a TypedArray");
    // 1. If IsDetachedBuffer(O.[[ViewedArrayBuffer]]) is true, return false.
    if is_detached_buffer(agent, d.buffer.expect("initialized")) {
        return false;
    }
    // 2-3. If IsIntegralNumber(index) is false or index is -0𝔽, return false.
    if index.fract() != 0.0
        || index.is_infinite()
        || index.is_nan()
        || (index == 0.0 && index.is_sign_negative())
    {
        return false;
    }
    // 4-6. If IsTypedArrayOutOfBounds(taRecord) is true, return false.
    let rec = make_ta_record(agent, o);
    if is_typed_array_out_of_bounds(agent, rec) {
        return false;
    }
    // 7-8. If ℝ(index) < 0 or ℝ(index) ≥ length, return false.
    index >= 0.0 && index < typed_array_length(agent, rec) as f64
}

/// TypedArrayGetElement(O, index) (§10.4.5.18)
pub fn typed_array_get_element(agent: &mut Agent, o: Object, index: f64) -> Value {
    // 1. If IsValidIntegerIndex(O, index) is false, return undefined.
    if !is_valid_integer_index(agent, o, index) {
        return Value::Undefined;
    }
    let d = typed_array_data(agent, o).expect("a TypedArray");
    // 2-5. byteIndexInBuffer = (ℝ(index) × elementSize) + offset.
    let byte_index = index as usize * d.kind.size() + d.byte_offset;
    // 6-7. Return GetValueFromBuffer(O.[[ViewedArrayBuffer]], byteIndexInBuffer, elementType, true, unordered).
    get_value_from_buffer(
        agent,
        d.buffer.expect("initialized"),
        byte_index,
        d.kind,
        true,
    )
}

/// TypedArraySetElement(O, index, value) (§10.4.5.19)
pub fn typed_array_set_element(
    agent: &mut Agent,
    o: Object,
    index: f64,
    value: Value,
) -> JsResult<()> {
    let d = typed_array_data(agent, o).expect("a TypedArray");
    // 1-2. If O.[[ContentType]] is bigint, let numValue be ? ToBigInt(value); else ? ToNumber(value).
    let num = if d.kind.is_bigint() {
        Value::BigInt(ops::bigint::to_bigint(agent, value)?)
    } else {
        Value::Number(ops::to_number(agent, value)?)
    };
    // 3. If IsValidIntegerIndex(O, index) is true, then
    if is_valid_integer_index(agent, o, index) {
        let d = typed_array_data(agent, o).expect("a TypedArray");
        // a-e. Perform SetValueInBuffer(O.[[ViewedArrayBuffer]], byteIndexInBuffer, elementType, numValue, true, unordered).
        let byte_index = index as usize * d.kind.size() + d.byte_offset;
        set_value_in_buffer(
            agent,
            d.buffer.expect("initialized"),
            byte_index,
            d.kind,
            num,
            true,
        );
    }
    // 4. Return unused.
    Ok(())
}

// ---- The exotic internal methods (§10.4.5) -------------------------------------------

/// The numeric index a key stands for (CanonicalNumericIndexString), if any.
fn numeric_index(agent: &mut Agent, key: PropertyKey) -> Option<f64> {
    match key {
        PropertyKey::Index(i) => Some(i as f64),
        PropertyKey::Symbol(_) => None,
        PropertyKey::String(_) => ops::canonical_numeric_index_string(agent, key),
    }
}

fn is_typed_array(agent: &Agent, o: Object) -> bool {
    matches!(agent.obj(o).kind, ObjectKind::TypedArray(_))
}

/// [[GetOwnProperty]](P) (§10.4.5.1)
pub fn get_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
) -> Option<JsResult<Option<PropertyDescriptor>>> {
    if !is_typed_array(agent, o) {
        return None;
    }
    // 1. If P is a String, then
    let index = numeric_index(agent, key)?;
    // b.i. Let value be TypedArrayGetElement(O, numericIndex).
    let value = typed_array_get_element(agent, o, index);
    // b.ii. If value is undefined, return undefined.
    if value.is_undefined() {
        return Some(Ok(None));
    }
    // b.iii. Return the PropertyDescriptor { [[Value]]: value, [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: true }.
    Some(Ok(Some(PropertyDescriptor {
        value: Some(value),
        writable: Some(true),
        get: None,
        set: None,
        enumerable: Some(true),
        configurable: Some(true),
    })))
}

/// [[HasProperty]](P) (§10.4.5.2)
pub fn has_property(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<bool>> {
    if !is_typed_array(agent, o) {
        return None;
    }
    // 1.b. If numericIndex is not undefined, return IsValidIntegerIndex(O, numericIndex).
    let index = numeric_index(agent, key)?;
    Some(Ok(is_valid_integer_index(agent, o, index)))
}

/// [[DefineOwnProperty]](P, Desc) (§10.4.5.3)
pub fn define_own_property(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    desc: PropertyDescriptor,
) -> Option<JsResult<bool>> {
    if !is_typed_array(agent, o) {
        return None;
    }
    let index = numeric_index(agent, key)?;
    Some((|| {
        // i. If IsValidIntegerIndex(O, numericIndex) is false, return false.
        if !is_valid_integer_index(agent, o, index) {
            return Ok(false);
        }
        // ii-v. Configurable false, enumerable false, accessor, or writable false: return false.
        if desc.configurable == Some(false)
            || desc.enumerable == Some(false)
            || desc.get.is_some()
            || desc.set.is_some()
            || desc.writable == Some(false)
        {
            return Ok(false);
        }
        // vi. If Desc has a [[Value]] field, perform ? TypedArraySetElement(O, numericIndex, Desc.[[Value]]).
        if let Some(v) = desc.value {
            typed_array_set_element(agent, o, index, v)?;
        }
        // vii. Return true.
        Ok(true)
    })())
}

/// [[Get]](P, Receiver) (§10.4.5.4)
pub fn get(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<Value>> {
    if !is_typed_array(agent, o) {
        return None;
    }
    // 1.b. If numericIndex is not undefined, return TypedArrayGetElement(O, numericIndex).
    let index = numeric_index(agent, key)?;
    Some(Ok(typed_array_get_element(agent, o, index)))
}

/// [[Set]](P, V, Receiver) (§10.4.5.5): `None` means continue with OrdinarySet.
pub fn set(
    agent: &mut Agent,
    o: Object,
    key: PropertyKey,
    value: Value,
    receiver: Value,
) -> Option<JsResult<bool>> {
    if !is_typed_array(agent, o) {
        return None;
    }
    let index = numeric_index(agent, key)?;
    // i. If SameValue(O, Receiver) is true, then perform ? TypedArraySetElement(O, numericIndex, V); return true.
    if let Value::Object(r) = receiver
        && r == o
    {
        return Some(typed_array_set_element(agent, o, index, value).map(|_| true));
    }
    // ii. Else if IsValidIntegerIndex(O, numericIndex) is false, return true.
    if !is_valid_integer_index(agent, o, index) {
        return Some(Ok(true));
    }
    // 2. Return ? OrdinarySet(O, P, V, Receiver).
    None
}

/// [[Delete]](P) (§10.4.5.6)
pub fn delete(agent: &mut Agent, o: Object, key: PropertyKey) -> Option<JsResult<bool>> {
    if !is_typed_array(agent, o) {
        return None;
    }
    // 1.b. If numericIndex is not undefined, return IsValidIntegerIndex(O, numericIndex) is false.
    let index = numeric_index(agent, key)?;
    Some(Ok(!is_valid_integer_index(agent, o, index)))
}

/// [[OwnPropertyKeys]] (§10.4.5.7)
pub fn own_property_keys(agent: &mut Agent, o: Object) -> Option<JsResult<Vec<PropertyKey>>> {
    if !is_typed_array(agent, o) {
        return None;
    }
    // 1. Let taRecord be MakeTypedArrayWithBufferWitnessRecord(O, seq-cst).
    let rec = make_ta_record(agent, o);
    let mut keys = Vec::new();
    // 3. If IsTypedArrayOutOfBounds(taRecord) is false, add each integer index below the length.
    if !is_typed_array_out_of_bounds(agent, rec) {
        let length = typed_array_length(agent, rec);
        keys.extend((0..length as u32).map(PropertyKey::Index));
    }
    // 4-5. Then the String keys in creation order, then the Symbol keys.
    keys.extend(
        agent
            .obj(o)
            .properties
            .keys_in_spec_order()
            .into_iter()
            .filter(|k| !matches!(k, PropertyKey::Index(_))),
    );
    Some(Ok(keys))
}
