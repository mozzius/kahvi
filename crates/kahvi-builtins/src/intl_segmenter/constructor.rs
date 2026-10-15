//! Intl.Segmenter ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.segmenter

use kahvi_core::prelude::*;

/// Intl.Segmenter
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.Segmenter` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _internalSlotsList_ be « [[InitializedSegmenter]], [[Locale]], [[SegmenterGranularity]] ».
    // 3. Let _segmenter_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.Segmenter.prototype%"*, _internalSlotsList_).
    // 4. Let _optionsResolution_ be ? ResolveOptions(%Intl.Segmenter%, %Intl.Segmenter%.[[LocaleData]], _locales_, _options_).
    // 5. Set _options_ to _optionsResolution_.[[Options]].
    // 6. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    // 7. Set _segmenter_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 8. Let _granularity_ be ? GetOption(_options_, *"granularity"*, ~string~, « *"grapheme"*, *"word"*, *"sentence"* », *"grapheme"*).
    // 9. Set _segmenter_.[[SegmenterGranularity]] to _granularity_.
    // 10. Return _segmenter_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.Segmenter"))
}
