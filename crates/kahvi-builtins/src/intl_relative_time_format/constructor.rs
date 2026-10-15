//! Intl.RelativeTimeFormat ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.RelativeTimeFormat

use kahvi_core::prelude::*;

/// Intl.RelativeTimeFormat
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.RelativeTimeFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _relativeTimeFormat_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.RelativeTimeFormat.prototype%"*, « [[InitializedRelativeTimeFormat]], [[Locale]], [[LocaleData]], [[Style]], [[Numeric]], [[NumberFormat]], [[NumberingSystem]], [[PluralRules]] »).
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.RelativeTimeFormat%, %Intl.RelativeTimeFormat%.[[LocaleData]], _locales_, _options_, « ~coerce-options~ »).
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    // 6. Let _locale_ be _resolvedLocale_.[[Locale]].
    // 7. Set _relativeTimeFormat_.[[Locale]] to _locale_.
    // 8. Set _relativeTimeFormat_.[[LocaleData]] to _resolvedLocale_.[[LocaleData]].
    // 9. Set _relativeTimeFormat_.[[NumberingSystem]] to _resolvedLocale_.[[nu]].
    // 10. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"long"*, *"short"*, *"narrow"* », *"long"*).
    // 11. Set _relativeTimeFormat_.[[Style]] to _style_.
    // 12. Let _numeric_ be ? GetOption(_options_, *"numeric"*, ~string~, « *"always"*, *"auto"* », *"always"*).
    // 13. Set _relativeTimeFormat_.[[Numeric]] to _numeric_.
    // 14. Let _nfOptions_ be OrdinaryObjectCreate(*null*).
    // 15. Perform ! CreateDataPropertyOrThrow(_nfOptions_, *"numberingSystem"*, _relativeTimeFormat_.[[NumberingSystem]]).
    // 16. Let _relativeTimeFormat_.[[NumberFormat]] be ! Construct(%Intl.NumberFormat%, « _locale_, _nfOptions_ »).
    // 17. Let _relativeTimeFormat_.[[PluralRules]] be ! Construct(%Intl.PluralRules%, « _locale_ »).
    // 18. Return _relativeTimeFormat_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.RelativeTimeFormat"))
}
