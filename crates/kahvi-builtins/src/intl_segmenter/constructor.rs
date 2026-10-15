//! Intl.Segmenter ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.segmenter

use kahvi_core::prelude::*;

use crate::support::intl::{
    ServiceConstructor, available_locales, get_string_option, resolve_options,
};
use crate::support::intl_segmenter::SegmenterSlots;

/// Intl.Segmenter
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.Segmenter` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    let _ = this;
    let locales = args.get_or_undefined(0);
    let options = args.get_or_undefined(1);
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    let Some(new_target) = new_target else {
        return Err(agent.throw_type_error("Constructor Intl.Segmenter requires 'new'"));
    };
    // 2. Let _internalSlotsList_ be « [[InitializedSegmenter]], [[Locale]], [[SegmenterGranularity]] ».
    // 3. Let _segmenter_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.Segmenter.prototype%"*, _internalSlotsList_).
    let segmenter = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Intl.Segmenter.prototype%",
        ObjectKind::Native(Box::new(SegmenterSlots::default())),
    )?;
    // 4. Let _optionsResolution_ be ? ResolveOptions(%Intl.Segmenter%, %Intl.Segmenter%.[[LocaleData]], _locales_, _options_).
    // (%Intl.Segmenter% has no relevant extension keys or resolution option descriptors.)
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
    // 5. Set _options_ to _optionsResolution_.[[Options]].
    let options = options_resolution.options;
    // 6. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    let resolved_locale = options_resolution.resolved_locale;
    // 7. Set _segmenter_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 8. Let _granularity_ be ? GetOption(_options_, *"granularity"*, ~string~, « *"grapheme"*, *"word"*, *"sentence"* », *"grapheme"*).
    let Some(granularity) = get_string_option(
        agent,
        options,
        "granularity",
        &["grapheme", "word", "sentence"],
        Some("grapheme"),
    )?
    else {
        return Err(agent.not_implemented("Intl.Segmenter: granularity option has no default"));
    };
    // 9. Set _segmenter_.[[SegmenterGranularity]] to _granularity_.
    let Some(slots) = agent.native_mut::<SegmenterSlots>(segmenter) else {
        return Err(agent.not_implemented("Intl.Segmenter: lost internal slots"));
    };
    slots.locale = resolved_locale.locale;
    slots.granularity = granularity;
    // 10. Return _segmenter_.
    Ok(Value::Object(segmenter))
}
