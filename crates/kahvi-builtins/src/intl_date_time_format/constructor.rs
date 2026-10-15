//! Intl.DateTimeFormat ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.datetimeformat

use kahvi_core::prelude::*;

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
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DateTimeFormat"))
}
