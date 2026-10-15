//! Intl.DurationFormat ( [ _locales_ [ , _options_ ] ] )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.DurationFormat

use kahvi_core::prelude::*;

/// Intl.DurationFormat
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `Intl.DurationFormat` function is called with optional arguments _locales_ and _options_, the following steps are taken:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _durationFormat_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Intl.DurationFormatPrototype%"*, « [[InitializedDurationFormat]], [[Locale]], [[NumberingSystem]], [[Style]], [[YearsOptions]], [[MonthsOptions]], [[WeeksOptions]], [[DaysOptions]], [[HoursOptions]], [[MinutesOptions]], [[SecondsOptions]], [[MillisecondsOptions]], [[MicrosecondsOptions]], [[NanosecondsOptions]], [[HourMinuteSeparator]], [[MinuteSecondSeparator]], [[FractionalDigits]] »).
    // 3. Let _optionsResolution_ be ? ResolveOptions(%Intl.DurationFormat%, %Intl.DurationFormat%.[[LocaleData]], _locales_, _options_).
    // 4. Set _options_ to _optionsResolution_.[[Options]].
    // 5. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    // 6. Set _durationFormat_.[[Locale]] to _resolvedLocale_.[[Locale]].
    // 7. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
    // 8. Let _digitalFormat_ be _resolvedLocaleData_.[[DigitalFormat]].
    // 9. Set _durationFormat_.[[HourMinuteSeparator]] to _digitalFormat_.[[HourMinuteSeparator]].
    // 10. Set _durationFormat_.[[MinuteSecondSeparator]] to _digitalFormat_.[[MinuteSecondSeparator]].
    // 11. Set _durationFormat_.[[NumberingSystem]] to _resolvedLocale_.[[nu]].
    // 12. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"long"*, *"short"*, *"narrow"*, *"digital"* », *"short"*).
    // 13. Set _durationFormat_.[[Style]] to _style_.
    // 14. Let _prevStyle_ be the empty String.
    // 15. For each row of [table-durationformat], except the header row, in table order, do
    //    a. Let _slot_ be the Internal Slot value of the current row.
    //    b. Let _unit_ be the Unit value of the current row.
    //    c. Let _styles_ be the Styles value of the current row.
    //    d. Let _digitalBase_ be the Digital Default value of the current row.
    //    e. Let _unitOptions_ be ? GetDurationUnitOptions(_unit_, _options_, _style_, _styles_, _digitalBase_, _prevStyle_, _digitalFormat_.[[TwoDigitHours]]).
    //    f. Set the value of _durationFormat_'s internal slot whose name is _slot_ to _unitOptions_.
    //    g. If _unit_ is one of *"hours"*, *"minutes"*, *"seconds"*, *"milliseconds"*, or *"microseconds"*, then
    //       i. Set _prevStyle_ to _unitOptions_.[[Style]].
    // 16. Set _durationFormat_.[[FractionalDigits]] to ? GetNumberOption(_options_, *"fractionalDigits"*, 0, 9, *undefined*).
    // 17. Return _durationFormat_.
    //
    // Internal slots and property names of DurationFormat instances
    // Internal Slot |
    // Unit |
    // Styles |
    // Digital Default |
    // [[YearsOptions]] |
    // *"years"* |
    // « *"long"*, *"short"*, *"narrow"* » |
    // *"short"* |
    // [[MonthsOptions]] |
    // *"months"* |
    // « *"long"*, *"short"*, *"narrow"* » |
    // *"short"* |
    // [[WeeksOptions]] |
    // *"weeks"* |
    // « *"long"*, *"short"*, *"narrow"* » |
    // *"short"* |
    // [[DaysOptions]] |
    // *"days"* |
    // « *"long"*, *"short"*, *"narrow"* » |
    // *"short"* |
    // [[HoursOptions]] |
    // *"hours"* |
    // « *"long"*, *"short"*, *"narrow"*, *"numeric"*, *"2-digit"* » |
    // *"numeric"* |
    // [[MinutesOptions]] |
    // *"minutes"* |
    // « *"long"*, *"short"*, *"narrow"*, *"numeric"*, *"2-digit"* » |
    // *"numeric"* |
    // [[SecondsOptions]] |
    // *"seconds"* |
    // « *"long"*, *"short"*, *"narrow"*, *"numeric"*, *"2-digit"* » |
    // *"numeric"* |
    // [[MillisecondsOptions]] |
    // *"milliseconds"* |
    // « *"long"*, *"short"*, *"narrow"*, *"numeric"* » |
    // *"numeric"* |
    // [[MicrosecondsOptions]] |
    // *"microseconds"* |
    // « *"long"*, *"short"*, *"narrow"*, *"numeric"* » |
    // *"numeric"* |
    // [[NanosecondsOptions]] |
    // *"nanoseconds"* |
    // « *"long"*, *"short"*, *"narrow"*, *"numeric"* » |
    // *"numeric"* |
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.DurationFormat"))
}
