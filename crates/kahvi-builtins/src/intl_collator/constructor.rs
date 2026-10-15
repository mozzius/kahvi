//! Intl.Collator ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.collator

use kahvi_core::prelude::*;

/// Intl.Collator
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.Collator` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object, else let _newTarget_ be NewTarget.
    // 2. Let _internalSlotsList_ be « [[InitializedCollator]], [[Locale]], [[Usage]], [[Collation]], [[Numeric]], [[CaseFirst]], [[Sensitivity]], [[IgnorePunctuation]], [[BoundCompare]] ».
    // 3. Let _collator_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%Intl.Collator.prototype%"*, _internalSlotsList_).
    // 4. NOTE: The source of locale data for ResolveOptions depends upon the *"usage"* property of _options_, but the following two steps must observably precede that lookup (and must not observably repeat inside ResolveOptions).
    // 5. Let _requestedLocales_ be ? CanonicalizeLocaleList(_locales_).
    // 6. Set _options_ to ? CoerceOptionsToObject(_options_).
    // 7. Let _usage_ be ? GetOption(_options_, *"usage"*, ~string~, « *"sort"*, *"search"* », *"sort"*).
    // 8. Set _collator_.[[Usage]] to _usage_.
    // 9. If _usage_ is *"sort"*, then
    //    a. Let _localeData_ be %Intl.Collator%.[[SortLocaleData]].
    // 10. Else,
    //    a. Let _localeData_ be %Intl.Collator%.[[SearchLocaleData]].
    // 11. Let _optionsResolution_ be ? ResolveOptions(%Intl.Collator%, _localeData_, CreateArrayFromList(_requestedLocales_), _options_).
    // 12. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    // 13. Set _collator_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 14. If _resolvedLocale_.[[co]] is *null*, let _collation_ be *"default"*. Otherwise, let _collation_ be _resolvedLocale_.[[co]].
    // 15. Set _collator_.[[Collation]] to _collation_.
    // 16. Set _collator_.[[Numeric]] to SameValue(_resolvedLocale_.[[kn]], *"true"*).
    // 17. Set _collator_.[[CaseFirst]] to _resolvedLocale_.[[kf]].
    // 18. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
    // 19. If _usage_ is *"sort"*, let _defaultSensitivity_ be *"variant"*. Otherwise, let _defaultSensitivity_ be _resolvedLocaleData_.[[sensitivity]].
    // 20. Set _collator_.[[Sensitivity]] to ? GetOption(_options_, *"sensitivity"*, ~string~, « *"base"*, *"accent"*, *"case"*, *"variant"* », _defaultSensitivity_).
    // 21. Let _defaultIgnorePunctuation_ be _resolvedLocaleData_.[[ignorePunctuation]].
    // 22. Set _collator_.[[IgnorePunctuation]] to ? GetOption(_options_, *"ignorePunctuation"*, ~boolean~, ~empty~, _defaultIgnorePunctuation_).
    // 23. Return _collator_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Collator"))
}
