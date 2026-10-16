//! Intl.DisplayNames ( _locales_, _options_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DisplayNames

use kahvi_core::prelude::*;

use crate::support::intl::{
    OptionType, ServiceConstructor, SpecialBehaviour, available_locales, get_option,
    get_string_option, resolve_options,
};
use crate::support::intl_display_names::DisplayNamesSlots;

/// Intl.DisplayNames
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.DisplayNames` function is called with arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _displayNames_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.DisplayNames.prototype%"*, « [[InitializedDisplayNames]], [[Locale]], [[Style]], [[Type]], [[Fallback]], [[LanguageDisplay]], [[Fields]] »).
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.DisplayNames%, %Intl.DisplayNames%.[[LocaleData]], _locales_, _options_, « ~require-options~ »).
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    // 6. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"narrow"*, *"short"*, *"long"* », *"long"*).
    // 7. Set _displayNames_.[[Style]] to _style_.
    // 8. Let _type_ be ? GetOption(_options_, *"type"*, ~string~, « *"language"*, *"region"*, *"script"*, *"currency"*, *"calendar"*, *"dateTimeField"* », ~required~).
    // 9. Set _displayNames_.[[Type]] to _type_.
    // 10. Let _fallback_ be ? GetOption(_options_, *"fallback"*, ~string~, « *"code"*, *"none"* », *"code"*).
    // 11. Set _displayNames_.[[Fallback]] to _fallback_.
    // 12. Set _displayNames_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 13. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
    // 14. Let _types_ be _resolvedLocaleData_.[[types]].
    // 15. Assert: _types_ is a Record (see [sec-Intl.DisplayNames-internal-slots]).
    // 16. Let _languageDisplay_ be ? GetOption(_options_, *"languageDisplay"*, ~string~, « *"dialect"*, *"standard"* », *"dialect"*).
    // 17. Let _typeFields_ be _types_.[[<_type_>]].
    // 18. Assert: _typeFields_ is a Record (see [sec-Intl.DisplayNames-internal-slots]).
    // 19. If _type_ is *"language"*, then
    //    a. Set _displayNames_.[[LanguageDisplay]] to _languageDisplay_.
    //    b. Set _typeFields_ to _typeFields_.[[<_languageDisplay_>]].
    //    c. Assert: _typeFields_ is a Record (see [sec-Intl.DisplayNames-internal-slots]).
    // 20. Let _styleFields_ be _typeFields_.[[<_style_>]].
    // 21. Assert: _styleFields_ is a Record (see [sec-Intl.DisplayNames-internal-slots]).
    // 22. Set _displayNames_.[[Fields]] to _styleFields_.
    // 23. Return _displayNames_.
    let _ = this;
    let locales = args.get_or_undefined(0);
    let options = args.get_or_undefined(1);
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor Intl.DisplayNames requires 'new'"));
    };
    // 2. Let _displayNames_ be ? OrdinaryCreateFromConstructor(...).
    let display_names = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Intl.DisplayNames.prototype%",
        ObjectKind::Native(Box::new(DisplayNamesSlots::default())),
    )?;
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.DisplayNames%, %Intl.DisplayNames%.[[LocaleData]], _locales_, _options_, « ~require-options~ »).
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
        &[SpecialBehaviour::RequireOptions],
        None,
    )?;
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    let options = options_resolution.options;
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    let resolved_locale = options_resolution.resolved_locale;
    // 6. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"narrow"*, *"short"*, *"long"* », *"long"*).
    let style = get_string_option(
        agent,
        options,
        "style",
        &["narrow", "short", "long"],
        Some("long"),
    )?
    .unwrap_or_default();
    // 7. Set _displayNames_.[[Style]] to _style_.
    // 8. Let _type_ be ? GetOption(_options_, *"type"*, ~string~, « ... », ~required~).
    let r#type = match get_option(
        agent,
        options,
        "type",
        OptionType::String,
        Some(&[
            "language",
            "region",
            "script",
            "currency",
            "calendar",
            "dateTimeField",
        ]),
        None,
    )? {
        Value::String(s) => agent.str_lossy(s),
        _ => return Err(agent.throw_type_error("type is required")),
    };
    // 9. Set _displayNames_.[[Type]] to _type_.
    // 10. Let _fallback_ be ? GetOption(_options_, *"fallback"*, ~string~, « *"code"*, *"none"* », *"code"*).
    let fallback = get_string_option(agent, options, "fallback", &["code", "none"], Some("code"))?
        .unwrap_or_default();
    // 11. Set _displayNames_.[[Fallback]] to _fallback_.
    // 12. Set _displayNames_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 13-15. (The [[types]] locale data is looked up from ICU4X when `of` is called.)
    // 16. Let _languageDisplay_ be ? GetOption(_options_, *"languageDisplay"*, ~string~, « *"dialect"*, *"standard"* », *"dialect"*).
    let language_display = get_string_option(
        agent,
        options,
        "languageDisplay",
        &["dialect", "standard"],
        Some("dialect"),
    )?;
    // 17-18. Let _typeFields_ be _types_.[[<_type_>]].
    // 19. If _type_ is *"language"*, then
    //    a. Set _displayNames_.[[LanguageDisplay]] to _languageDisplay_.
    let language_display = if r#type == "language" {
        language_display
    } else {
        None
    };
    // 20-22. Set _displayNames_.[[Fields]] to _styleFields_ (resolved from ICU4X on lookup).
    let Some(slots) = agent.native_mut::<DisplayNamesSlots>(display_names) else {
        return Err(agent.throw_type_error("not an Intl.DisplayNames"));
    };
    slots.locale = resolved_locale.locale;
    slots.style = style;
    slots.r#type = r#type;
    slots.fallback = fallback;
    slots.language_display = language_display;
    // 23. Return _displayNames_.
    Ok(Value::Object(display_names))
}
