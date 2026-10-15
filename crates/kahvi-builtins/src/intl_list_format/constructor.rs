//! Intl.ListFormat ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.ListFormat

use kahvi_core::prelude::*;

use crate::support::intl::{
    ServiceConstructor, available_locales, get_string_option, resolve_options,
};
use crate::support::intl_list_format::{ListFormatSlots, templates_for};

/// Intl.ListFormat
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.ListFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    let _ = this;
    let locales = args.get_or_undefined(0);
    let options = args.get_or_undefined(1);
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor Intl.ListFormat requires 'new'"));
    };
    // 2. Let _listFormat_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.ListFormat.prototype%"*, « [[InitializedListFormat]], [[Locale]], [[Type]], [[Style]], [[Templates]] »).
    let list_format = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Intl.ListFormat.prototype%",
        ObjectKind::Native(Box::new(ListFormatSlots::default())),
    )?;
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.ListFormat%, %Intl.ListFormat%.[[LocaleData]], _locales_, _options_).
    // (%Intl.ListFormat% has no relevant extension keys or resolution option descriptors.)
    let ctor = ServiceConstructor {
        available_locales: available_locales(),
        relevant_extension_keys: &[],
        resolution_option_descriptors: &[],
    };
    let options_resolution = resolve_options(
        agent,
        &ctor,
        &|_, _| Vec::new(),
        locales,
        options,
        &[],
        None,
    )?;
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    let options = options_resolution.options;
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    let resolved_locale = options_resolution.resolved_locale;
    // 6. Set _listFormat_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 7. Let _type_ be ? GetOption(_options_, *"type"*, ~string~, « *"conjunction"*, *"disjunction"*, *"unit"* », *"conjunction"*).
    let r#type = get_string_option(
        agent,
        options,
        "type",
        &["conjunction", "disjunction", "unit"],
        Some("conjunction"),
    )?
    .unwrap_or_default();
    // 8. Set _listFormat_.[[Type]] to _type_.
    // 9. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"long"*, *"short"*, *"narrow"* », *"long"*).
    let style = get_string_option(
        agent,
        options,
        "style",
        &["long", "short", "narrow"],
        Some("long"),
    )?
    .unwrap_or_default();
    // 10. Set _listFormat_.[[Style]] to _style_.
    // 11. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
    // 12. Let _dataLocaleTypes_ be _resolvedLocaleData_.[[<_type_>]].
    // 13. Set _listFormat_.[[Templates]] to _dataLocaleTypes_.[[<_style_>]].
    let templates = templates_for(agent, &resolved_locale.data_locale, &r#type, &style)?;
    if let Some(slots) = agent.native_mut::<ListFormatSlots>(list_format) {
        slots.locale = resolved_locale.locale;
        slots.r#type = r#type;
        slots.style = style;
        slots.templates = Some(templates);
    }
    // 14. Return _listFormat_.
    Ok(Value::Object(list_format))
}
