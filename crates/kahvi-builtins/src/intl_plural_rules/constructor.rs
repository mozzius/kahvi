//! Intl.PluralRules ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.pluralrules

use kahvi_core::prelude::*;

/// Intl.PluralRules
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.PluralRules` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _pluralRules_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.PluralRules.prototype%"*, « [[InitializedPluralRules]], [[Locale]], [[Type]], [[Notation]], [[CompactDisplay]], [[MinimumIntegerDigits]], [[MinimumFractionDigits]], [[MaximumFractionDigits]], [[MinimumSignificantDigits]], [[MaximumSignificantDigits]], [[RoundingType]], [[RoundingIncrement]], [[RoundingMode]], [[ComputedRoundingPriority]], [[TrailingZeroDisplay]] »).
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.PluralRules%, %Intl.PluralRules%.[[LocaleData]], _locales_, _options_, « ~coerce-options~ »).
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    // 6. Set _pluralRules_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 7. Let _type_ be ? GetOption(_options_, *"type"*, ~string~, « *"cardinal"*, *"ordinal"* », *"cardinal"*).
    // 8. Set _pluralRules_.[[Type]] to _type_.
    // 9. Let _notation_ be ? GetOption(_options_, *"notation"*, ~string~, « *"standard"*, *"scientific"*, *"engineering"*, *"compact"* », *"standard"*).
    // 10. Set _pluralRules_.[[Notation]] to _notation_.
    // 11. Let _compactDisplay_ be ? GetOption(_options_, *"compactDisplay"*, ~string~, « *"short"*, *"long"* », *"short"*).
    // 12. If _notation_ is *"compact"*, then
    //    a. Set _pluralRules_.[[CompactDisplay]] to _compactDisplay_.
    // 13. Perform ? SetNumberFormatDigitOptions(_pluralRules_, _options_, 0, 3, _notation_).
    // 14. Return _pluralRules_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.PluralRules"))
}
