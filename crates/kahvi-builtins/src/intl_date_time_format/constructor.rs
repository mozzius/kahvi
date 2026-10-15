//! Intl.DateTimeFormat ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.datetimeformat

use kahvi_core::prelude::*;

use crate::support::intl::fallback_symbol;
use crate::support::intl_date_time_format::{Defaults, Required, create_date_time_format};

/// Intl.DateTimeFormat
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.DateTimeFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object, else let _newTarget_ be NewTarget.
    // 2. Let _dateTimeFormat_ be ? CreateDateTimeFormat(_newTarget_, _locales_, _options_, ~any~, ~date~).
    // 3. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Let _this_ be the *this* value.
    //    b. Return ? ChainDateTimeFormat(_dateTimeFormat_, NewTarget, _this_).
    // 4. Return _dateTimeFormat_.
    //
    // [subclause sec-chaindatetimeformat: ChainDateTimeFormat (
    // _dateTimeFormat_: an Intl.DateTimeFormat,
    // _newTarget_: an ECMAScript language value,
    // _this_: an ECMAScript language value,
    // ): either a normal completion containing an Object or a throw completion]
    let locales = args.get_or_undefined(0);
    let options = args.get_or_undefined(1);
    let new_target_arg = new_target;
    let new_target = match new_target {
        Some(nt) => nt,
        None => agent.intrinsic("%Intl.DateTimeFormat%"),
    };
    let date_time_format = create_date_time_format(
        agent,
        new_target,
        locales,
        options,
        Required::Any,
        Defaults::Date,
    )?;
    // ChainDateTimeFormat ( _dateTimeFormat_, _newTarget_, _this_ )
    //
    // 1. If _newTarget_ is *undefined* and ? OrdinaryHasInstance(%Intl.DateTimeFormat%, _this_) is *true*, then
    if new_target_arg.is_none() {
        let ctor = agent.intrinsic("%Intl.DateTimeFormat%");
        if ops::ordinary_has_instance(agent, Value::Object(ctor), this)? {
            let Value::Object(this_object) = this else {
                return Err(agent.throw_type_error("this is not an object"));
            };
            // a. Perform ? DefinePropertyOrThrow(_this_, %Intl%.[[FallbackSymbol]], PropertyDescriptor { [[Value]]: _dateTimeFormat_, [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
            let symbol = fallback_symbol(agent);
            ops::define_property_or_throw(
                agent,
                this_object,
                symbol.into(),
                PropertyDescriptor::data(Value::Object(date_time_format), false, false, false),
            )?;
            // b. Return _this_.
            return Ok(this);
        }
    }
    // 2. Return _dateTimeFormat_.
    Ok(Value::Object(date_time_format))
}
