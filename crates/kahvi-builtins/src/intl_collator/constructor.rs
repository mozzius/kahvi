//! Intl.Collator ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.collator

use kahvi_core::prelude::*;

use crate::support::intl::{
    OptionType, ResolutionOptionDescriptor, ServiceConstructor, available_locales,
    canonicalize_locale_list, coerce_options_to_object, get_option, get_string_option,
    resolve_options,
};
use crate::support::intl_collator::{
    CollatorSlots, collations_of, create_icu_collator, default_sensitivity, has_collation,
    locale_defaults,
};

/// [[ResolutionOptionDescriptors]] of %Intl.Collator%.
const RESOLUTION_OPTION_DESCRIPTORS: &[ResolutionOptionDescriptor] = &[
    ResolutionOptionDescriptor {
        key: "co",
        property: "collation",
        r#type: OptionType::String,
        values: None,
    },
    ResolutionOptionDescriptor {
        key: "kn",
        property: "numeric",
        r#type: OptionType::Boolean,
        values: None,
    },
    ResolutionOptionDescriptor {
        key: "kf",
        property: "caseFirst",
        r#type: OptionType::String,
        values: Some(&["upper", "lower", "false"]),
    },
];

/// Intl.Collator
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = this;
    let locales = args.get_or_undefined(0);
    let options = args.get_or_undefined(1);
    // When the `Intl.Collator` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object, else let _newTarget_ be NewTarget.
    let new_target = match new_target {
        Some(n) => n,
        None => agent.active_function().expect("a running built-in"),
    };
    // 2. Let _internalSlotsList_ be « [[InitializedCollator]], [[Locale]], [[Usage]], [[Collation]], [[Numeric]], [[CaseFirst]], [[Sensitivity]], [[IgnorePunctuation]], [[BoundCompare]] ».
    // 3. Let _collator_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%Intl.Collator.prototype%"*, _internalSlotsList_).
    // (The slots are filled in below: the object's proto lookup comes first.)
    let collator = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Intl.Collator.prototype%",
        ObjectKind::Ordinary,
    )?;
    // 4. NOTE: The source of locale data for ResolveOptions depends upon the *"usage"* property of _options_, but the following two steps must observably precede that lookup (and must not observably repeat inside ResolveOptions).
    // 5. Let _requestedLocales_ be ? CanonicalizeLocaleList(_locales_).
    let requested_locales = canonicalize_locale_list(agent, locales)?;
    // 6. Set _options_ to ? CoerceOptionsToObject(_options_).
    let options = coerce_options_to_object(agent, options)?;
    // 7. Let _usage_ be ? GetOption(_options_, *"usage"*, ~string~, « *"sort"*, *"search"* », *"sort"*).
    let usage = get_string_option(agent, options, "usage", &["sort", "search"], Some("sort"))?
        .expect("a default");
    // 8. Set _collator_.[[Usage]] to _usage_.
    // 9. If _usage_ is *"sort"*, then
    //    a. Let _localeData_ be %Intl.Collator%.[[SortLocaleData]].
    // 10. Else,
    //    a. Let _localeData_ be %Intl.Collator%.[[SearchLocaleData]].
    // Both have the same values for the relevant extension keys: a search
    // collator is the locale's "search" collation (below), and the spec keeps
    // "search" out of [[co]].
    let locale_data = |locale: &str, key: &str| -> Vec<Option<String>> {
        match key {
            "co" => collations_of(locale),
            "kn" => {
                let (_, numeric, _) = locale_defaults(locale, None);
                let (first, second) = if numeric {
                    ("true", "false")
                } else {
                    ("false", "true")
                };
                vec![Some(first.to_string()), Some(second.to_string())]
            }
            "kf" => {
                let (case_first, _, _) = locale_defaults(locale, None);
                let mut list = vec![Some(case_first.clone())];
                list.extend(
                    ["false", "lower", "upper"]
                        .iter()
                        .filter(|v| **v != case_first)
                        .map(|v| Some(v.to_string())),
                );
                list
            }
            _ => Vec::new(),
        }
    };
    // 11. Let _optionsResolution_ be ? ResolveOptions(%Intl.Collator%, _localeData_, CreateArrayFromList(_requestedLocales_), _options_).
    let requested: Vec<Value> = requested_locales
        .iter()
        .map(|l| Value::String(agent.string(l)))
        .collect();
    let requested = ops::create_array_from_list(agent, &requested);
    let ctor = ServiceConstructor {
        available_locales: available_locales(),
        relevant_extension_keys: &["co", "kf", "kn"],
        resolution_option_descriptors: RESOLUTION_OPTION_DESCRIPTORS,
    };
    let options_resolution = resolve_options(
        agent,
        &ctor,
        &locale_data,
        Value::Object(requested),
        Value::Object(options),
        &[],
        None,
    )?;
    // 12. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    let resolved_locale = options_resolution.resolved_locale;
    // 13. Set _collator_.[[Locale]] to _resolvedLocale_.[[Locale]].
    let locale = resolved_locale.locale.clone();
    // 14. If _resolvedLocale_.[[co]] is *null*, let _collation_ be *"default"*. Otherwise, let _collation_ be _resolvedLocale_.[[co]].
    // 15. Set _collator_.[[Collation]] to _collation_.
    let collation = resolved_locale.get("co").unwrap_or("default").to_string();
    // 16. Set _collator_.[[Numeric]] to SameValue(_resolvedLocale_.[[kn]], *"true"*).
    let numeric = resolved_locale.get("kn") == Some("true");
    // 17. Set _collator_.[[CaseFirst]] to _resolvedLocale_.[[kf]].
    let case_first = resolved_locale.get("kf").unwrap_or("false").to_string();
    // 18. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
    let (_, _, default_ignore_punctuation) = locale_defaults(
        &resolved_locale.data_locale,
        (collation != "default").then_some(collation.as_str()),
    );
    // 19. If _usage_ is *"sort"*, let _defaultSensitivity_ be *"variant"*. Otherwise, let _defaultSensitivity_ be _resolvedLocaleData_.[[sensitivity]].
    // [[SearchLocaleData]] is ICU4X's "search" collation for the locale.
    let search = usage != "sort";
    if search && !has_collation(&resolved_locale.data_locale, "search") {
        return Err(agent.not_implemented(
            "Intl.Collator usage \"search\" for a locale ICU4X has no search collation for",
        ));
    }
    let default_sensitivity = if search {
        let Some(s) = default_sensitivity(&resolved_locale.data_locale, Some("search")) else {
            return Err(agent.not_implemented(
                "Intl.Collator usage \"search\": ICU4X could not load the search collation",
            ));
        };
        s
    } else {
        "variant"
    };
    // 20. Set _collator_.[[Sensitivity]] to ? GetOption(_options_, *"sensitivity"*, ~string~, « *"base"*, *"accent"*, *"case"*, *"variant"* », _defaultSensitivity_).
    let sensitivity = get_string_option(
        agent,
        options,
        "sensitivity",
        &["base", "accent", "case", "variant"],
        Some(default_sensitivity),
    )?
    .expect("a default");
    // 21. Let _defaultIgnorePunctuation_ be _resolvedLocaleData_.[[ignorePunctuation]].
    // 22. Set _collator_.[[IgnorePunctuation]] to ? GetOption(_options_, *"ignorePunctuation"*, ~boolean~, ~empty~, _defaultIgnorePunctuation_).
    let ignore_punctuation = get_option(
        agent,
        options,
        "ignorePunctuation",
        OptionType::Boolean,
        None,
        Some(Value::Boolean(default_ignore_punctuation)),
    )?;
    let ignore_punctuation = matches!(ignore_punctuation, Value::Boolean(true));
    let Some(icu_collator) = create_icu_collator(
        &resolved_locale.data_locale,
        if search { "search" } else { &collation },
        numeric,
        &case_first,
        &sensitivity,
        ignore_punctuation,
    ) else {
        return Err(agent.not_implemented("Intl.Collator for a locale ICU4X has no collation for"));
    };
    agent.obj_mut(collator).kind = ObjectKind::Native(Box::new(CollatorSlots {
        locale,
        usage,
        collation,
        numeric,
        case_first,
        sensitivity,
        ignore_punctuation,
        bound_compare: None,
        collator: icu_collator,
    }));
    // 23. Return _collator_.
    Ok(Value::Object(collator))
}
