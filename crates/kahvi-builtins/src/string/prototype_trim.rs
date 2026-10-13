//! String.prototype.trim ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.trim

use kahvi_core::prelude::*;

/// String.prototype.trim
pub(crate) fn prototype_trim(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
    // It performs the following steps when called:
    //
    // 1. Let _thisValue_ be the *this* value.
    // 2. Return ? TrimString(_thisValue_, ~start+end~).
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    // [subclause sec-trimstring: TrimString (
    // _arg_: an ECMAScript language value,
    // _where_: ~start~, ~end~, or ~start+end~,
    // ): either a normal completion containing a String or a throw completion]
    let _ = (args, new_target);
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Return ? TrimString(_thisValue_, ~start+end~).
    let trimmed = trim_string(agent, this_value, TrimWhere::StartEnd)?;
    Ok(Value::String(trimmed))
}

/// The _where_ parameter of TrimString.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TrimWhere {
    Start,
    End,
    StartEnd,
}

/// TrimString ( _arg_, _where_ ) (§22.1.3.35.1)
pub(crate) fn trim_string(agent: &mut Agent, arg: Value, where_: TrimWhere) -> JsResult<JsString> {
    // 1. Perform ? RequireObjectCoercible(_arg_).
    ops::require_object_coercible(agent, arg)?;
    // 2. Let _string_ be ? ToString(_arg_).
    let string = ops::to_string(agent, arg)?;
    let units = agent.str(string);
    let len = units.len();
    // 3. If _where_ is ~start~, then
    //    a. Let _trimmedString_ be the String value that is a copy of _string_ with leading white space removed.
    // 4. Else if _where_ is ~end~, then
    //    a. Let _trimmedString_ be the String value that is a copy of _string_ with trailing white space removed.
    // 5. Else,
    //    a. Assert: _where_ is ~start+end~.
    //    b. Let _trimmedString_ be the String value that is a copy of _string_ with both leading and trailing white space removed.
    let mut from = 0;
    if where_ != TrimWhere::End {
        while from < len && is_trim_white_space(units[from]) {
            from += 1;
        }
    }
    let mut to = len;
    if where_ != TrimWhere::Start {
        while to > from && is_trim_white_space(units[to - 1]) {
            to -= 1;
        }
    }
    // 6. Return _trimmedString_.
    Ok(agent.substring(string, from, to))
}

/// The definition of white space used by TrimString: the union of |WhiteSpace|
/// and |LineTerminator|. Every member is in the BMP, so one UTF-16 code unit
/// is always enough to decide it. "Zs" is the Space_Separator general category.
fn is_trim_white_space(unit: u16) -> bool {
    matches!(
        unit,
        // |WhiteSpace|: TAB, VT, FF, ZWNBSP, and the Zs code points.
        0x0009 | 0x000B | 0x000C | 0x0020 | 0x00A0 | 0xFEFF | 0x1680 | 0x2000
            ..=0x200A
            | 0x202F | 0x205F | 0x3000
            // |LineTerminator|: LF, CR, LS, PS.
            | 0x000A | 0x000D | 0x2028 | 0x2029
    )
}
