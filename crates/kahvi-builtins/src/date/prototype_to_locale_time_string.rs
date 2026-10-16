//! Date.prototype.toLocaleTimeString ( [ _reserved1_ [ , _reserved2_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.tolocaletimestring

use kahvi_core::prelude::*;

use crate::support::date;
use crate::support::intl_date_time_format::{
    Defaults, Required, create_date_time_format, format_date_time,
};

/// Date.prototype.toLocaleTimeString
pub(crate) fn prototype_to_locale_time_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    if agent.intl {
        let locales = args.first().copied().unwrap_or(Value::Undefined);
        let options = args.get(1).copied().unwrap_or(Value::Undefined);
        // 1. Let _dateObj_ be the *this* value.
        // 2. Perform ? RequireInternalSlot(_dateObj_, [[DateValue]]).
        // 3. Let _x_ be _dateObj_.[[DateValue]].
        let x = date::this_time_value(agent, this)?;
        // 4. If _x_ is *NaN*, return *"Invalid Date"*.
        if x.is_nan() {
            return Ok(Value::String(agent.string("Invalid Date")));
        }
        // 5. Let _timeFormat_ be ? CreateDateTimeFormat(%Intl.DateTimeFormat%, _locales_, _options_, ~time~, ~time~).
        let ctor = agent.intrinsic("%Intl.DateTimeFormat%");
        let time_format = create_date_time_format(
            agent,
            ctor,
            locales,
            options,
            Required::Time,
            Defaults::Time,
        )?;
        // 6. Return ! FormatDateTime(_timeFormat_, _x_).
        let result = format_date_time(agent, time_format, x)?;
        return Ok(Value::String(agent.string(&result)));
    }
    // An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used:
    // This method returns a String value. The contents of the String are implementation-defined, but are intended to represent the “time” portion of the Date in the current time zone in a convenient, human-readable form that corresponds to the conventions of the host environment's current locale.
    // The meaning of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.
    // kahvi has no ECMA-402 and no host locale: it returns what `toTimeString` returns.
    // The optional parameters are ECMA-402's, so none is passed on.
    let _ = args;
    super::prototype_to_time_string::prototype_to_time_string(agent, this, &[], new_target)
}
