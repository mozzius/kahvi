//! get Intl.DateTimeFormat.prototype.format
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.datetimeformat.prototype.format

use kahvi_core::function::{builtin_slots, create_builtin_function};
use kahvi_core::prelude::*;

use crate::support::date;
use crate::support::intl_date_time_format::{
    DateTimeFormatSlots, format_date_time, unwrap_date_time_format,
};

/// The closure of step 4.a.
fn bound_format(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let Some(f) = agent.active_function() else {
        return Err(agent.throw_type_error("no running built-in function"));
    };
    let Some(Value::Object(dtf)) = builtin_slots(agent, f).first().copied() else {
        return Err(agent.throw_type_error("format function without its DateTimeFormat"));
    };
    // i. If _date_ is *undefined*, let _x_ be ! Call(%Date.now%, *undefined*); else let _x_ be ? ToNumber(_date_).
    let x = match args.get_or_undefined(0) {
        Value::Undefined => date::now(agent),
        date => ops::to_number(agent, date)?,
    };
    // ii. Return ? FormatDateTime(_dtf_, _x_).
    let result = format_date_time(agent, dtf, x)?;
    Ok(Value::String(agent.string(&result)))
}

/// get Intl.DateTimeFormat.prototype.format
pub(crate) fn prototype_get_format(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // Intl.DateTimeFormat.prototype.format is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _dtf_ be the *this* value.
    // 2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Set _dtf_ to ? UnwrapDateTimeFormat(_dtf_).
    // 3. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
    // 4. If _dtf_.[[BoundFormat]] is *undefined*, then
    //    a. Let _closure_ be a new Abstract Closure with parameters (_date_) that captures _dtf_ and performs the following steps when called:
    //       i. If _date_ is *undefined*, let _x_ be ! Call(%Date.now%, *undefined*); else let _x_ be ? ToNumber(_date_).
    //       ii. Return ? FormatDateTime(_dtf_, _x_).
    //    b. Set _dtf_.[[BoundFormat]] to CreateBuiltinFunction(_closure_, 1, *""*, « »).
    // 5. Return _dtf_.[[BoundFormat]].
    //
    // NOTE:
    // The returned function is bound to _dtf_ so that it can be passed directly to `Array.prototype.map` or other functions.
    // This is considered a historical artefact, as part of a convention which is no longer followed for new features, but is preserved to maintain compatibility with existing programs.
    let _ = (args, new_target);
    // 1. Let _dtf_ be the *this* value.
    // 2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Set _dtf_ to ? UnwrapDateTimeFormat(_dtf_).
    let dtf = unwrap_date_time_format(agent, this)?;
    // 3. Perform ? RequireInternalSlot(_dtf_, [[InitializedDateTimeFormat]]).
    let dtf = match dtf {
        Value::Object(o) if agent.native::<DateTimeFormatSlots>(o).is_some() => o,
        _ => return Err(agent.throw_type_error("not an Intl.DateTimeFormat")),
    };
    // 4. If _dtf_.[[BoundFormat]] is *undefined*, then
    let bound = agent
        .native::<DateTimeFormatSlots>(dtf)
        .and_then(|s| s.bound_format);
    let bound = match bound {
        Some(b) => b,
        None => {
            // a. Let _closure_ be a new Abstract Closure with parameters (_date_) that captures _dtf_ and performs the steps of bound_format.
            // b. Set _dtf_.[[BoundFormat]] to CreateBuiltinFunction(_closure_, 1, *""*, « »).
            let name = PropertyKey::String(agent.names.empty);
            let f = create_builtin_function(
                agent,
                bound_format,
                1,
                name,
                vec![Value::Object(dtf)],
                None,
                None,
                None,
                false,
            );
            if let Some(slots) = agent.native_mut::<DateTimeFormatSlots>(dtf) {
                slots.bound_format = Some(f);
            }
            f
        }
    };
    // 5. Return _dtf_.[[BoundFormat]].
    Ok(Value::Object(bound))
}
