//! Intl.DisplayNames.prototype.of ( _code_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DisplayNames.prototype.of

use kahvi_core::prelude::*;

use crate::support::intl_display_names::{
    DisplayNamesSlots, canonical_code_for_display_names, lookup_display_name,
};

/// Intl.DisplayNames.prototype.of
pub(crate) fn prototype_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.DisplayNames.prototype.of` is called with an argument _code_, the following steps are taken:
    //
    // 1. Let _displayNames_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_displayNames_, [[InitializedDisplayNames]]).
    // 3. Set _code_ to ? ToString(_code_).
    // 4. Set _code_ to ? CanonicalCodeForDisplayNames(_displayNames_.[[Type]], _code_).
    // 5. Let _fields_ be _displayNames_.[[Fields]].
    // 6. If _fields_ has a field [[<_code_>]], return _fields_.[[<_code_>]].
    // 7. If _displayNames_.[[Fallback]] is *"code"*, return _code_.
    // 8. Return *undefined*.
    let _ = new_target;
    // 1. Let _displayNames_ be *this* value.
    // 2. Perform ? RequireInternalSlot(_displayNames_, [[InitializedDisplayNames]]).
    let slots = match this {
        Value::Object(o) => agent.native::<DisplayNamesSlots>(o).cloned(),
        _ => None,
    };
    let Some(slots) = slots else {
        return Err(agent.throw_type_error("not an Intl.DisplayNames"));
    };
    // 3. Set _code_ to ? ToString(_code_).
    let code = ops::to_string(agent, args.get_or_undefined(0))?;
    let code = agent.str_lossy(code);
    // 4. Set _code_ to ? CanonicalCodeForDisplayNames(_displayNames_.[[Type]], _code_).
    let code = canonical_code_for_display_names(agent, &slots.r#type, &code)?;
    // 5. Let _fields_ be _displayNames_.[[Fields]].
    // 6. If _fields_ has a field [[<_code_>]], return _fields_.[[<_code_>]].
    if let Some(name) = lookup_display_name(agent, &slots, &code)? {
        return Ok(Value::String(agent.string(&name)));
    }
    // 7. If _displayNames_.[[Fallback]] is *"code"*, return _code_.
    if slots.fallback == "code" {
        return Ok(Value::String(agent.string(&code)));
    }
    // 8. Return *undefined*.
    Ok(Value::Undefined)
}
