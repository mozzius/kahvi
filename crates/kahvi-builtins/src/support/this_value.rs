//! The this*Value abstract operations of the primitive wrapper prototypes.

use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;

/// ThisBooleanValue(value) (§20.3.3.3.1)
pub(crate) fn this_boolean_value(agent: &mut Agent, value: Value) -> JsResult<bool> {
    match value {
        // 1. If value is a Boolean, return value.
        Value::Boolean(b) => Ok(b),
        // 2. If value is an Object and value has a [[BooleanData]] internal slot, then return b.
        Value::Object(o) if matches!(agent.obj(o).kind, ObjectKind::BooleanObject(_)) => {
            let ObjectKind::BooleanObject(b) = agent.obj(o).kind else {
                unreachable!()
            };
            Ok(b)
        }
        // 3. Throw a TypeError exception.
        _ => Err(agent.throw_type_error("not a Boolean")),
    }
}

/// ThisNumberValue(value) (§21.1.3.7.1)
pub(crate) fn this_number_value(agent: &mut Agent, value: Value) -> JsResult<f64> {
    match value {
        Value::Number(n) => Ok(n),
        Value::Object(o) if matches!(agent.obj(o).kind, ObjectKind::NumberObject(_)) => {
            let ObjectKind::NumberObject(n) = agent.obj(o).kind else {
                unreachable!()
            };
            Ok(n)
        }
        _ => Err(agent.throw_type_error("not a Number")),
    }
}

/// ThisStringValue(value) (§22.1.3.35.1)
pub(crate) fn this_string_value(agent: &mut Agent, value: Value) -> JsResult<JsString> {
    match value {
        Value::String(s) => Ok(s),
        Value::Object(o) if matches!(agent.obj(o).kind, ObjectKind::StringObject(_)) => {
            let ObjectKind::StringObject(s) = agent.obj(o).kind else {
                unreachable!()
            };
            Ok(s)
        }
        _ => Err(agent.throw_type_error("not a String")),
    }
}

/// ThisSymbolValue(value) (§20.4.3.4.1)
pub(crate) fn this_symbol_value(agent: &mut Agent, value: Value) -> JsResult<Symbol> {
    match value {
        Value::Symbol(s) => Ok(s),
        Value::Object(o) if matches!(agent.obj(o).kind, ObjectKind::SymbolObject(_)) => {
            let ObjectKind::SymbolObject(s) = agent.obj(o).kind else {
                unreachable!()
            };
            Ok(s)
        }
        _ => Err(agent.throw_type_error("not a Symbol")),
    }
}

/// SymbolDescriptiveString(sym) (§20.4.3.3.1)
pub(crate) fn symbol_descriptive_string(agent: &mut Agent, sym: Symbol) -> JsString {
    // 1. Let desc be sym's [[Description]] value.
    // 2. If desc is undefined, set desc to the empty String.
    let desc = agent.symbol_description(sym).unwrap_or(agent.names.empty);
    // 4. Return the string-concatenation of "Symbol(", desc, and ")".
    let mut units: Vec<u16> = "Symbol(".encode_utf16().collect();
    units.extend_from_slice(agent.str(desc));
    units.push(u16::from(b')'));
    agent.intern(&units)
}
