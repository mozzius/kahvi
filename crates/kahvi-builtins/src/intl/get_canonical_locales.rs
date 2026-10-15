//! Intl.getCanonicalLocales ( _locales_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.getcanonicallocales

use kahvi_core::prelude::*;

use crate::support::intl::canonicalize_locale_list;

/// Intl.getCanonicalLocales
pub(crate) fn get_canonical_locales(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `getCanonicalLocales` method is called with argument _locales_, the following steps are taken:
    //
    let _ = (this, new_target);
    let locales = args.get_or_undefined(0);
    // 1. Let _localeList_ be ? CanonicalizeLocaleList(_locales_).
    let locale_list = canonicalize_locale_list(agent, locales)?;
    // 2. Return CreateArrayFromList(_localeList_).
    let elements: Vec<Value> = locale_list
        .iter()
        .map(|l| Value::String(agent.string(l)))
        .collect();
    Ok(Value::Object(ops::create_array_from_list(agent, &elements)))
}
