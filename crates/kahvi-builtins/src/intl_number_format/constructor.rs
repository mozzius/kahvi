//! Intl.NumberFormat ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.numberformat

use kahvi_core::prelude::*;

use crate::support::intl::{
    OptionType, ResolutionOptionDescriptor, ServiceConstructor, SpecialBehaviour,
    available_locales, fallback_symbol, get_boolean_or_string_number_format_option,
    get_string_option, resolve_options,
};
use crate::support::intl_number_format::{
    NumberFormatSlots, numbering_systems_of, set_number_format_digit_options,
    set_number_format_unit_options,
};

/// %Intl.NumberFormat%.[[LocaleData]]: the supported numbering systems for a
/// locale, the default first (support/intl_number_format.rs). ResolveLocale
/// only asks for locales in [[AvailableLocales]], which ICU4X has data for.
fn locale_data(locale: &str, key: &str) -> Vec<Option<String>> {
    match key {
        "nu" => numbering_systems_of(locale).unwrap_or_default(),
        _ => Vec::new(),
    }
}

/// Intl.NumberFormat
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let locales = args.get_or_undefined(0);
    let options = args.get_or_undefined(1);
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object, else let _newTarget_ be NewTarget.
    let ctor = match new_target {
        Some(nt) => nt,
        None => match agent.active_function() {
            Some(f) => f,
            None => return Err(agent.throw_type_error("no active function")),
        },
    };
    // 2. Let _numberFormat_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%Intl.NumberFormat.prototype%"*, « ... »).
    let number_format = ops::ordinary_create_from_constructor(
        agent,
        ctor,
        "%Intl.NumberFormat.prototype%",
        ObjectKind::Native(Box::new(NumberFormatSlots::new(
            String::new(),
            String::new(),
        ))),
    )?;
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.NumberFormat%, %Intl.NumberFormat%.[[LocaleData]], _locales_, _options_, « ~coerce-options~ »).
    let descriptors = [ResolutionOptionDescriptor {
        key: "nu",
        property: "numberingSystem",
        r#type: OptionType::String,
        values: None,
    }];
    let service = ServiceConstructor {
        available_locales: available_locales(),
        relevant_extension_keys: &["nu"],
        resolution_option_descriptors: &descriptors,
    };
    let options_resolution = resolve_options(
        agent,
        &service,
        &locale_data,
        locales,
        options,
        &[SpecialBehaviour::CoerceOptions],
        None,
    )?;
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    let options = options_resolution.options;
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    let resolved_locale = options_resolution.resolved_locale;
    // 6. Set _numberFormat_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 7. Set _numberFormat_.[[LocaleData]] to _resolvedLocale_.[[LocaleData]].
    // 8. Set _numberFormat_.[[NumberingSystem]] to _resolvedLocale_.[[nu]].
    let Some(numbering_system) = resolved_locale.get("nu") else {
        return Err(agent.not_implemented(&format!(
            "Intl.NumberFormat: ICU4X has no decimal symbols for {:?}",
            resolved_locale.locale
        )));
    };
    let mut slots =
        NumberFormatSlots::new(resolved_locale.locale.clone(), numbering_system.to_string());
    // 9. Perform ? SetNumberFormatUnitOptions(_numberFormat_, _options_).
    set_number_format_unit_options(agent, &mut slots, options)?;
    // 10. Let _style_ be _numberFormat_.[[Style]].
    let style = slots.style.clone();
    // 11. Let _notation_ be ? GetOption(_options_, *"notation"*, ~string~, « *"standard"*, *"scientific"*, *"engineering"*, *"compact"* », *"standard"*).
    let notation = get_string_option(
        agent,
        options,
        "notation",
        &["standard", "scientific", "engineering", "compact"],
        Some("standard"),
    )?
    .unwrap_or_default();
    // 12. Set _numberFormat_.[[Notation]] to _notation_.
    slots.notation = notation.clone();
    // 13. If _style_ is *"currency"* and _notation_ is *"standard"*, then
    let (mnfd_default, mxfd_default) = if style == "currency" && notation == "standard" {
        // a. Let _currency_ be _numberFormat_.[[Currency]].
        // b. Let _cDigits_ be CurrencyDigits(_currency_).
        // c. Let _mnfdDefault_ be _cDigits_.
        // d. Let _mxfdDefault_ be _cDigits_.
        return Err(
            agent.not_implemented("CurrencyDigits: ICU4X has no currency fraction digit data")
        );
    } else if style == "percent" {
        // 14. Else,
        // a. Let _mnfdDefault_ be 0.
        // b. If _style_ is *"percent"*, then
        //    i. Let _mxfdDefault_ be 0.
        (0.0, 0.0)
    } else {
        // c. Else,
        //    i. Let _mxfdDefault_ be 3.
        (0.0, 3.0)
    };
    // 15. Perform ? SetNumberFormatDigitOptions(_numberFormat_, _options_, _mnfdDefault_, _mxfdDefault_, _notation_).
    set_number_format_digit_options(
        agent,
        &mut slots,
        options,
        mnfd_default,
        mxfd_default,
        &notation,
    )?;
    // 16. Let _compactDisplay_ be ? GetOption(_options_, *"compactDisplay"*, ~string~, « *"short"*, *"long"* », *"short"*).
    let compact_display = get_string_option(
        agent,
        options,
        "compactDisplay",
        &["short", "long"],
        Some("short"),
    )?;
    // 17. Let _defaultUseGrouping_ be *"auto"*.
    let mut default_use_grouping = "auto";
    // 18. If _notation_ is *"compact"*, then
    if notation == "compact" {
        // a. Set _numberFormat_.[[CompactDisplay]] to _compactDisplay_.
        slots.compact_display = compact_display;
        // b. Set _defaultUseGrouping_ to *"min2"*.
        default_use_grouping = "min2";
    }
    // 19. NOTE: For historical reasons, the strings *"true"* and *"false"* are accepted and replaced with the default value.
    // 20. Let _useGrouping_ be ? GetBooleanOrStringNumberFormatOption(_options_, *"useGrouping"*, « *"min2"*, *"auto"*, *"always"*, *"true"*, *"false"* », _defaultUseGrouping_).
    let fallback = Value::String(agent.string(default_use_grouping));
    let use_grouping = get_boolean_or_string_number_format_option(
        agent,
        options,
        "useGrouping",
        &["min2", "auto", "always", "true", "false"],
        fallback,
    )?;
    slots.use_grouping = match use_grouping {
        // 22. If _useGrouping_ is *true*, set _useGrouping_ to *"always"*.
        Value::Boolean(true) => Some("always".to_string()),
        Value::Boolean(_) => None,
        Value::String(s) => {
            let s = agent.str_lossy(s);
            // 21. If _useGrouping_ is *"true"* or _useGrouping_ is *"false"*, set _useGrouping_ to _defaultUseGrouping_.
            if s == "true" || s == "false" {
                Some(default_use_grouping.to_string())
            } else {
                Some(s)
            }
        }
        _ => None,
    };
    // 23. Set _numberFormat_.[[UseGrouping]] to _useGrouping_.
    // 24. Let _signDisplay_ be ? GetOption(_options_, *"signDisplay"*, ~string~, « *"auto"*, *"never"*, *"always"*, *"exceptZero"*, *"negative"* », *"auto"*).
    let sign_display = get_string_option(
        agent,
        options,
        "signDisplay",
        &["auto", "never", "always", "exceptZero", "negative"],
        Some("auto"),
    )?
    .unwrap_or_default();
    // 25. Set _numberFormat_.[[SignDisplay]] to _signDisplay_.
    slots.sign_display = sign_display;
    if let Some(target) = agent.native_mut::<NumberFormatSlots>(number_format) {
        *target = slots;
    }
    // 26. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Let _this_ be the *this* value.
    //    b. Return ? ChainNumberFormat(_numberFormat_, NewTarget, _this_).
    //
    // ChainNumberFormat ( _numberFormat_, _newTarget_, _this_ )
    // 1. If _newTarget_ is *undefined* and ? OrdinaryHasInstance(%Intl.NumberFormat%, _this_) is *true*, then
    if new_target.is_none() {
        let intrinsic = Value::Object(agent.intrinsic("%Intl.NumberFormat%"));
        if ops::ordinary_has_instance(agent, intrinsic, this)? {
            if let Value::Object(this_obj) = this {
                // a. Perform ? DefinePropertyOrThrow(_this_, %Intl%.[[FallbackSymbol]], PropertyDescriptor { [[Value]]: _numberFormat_, [[Writable]]: *false*, [[Enumerable]]: *false*, [[Configurable]]: *false* }).
                let key = PropertyKey::Symbol(fallback_symbol(agent));
                ops::define_property_or_throw(
                    agent,
                    this_obj,
                    key,
                    PropertyDescriptor::data(Value::Object(number_format), false, false, false),
                )?;
                // b. Return _this_.
                return Ok(this);
            }
        }
    }
    // 2. Return _numberFormat_.
    // 27. Return _numberFormat_.
    Ok(Value::Object(number_format))
}
