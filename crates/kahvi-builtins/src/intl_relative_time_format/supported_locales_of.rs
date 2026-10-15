//! Intl.RelativeTimeFormat.supportedLocalesOf ( _locales_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.RelativeTimeFormat.supportedLocalesOf

use kahvi_core::prelude::*;

use crate::support::intl::{available_locales, canonicalize_locale_list, filter_locales};

/// Intl.RelativeTimeFormat.supportedLocalesOf
pub(crate) fn supported_locales_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `supportedLocalesOf` method is called with arguments _locales_ and _options_, the following steps are taken:
    //
    let _ = (this, new_target);
    let locales = args.get_or_undefined(0);
    let options = args.get_or_undefined(1);
    // 1. Let _availableLocales_ be %Intl.RelativeTimeFormat%.[[AvailableLocales]].
    let available_locales = available_locales();
    // 2. Let _requestedLocales_ be ? CanonicalizeLocaleList(_locales_).
    let requested_locales = canonicalize_locale_list(agent, locales)?;
    // 3. Return ? FilterLocales(_availableLocales_, _requestedLocales_, _options_).
    Ok(Value::Object(filter_locales(
        agent,
        available_locales,
        &requested_locales,
        options,
    )?))
}
