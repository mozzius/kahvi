//! Intl.DisplayNames ( _locales_, _options_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DisplayNames

use kahvi_core::prelude::*;

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
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DisplayNames"))
}
