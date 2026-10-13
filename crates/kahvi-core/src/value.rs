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
