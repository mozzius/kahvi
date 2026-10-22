//! ECMAScript language values (ECMA-262 §6.1).
//!
//! Every value is `Copy`: strings, symbols, BigInts and objects are handles
//! into the agent's heap. Strings are interned, so two `JsString`s are equal
//! exactly when their contents are.

/// A String value: a handle to an interned sequence of UTF-16 code units.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct JsString(pub(crate) u32);

/// A Symbol value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Symbol(pub(crate) u32);

/// A BigInt value (handle; arithmetic lives in `ops::bigint`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BigInt(pub(crate) u32);

/// An Object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Object(pub(crate) u32);

impl Object {
    /// A stable number identifying this object (for maps keyed by identity).
    pub fn id(self) -> u32 {
        self.0
    }
}

/// An ECMAScript language value.
///
/// `Value` deliberately does not implement `PartialEq`: JavaScript has
/// several equalities. Use [`crate::ops::same_value`],
/// [`crate::ops::same_value_zero`], [`crate::ops::is_strictly_equal`] or
/// [`crate::ops::is_loosely_equal`] as the spec says.
#[derive(Clone, Copy, Debug, Default)]
pub enum Value {
    #[default]
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(JsString),
    Symbol(Symbol),
    BigInt(BigInt),
    Object(Object),
}

/// A [`Value`] in 8 bytes, for where many are kept (an object's property
/// values). A Number is its own bits, every NaN as the one NaN; any other
/// value is a NaN no Number is: its kind in the top 16 bits and its handle
/// (or boolean) in the low 32.
#[derive(Clone, Copy)]
pub(crate) struct PackedValue(u64);

/// The bits of every NaN Number.
const PACKED_NAN: u64 = 0x7FF8_0000_0000_0000;
/// The top 16 bits of the NaNs that are other values, less their kind
/// (1 to 7); no Number has top bits above this one.
const PACKED_TAG: u64 = 0xFFF8;

impl PackedValue {
    #[inline]
    const fn tagged(kind: u64, payload: u32) -> PackedValue {
        PackedValue((PACKED_TAG + kind) << 48 | payload as u64)
    }

    #[inline]
    pub(crate) fn pack(v: Value) -> PackedValue {
        match v {
            Value::Number(n) if n.is_nan() => PackedValue(PACKED_NAN),
            Value::Number(n) => PackedValue(n.to_bits()),
            Value::Undefined => Self::tagged(1, 0),
            Value::Null => Self::tagged(2, 0),
            Value::Boolean(b) => Self::tagged(3, b as u32),
            Value::String(s) => Self::tagged(4, s.0),
            Value::Symbol(s) => Self::tagged(5, s.0),
            Value::BigInt(b) => Self::tagged(6, b.0),
            Value::Object(o) => Self::tagged(7, o.0),
        }
    }

    #[inline]
    pub(crate) fn unpack(self) -> Value {
        let top = self.0 >> 48;
        if top <= PACKED_TAG {
            return Value::Number(f64::from_bits(self.0));
        }
        let payload = self.0 as u32;
        match top - PACKED_TAG {
            1 => Value::Undefined,
            2 => Value::Null,
            3 => Value::Boolean(payload != 0),
            4 => Value::String(JsString(payload)),
            5 => Value::Symbol(Symbol(payload)),
            6 => Value::BigInt(BigInt(payload)),
            _ => Value::Object(Object(payload)),
        }
    }

    /// No value (an uninitialized binding): a NaN neither a Number nor any
    /// other value is.
    pub(crate) const EMPTY: PackedValue = PackedValue(PACKED_TAG << 48);

    /// `v`, or [`EMPTY`](Self::EMPTY) for `None`.
    #[inline]
    pub(crate) fn pack_option(v: Option<Value>) -> PackedValue {
        v.map_or(Self::EMPTY, Self::pack)
    }

    /// The value, `None` for [`EMPTY`](Self::EMPTY).
    #[inline]
    pub(crate) fn unpack_option(self) -> Option<Value> {
        (self.0 != Self::EMPTY.0).then(|| self.unpack())
    }

    /// The bits, for a container keeping something else in the same 8
    /// bytes where it knows which it keeps.
    #[inline]
    pub(crate) const fn bits(self) -> u64 {
        self.0
    }

    #[inline]
    pub(crate) fn from_bits(bits: u64) -> PackedValue {
        PackedValue(bits)
    }
}

impl std::fmt::Debug for PackedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.unpack().fmt(f)
    }
}

impl Value {
    pub fn is_undefined(self) -> bool {
        matches!(self, Value::Undefined)
    }
    pub fn is_null(self) -> bool {
        matches!(self, Value::Null)
    }
    pub fn is_nullish(self) -> bool {
        matches!(self, Value::Undefined | Value::Null)
    }
    pub fn is_object(self) -> bool {
        matches!(self, Value::Object(_))
    }
    pub fn as_object(self) -> Option<Object> {
        match self {
            Value::Object(o) => Some(o),
            _ => None,
        }
    }
    pub fn as_number(self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }
    pub fn as_string(self) -> Option<JsString> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_symbol(self) -> Option<Symbol> {
        match self {
            Value::Symbol(s) => Some(s),
            _ => None,
        }
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Value {
        Value::Boolean(b)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Value {
        Value::Number(n)
    }
}

impl From<i32> for Value {
    fn from(n: i32) -> Value {
        Value::Number(n as f64)
    }
}

impl From<u32> for Value {
    fn from(n: u32) -> Value {
        Value::Number(n as f64)
    }
}

impl From<usize> for Value {
    fn from(n: usize) -> Value {
        Value::Number(n as f64)
    }
}

impl From<JsString> for Value {
    fn from(s: JsString) -> Value {
        Value::String(s)
    }
}

impl From<Symbol> for Value {
    fn from(s: Symbol) -> Value {
        Value::Symbol(s)
    }
}

impl From<Object> for Value {
    fn from(o: Object) -> Value {
        Value::Object(o)
    }
}

impl From<Option<Object>> for Value {
    /// `None` is `null` (as for prototypes).
    fn from(o: Option<Object>) -> Value {
        o.map_or(Value::Null, Value::Object)
    }
}

/// Convenience for builtin argument lists: missing arguments are undefined.
pub trait Args {
    fn get_or_undefined(&self, i: usize) -> Value;
}

impl Args for [Value] {
    fn get_or_undefined(&self, i: usize) -> Value {
        self.get(i).copied().unwrap_or(Value::Undefined)
    }
}
