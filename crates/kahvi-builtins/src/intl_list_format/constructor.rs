//! Intl.ListFormat ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.ListFormat

use kahvi_core::prelude::*;

/// Intl.ListFormat
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.ListFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _listFormat_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.ListFormat.prototype%"*, « [[InitializedListFormat]], [[Locale]], [[Type]], [[Style]], [[Templates]] »).
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.ListFormat%, %Intl.ListFormat%.[[LocaleData]], _locales_, _options_).
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    // 6. Set _listFormat_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 7. Let _type_ be ? GetOption(_options_, *"type"*, ~string~, « *"conjunction"*, *"disjunction"*, *"unit"* », *"conjunction"*).
    // 8. Set _listFormat_.[[Type]] to _type_.
    // 9. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"long"*, *"short"*, *"narrow"* », *"long"*).
    // 10. Set _listFormat_.[[Style]] to _style_.
    // 11. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
    // 12. Let _dataLocaleTypes_ be _resolvedLocaleData_.[[<_type_>]].
    // 13. Set _listFormat_.[[Templates]] to _dataLocaleTypes_.[[<_style_>]].
    // 14. Return _listFormat_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.ListFormat"))
}
