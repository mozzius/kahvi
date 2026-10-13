//! Date.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.prototype.tolocalestring

use kahvi_core::prelude::*;

/// Date.prototype.toLocaleString
pub(crate) fn prototype_to_locale_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used:
    // This method returns a String value. The contents of the String are implementation-defined, but are intended to represent the Date in the current time zone in a convenient, human-readable form that corresponds to the conventions of the host environment's current locale.
    // The meaning of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.prototype.toLocaleString"))
}
