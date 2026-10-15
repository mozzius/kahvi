//! Intl.DateTimeFormat: the [[InitializedDateTimeFormat]] internal slots and the
//! abstract operations shared by its built-ins.
//!
//! The ticket for this service owns this file: define the internal-slot
//! struct here (`ObjectKind::Native`, see docs/INTL.md) and the abstract
//! operations more than one of its built-ins needs.
#![allow(dead_code)] // helpers for the Intl tickets

use std::cell::RefCell;

use icu::time::zone::TimeZone;
use icu::time::zone::iana::IanaParserExtended;
use kahvi_core::gc::{Trace, Tracer};
use kahvi_core::prelude::*;

use crate::support::intl::{
    KeyValue, OptionType, ResolutionOptionDescriptor, ServiceConstructor, SpecialBehaviour,
    available_locales, get_number_option, get_option, get_string_option, resolve_options,
};
use crate::support::intl_dtf_icu::{
    Prefs, best_fit_format_matcher, calendars_of, date_time_style_format, hour_cycles_of,
};
use crate::support::intl_number_format::numbering_systems_of;

/// The rows of [table-datetimeformat-components]: Property and Values.
pub(crate) const COMPONENTS: &[(&str, &[&str])] = &[
    ("weekday", &["narrow", "short", "long"]),
    ("era", &["narrow", "short", "long"]),
    ("year", &["2-digit", "numeric"]),
    ("month", &["2-digit", "numeric", "narrow", "short", "long"]),
    ("day", &["2-digit", "numeric"]),
    ("dayPeriod", &["narrow", "short", "long"]),
    ("hour", &["2-digit", "numeric"]),
    ("minute", &["2-digit", "numeric"]),
    ("second", &["2-digit", "numeric"]),
    ("fractionalSecondDigits", &[]),
    (
        "timeZoneName",
        &[
            "short",
            "long",
            "shortOffset",
            "longOffset",
            "shortGeneric",
            "longGeneric",
        ],
    ),
];

/// The component fields of a DateTime Format Record, in table order. Each is
/// `None` (*undefined*) or the option string; fractionalSecondDigits is kept
/// as its digit count.
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRecord {
    pub weekday: Option<String>,
    pub era: Option<String>,
    pub year: Option<String>,
    pub month: Option<String>,
    pub day: Option<String>,
    pub day_period: Option<String>,
    pub hour: Option<String>,
    pub minute: Option<String>,
    pub second: Option<String>,
    pub fractional_second_digits: Option<u8>,
    pub time_zone_name: Option<String>,
}

impl FormatRecord {
    /// The string value of the component named _property_ (not
    /// fractionalSecondDigits).
    pub(crate) fn get(&self, property: &str) -> Option<&String> {
        match property {
            "weekday" => self.weekday.as_ref(),
            "era" => self.era.as_ref(),
            "year" => self.year.as_ref(),
            "month" => self.month.as_ref(),
            "day" => self.day.as_ref(),
            "dayPeriod" => self.day_period.as_ref(),
            "hour" => self.hour.as_ref(),
            "minute" => self.minute.as_ref(),
            "second" => self.second.as_ref(),
            "timeZoneName" => self.time_zone_name.as_ref(),
            _ => None,
        }
    }

    pub(crate) fn set(&mut self, property: &str, value: Option<String>) {
        match property {
            "weekday" => self.weekday = value,
            "era" => self.era = value,
            "year" => self.year = value,
            "month" => self.month = value,
            "day" => self.day = value,
            "dayPeriod" => self.day_period = value,
            "hour" => self.hour = value,
            "minute" => self.minute = value,
            "second" => self.second = value,
            "timeZoneName" => self.time_zone_name = value,
            _ => {}
        }
    }

    /// Whether the property named _property_ is not *undefined*.
    pub(crate) fn has(&self, property: &str) -> bool {
        if property == "fractionalSecondDigits" {
            self.fractional_second_digits.is_some()
        } else {
            self.get(property).is_some()
        }
    }
}

/// The internal slots of an Intl.DateTimeFormat.
#[derive(Debug, Default)]
pub(crate) struct DateTimeFormatSlots {
    /// [[Locale]]
    pub locale: String,
    /// [[Calendar]]
    pub calendar: String,
    /// [[NumberingSystem]]
    pub numbering_system: String,
    /// [[TimeZone]]
    pub time_zone: String,
    /// [[HourCycle]]
    pub hour_cycle: Option<String>,
    /// [[DateStyle]]
    pub date_style: Option<String>,
    /// [[TimeStyle]]
    pub time_style: Option<String>,
    /// [[DateTimeFormat]]: the component fields of the chosen format.
    pub date_time_format: FormatRecord,
    /// The ICU4X field set that formats [[DateTimeFormat]].
    pub fields: icu::datetime::fieldsets::builder::FieldSetBuilder,
    /// [[BoundFormat]]
    pub bound_format: Option<Object>,
}

impl Trace for DateTimeFormatSlots {
    fn trace(&self, t: &mut Tracer) {
        if let Some(f) = &self.bound_format {
            f.trace(t);
        }
    }
}

const HOUR_CYCLES: &[&str] = &["h11", "h12", "h23", "h24"];

/// %Intl.DateTimeFormat%.[[LocaleData]]: for a locale and relevant extension
/// key, the supported values with the default first (support/intl_dtf_icu.rs).
/// The hour cycles are read in CreateDateTimeFormat; ICU4X makes no h24
/// patterns, so formatting with h24 is not implemented.
/// ResolveLocale only asks for locales in [[AvailableLocales]], which ICU4X
/// has data for; an empty list resolves the key to *null*, which
/// CreateDateTimeFormat reports as not implemented.
fn locale_data(locale: &str, key: &str) -> Vec<Option<String>> {
    match key {
        "ca" => calendars_of(locale).unwrap_or_default(),
        "nu" => numbering_systems_of(locale).unwrap_or_default(),
        // The spec fixes this list; the locale's own hour cycles are
        // [[hourCycle]], [[hourCycle12]] and [[hourCycle24]].
        "hc" => std::iter::once(None)
            .chain(HOUR_CYCLES.iter().map(|h| Some(h.to_string())))
            .collect(),
        _ => Vec::new(),
    }
}

/// Which of the required fields a CreateDateTimeFormat call needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Required {
    Date,
    Time,
    Any,
}

/// The _defaults_ of a CreateDateTimeFormat call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Defaults {
    Date,
    Time,
    All,
}

/// SystemTimeZoneIdentifier ( )
fn system_time_zone_identifier() -> &'static str {
    // 1. If the implementation only supports the UTC time zone, return *"UTC"*.
    "UTC"
}

/// Parses a UTCOffset string (±HH, ±HHMM, ±HH:MM, optionally with seconds).
/// Returns `None` when it is not a UTCOffset, else the number of
/// MinuteSecond components and the total offset in minutes.
fn parse_utc_offset(s: &str) -> Option<(usize, i32)> {
    if !s.is_ascii() {
        return None;
    }
    let b = s.as_bytes();
    let sign = match b.first()? {
        b'+' => 1,
        b'-' => -1,
        _ => return None,
    };
    let rest = &s[1..];
    let digits = |t: &str| t.len() == 2 && t.bytes().all(|c| c.is_ascii_digit());
    let num = |t: &str| t.parse::<i32>().ok();
    let parts: Vec<&str> = if rest.contains(':') {
        rest.split(':').collect()
    } else if rest.len() == 2 {
        vec![rest]
    } else if rest.len() == 4 {
        vec![&rest[..2], &rest[2..]]
    } else if rest.len() == 6 {
        vec![&rest[..2], &rest[2..4], &rest[4..]]
    } else if rest.len() > 7 && matches!(rest.as_bytes()[6], b'.' | b',') {
        let (head, frac) = rest.split_at(6);
        if !frac[1..].bytes().all(|c| c.is_ascii_digit()) || frac.len() > 10 {
            return None;
        }
        vec![&head[..2], &head[2..4], &head[4..]]
    } else {
        return None;
    };
    if parts.len() > 3 || !parts.iter().all(|p| digits(p)) {
        return None;
    }
    let hours = num(parts[0])?;
    let minutes = parts.get(1).map(|p| num(p)).unwrap_or(Some(0))?;
    let seconds = parts.get(2).map(|p| num(p)).unwrap_or(Some(0))?;
    if hours > 23 || minutes > 59 || seconds > 59 {
        return None;
    }
    Some((parts.len().saturating_sub(1), sign * (hours * 60 + minutes)))
}

/// CreateDateTimeFormat ( _newTarget_, _locales_, _options_, _required_, _defaults_ )
pub(crate) fn create_date_time_format(
    agent: &mut Agent,
    new_target: Object,
    locales: Value,
    options: Value,
    required: Required,
    defaults: Defaults,
) -> JsResult<Object> {
    // 1. Let _dateTimeFormat_ be ? OrdinaryCreateFromConstructor(_newTarget_, *"%Intl.DateTimeFormat.prototype%"*, « [[InitializedDateTimeFormat]], [[Locale]], [[Calendar]], [[NumberingSystem]], [[TimeZone]], [[HourCycle]], [[DateStyle]], [[TimeStyle]], [[DateTimeFormat]], [[BoundFormat]] »).
    let date_time_format = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%Intl.DateTimeFormat.prototype%",
        ObjectKind::Native(Box::new(DateTimeFormatSlots::default())),
    )?;
    // 2. Let _specialOptions_ be the Record { [[Hour12]]: *undefined* }.
    let special_hour12: RefCell<KeyValue> = RefCell::new(KeyValue::Undefined);
    // 3. Let _modifyResolutionOptions_ be a new Abstract Closure with parameters (_options_) that captures _specialOptions_ and performs the following steps when called:
    let modify = |opts: &mut crate::support::intl::ResolutionOptions| {
        // a. Set _specialOptions_.[[Hour12]] to _options_.[[hour12]].
        let hour12 = opts.get("hour12").cloned().unwrap_or(KeyValue::Undefined);
        // b. Remove field [[hour12]] from _options_.
        opts.keys.retain(|(k, _)| *k != "hour12");
        // c. If _specialOptions_.[[Hour12]] is not *undefined*, set _options_.[[hc]] to *null*.
        if hour12 != KeyValue::Undefined {
            opts.set("hc", KeyValue::Null);
        }
        *special_hour12.borrow_mut() = hour12;
    };
    // 4. Let _optionsResolution_ be ? ResolveOptions(%Intl.DateTimeFormat%, %Intl.DateTimeFormat%.[[LocaleData]], _locales_, _options_, « ~coerce-options~ », _modifyResolutionOptions_).
    let descriptors = [
        ResolutionOptionDescriptor {
            key: "ca",
            property: "calendar",
            r#type: OptionType::String,
            values: None,
        },
        ResolutionOptionDescriptor {
            key: "nu",
            property: "numberingSystem",
            r#type: OptionType::String,
            values: None,
        },
        ResolutionOptionDescriptor {
            key: "hour12",
            property: "hour12",
            r#type: OptionType::Boolean,
            values: None,
        },
        ResolutionOptionDescriptor {
            key: "hc",
            property: "hourCycle",
            r#type: OptionType::String,
            values: Some(HOUR_CYCLES),
        },
    ];
    let ctor = ServiceConstructor {
        available_locales: available_locales(),
        relevant_extension_keys: &["ca", "hc", "nu"],
        resolution_option_descriptors: &descriptors,
    };
    let options_resolution = resolve_options(
        agent,
        &ctor,
        &locale_data,
        locales,
        options,
        &[SpecialBehaviour::CoerceOptions],
        Some(&modify),
    )?;
    // 5. Let _hour12_ be _specialOptions_.[[Hour12]].
    let hour12 = special_hour12.borrow().clone();
    // 6. Set _options_ to _optionsResolution_.[[Options]].
    let options = options_resolution.options;
    // 7. Let _resolvedLocale_ be _optionsResolution_.[[ResolvedLocale]].
    let resolved_locale = options_resolution.resolved_locale;
    // 8. Set _dateTimeFormat_.[[Locale]] to _resolvedLocale_.[[Locale]].
    let locale = resolved_locale.locale.clone();
    // 9. Let _resolvedCalendar_ be _resolvedLocale_.[[ca]].
    // 10. Set _dateTimeFormat_.[[Calendar]] to _resolvedCalendar_.
    // 11. Set _dateTimeFormat_.[[NumberingSystem]] to _resolvedLocale_.[[nu]].
    // 12. Let _resolvedLocaleData_ be _resolvedLocale_.[[LocaleData]].
    // (Each is *null* only when ICU4X had no data for the locale.)
    let (Some(resolved_calendar), Some(numbering_system), Some(hour_cycles)) = (
        resolved_locale.get("ca").map(str::to_string),
        resolved_locale.get("nu").map(str::to_string),
        hour_cycles_of(&resolved_locale.data_locale),
    ) else {
        return Err(agent.not_implemented(&format!(
            "Intl.DateTimeFormat: ICU4X calendar, numbering system or hour cycle data for {:?}",
            resolved_locale.locale
        )));
    };
    let [locale_hour_cycle, hour_cycle12, hour_cycle24] = hour_cycles;
    // 13. Let _hc_ be _resolvedLocale_.[[hc]].
    let mut hc: Option<String> = resolved_locale.get("hc").map(str::to_string);
    // 14. If _hour12_ is *true*, then
    if hour12 == KeyValue::String("true".to_string()) {
        // a. Set _hc_ to _resolvedLocaleData_.[[hourCycle12]].
        hc = Some(hour_cycle12.to_string());
    } else if hour12 == KeyValue::String("false".to_string()) {
        // 15. Else if _hour12_ is *false*, then
        // a. Set _hc_ to _resolvedLocaleData_.[[hourCycle24]].
        hc = Some(hour_cycle24.to_string());
    } else if hc.is_none() {
        // 16. Else if _hc_ is *null*, then
        // a. Set _hc_ to _resolvedLocaleData_.[[hourCycle]].
        hc = Some(locale_hour_cycle.to_string());
    }
    // 17. Let _timeZone_ be ? GetOption(_options_, *"timeZone"*, ~string~, ~empty~, SystemTimeZoneIdentifier()).
    let default_zone = agent.string(system_time_zone_identifier());
    let time_zone = get_option(
        agent,
        options,
        "timeZone",
        OptionType::String,
        None,
        Some(Value::String(default_zone)),
    )?;
    let Value::String(time_zone) = time_zone else {
        return Err(agent.throw_type_error("timeZone must be a string"));
    };
    // Lossy is exact here: no time zone identifier has a lone surrogate, and
    // U+FFFD matches none either.
    let time_zone = agent.str_lossy(time_zone);
    // 18. If IsTimeZoneOffsetString(_timeZone_) is *true*, then
    let time_zone = if let Some((minute_seconds, offset_minutes)) = parse_utc_offset(&time_zone) {
        // a. Let _parseResult_ be ParseText(StringToCodePoints(_timeZone_), |UTCOffset|).
        // b. Assert: _parseResult_ is a Parse Node.
        // c. If _parseResult_ contains more than one |MinuteSecond| Parse Node, throw a *RangeError* exception.
        if minute_seconds > 1 {
            return Err(agent.throw_range_error("time zone offsets with seconds are not supported"));
        }
        // d. Let _offsetNanoseconds_ be ParseTimeZoneOffsetString(_timeZone_).
        // e. Let _offsetMinutes_ be _offsetNanoseconds_ / (6 × 10^10).
        // f. Assert: _offsetMinutes_ is an integer.
        // g. Set _timeZone_ to FormatOffsetTimeZoneIdentifier(_offsetMinutes_).
        format_offset_time_zone_identifier(offset_minutes)
    } else {
        // 19. Else,
        // a. Let _timeZoneIdentifierRecord_ be GetAvailableNamedTimeZoneIdentifier(_timeZone_).
        // b. If _timeZoneIdentifierRecord_ is ~empty~, throw a *RangeError* exception.
        // c. Set _timeZone_ to _timeZoneIdentifierRecord_.[[PrimaryIdentifier]].
        match get_available_named_time_zone_identifier(&time_zone) {
            Some(primary) => primary,
            None => return Err(agent.throw_range_error("invalid time zone")),
        }
    };
    // 20. Set _dateTimeFormat_.[[TimeZone]] to _timeZone_.
    // 21. Let _formatOptions_ be a new Record.
    // 22. Set _formatOptions_.[[hourCycle]] to _hc_.
    let mut format_options = FormatRecord::default();
    // 23. Let _hasExplicitFormatComponents_ be *false*.
    let mut has_explicit_format_components = false;
    // 24. For each row of [table-datetimeformat-components], except the header row, in table order, do
    for (property_key, values) in COMPONENTS {
        // a. Let _propertyKey_ be the name given in the Property column of the current row.
        if *property_key == "fractionalSecondDigits" {
            // b. If _propertyKey_ is *"fractionalSecondDigits"*, then
            // i. Let _value_ be ? GetNumberOption(_options_, *"fractionalSecondDigits"*, 1, 3, *undefined*).
            let value =
                get_number_option(agent, options, "fractionalSecondDigits", 1.0, 3.0, None)?;
            // d. Set _formatOptions_.[[<_propertyKey_>]] to _value_.
            format_options.fractional_second_digits = value.map(|v| v as u8);
            // e. If _value_ is not *undefined*, set _hasExplicitFormatComponents_ to *true*.
            if value.is_some() {
                has_explicit_format_components = true;
            }
        } else {
            // c. Else,
            // i. Let _values_ be a List whose elements are the strings given in the Values column of the current row.
            // ii. Let _value_ be ? GetOption(_options_, _propertyKey_, ~string~, _values_, *undefined*).
            let value = get_string_option(agent, options, property_key, values, None)?;
            // e. If _value_ is not *undefined*, set _hasExplicitFormatComponents_ to *true*.
            if value.is_some() {
                has_explicit_format_components = true;
            }
            // d. Set _formatOptions_.[[<_propertyKey_>]] to _value_.
            format_options.set(property_key, value);
        }
    }
    // 25. Let _formatMatcher_ be ? GetOption(_options_, *"formatMatcher"*, ~string~, « *"basic"*, *"best fit"* », *"best fit"*).
    let format_matcher = get_string_option(
        agent,
        options,
        "formatMatcher",
        &["basic", "best fit"],
        Some("best fit"),
    )?;
    // 26. Let _dateStyle_ be ? GetOption(_options_, *"dateStyle"*, ~string~, « *"full"*, *"long"*, *"medium"*, *"short"* », *undefined*).
    let styles = &["full", "long", "medium", "short"];
    let date_style = get_string_option(agent, options, "dateStyle", styles, None)?;
    // 27. Set _dateTimeFormat_.[[DateStyle]] to _dateStyle_.
    // 28. Let _timeStyle_ be ? GetOption(_options_, *"timeStyle"*, ~string~, « *"full"*, *"long"*, *"medium"*, *"short"* », *undefined*).
    let time_style = get_string_option(agent, options, "timeStyle", styles, None)?;
    // 29. Set _dateTimeFormat_.[[TimeStyle]] to _timeStyle_.
    // 30. If _dateStyle_ is not *undefined* or _timeStyle_ is not *undefined*, then
    let prefs = Prefs {
        locale: &resolved_locale.data_locale,
        calendar: &resolved_calendar,
        numbering_system: &numbering_system,
        hour_cycle: hc.as_deref().filter(|h| *h != "h24"),
    };
    let best_format = if date_style.is_some() || time_style.is_some() {
        // a. If _hasExplicitFormatComponents_ is *true*, throw a *TypeError* exception.
        if has_explicit_format_components {
            return Err(agent.throw_type_error(
                "dateStyle and timeStyle cannot be combined with other format options",
            ));
        }
        // b. If _required_ is ~date~ and _timeStyle_ is not *undefined*, throw a *TypeError* exception.
        if required == Required::Date && time_style.is_some() {
            return Err(agent.throw_type_error("timeStyle is not allowed here"));
        }
        // c. If _required_ is ~time~ and _dateStyle_ is not *undefined*, throw a *TypeError* exception.
        if required == Required::Time && date_style.is_some() {
            return Err(agent.throw_type_error("dateStyle is not allowed here"));
        }
        // d. Let _styles_ be _resolvedLocaleData_.[[styles]].[[<_resolvedCalendar_>]].
        // e. Let _bestFormat_ be DateTimeStyleFormat(_dateStyle_, _timeStyle_, _styles_).
        date_time_style_format(&prefs, date_style.as_deref(), time_style.as_deref())
            .map_err(|what| agent.not_implemented(&format!("DateTimeStyleFormat: {what}")))?
    } else {
        // 31. Else,
        // a. Let _needDefaults_ be *true*.
        let mut need_defaults = true;
        // b. Let _propertyKeys_ be a new empty List.
        let mut property_keys: Vec<&str> = Vec::new();
        // c. If _required_ is ~date~ or ~any~, set _propertyKeys_ to the list-concatenation of _propertyKeys_ and « *"weekday"*, *"year"*, *"month"*, *"day"* ».
        if matches!(required, Required::Date | Required::Any) {
            property_keys.extend(["weekday", "year", "month", "day"]);
        }
        // d. If _required_ is ~time~ or ~any~, set _propertyKeys_ to the list-concatenation of _propertyKeys_ and « *"dayPeriod"*, *"hour"*, *"minute"*, *"second"*, *"fractionalSecondDigits"* ».
        if matches!(required, Required::Time | Required::Any) {
            property_keys.extend([
                "dayPeriod",
                "hour",
                "minute",
                "second",
                "fractionalSecondDigits",
            ]);
        }
        // e. For each property name _propertyKey_ of _propertyKeys_, do
        for property_key in property_keys {
            // i. Let _value_ be _formatOptions_.[[<_propertyKey_>]].
            // ii. If _value_ is not *undefined*, set _needDefaults_ to *false*.
            if format_options.has(property_key) {
                need_defaults = false;
            }
        }
        // f. If _needDefaults_ is *true*, then
        if need_defaults {
            // i. Let _numericProperties_ be a new empty List.
            let mut numeric_properties: Vec<&str> = Vec::new();
            // ii. If _defaults_ is either ~date~ or ~all~, set _numericProperties_ to the list-concatenation of _numericProperties_ and « *"year"*, *"month"*, *"day"* ».
            if matches!(defaults, Defaults::Date | Defaults::All) {
                numeric_properties.extend(["year", "month", "day"]);
            }
            // iii. If _defaults_ is either ~time~ or ~all~, set _numericProperties_ to the list-concatenation of _numericProperties_ and « *"hour"*, *"minute"*, *"second"* ».
            if matches!(defaults, Defaults::Time | Defaults::All) {
                numeric_properties.extend(["hour", "minute", "second"]);
            }
            // iv. For each property name _propertyKey_ of _numericProperties_, do
            for property_key in numeric_properties {
                // 1. Set _formatOptions_.[[<_propertyKey_>]] to *"numeric"*.
                format_options.set(property_key, Some("numeric".to_string()));
            }
        }
        // g. Let _formats_ be _resolvedLocaleData_.[[formats]].[[<_resolvedCalendar_>]].
        // h. If _formatMatcher_ is *"basic"*, let _bestFormat_ be BasicFormatMatcher(_formatOptions_, _formats_); else let _bestFormat_ be BestFitFormatMatcher(_formatOptions_, _formats_).
        if format_matcher.as_deref() == Some("basic") {
            // [[formats]] would be every pattern ICU4X can make, which it
            // does not list.
            return Err(agent.not_implemented("Intl.DateTimeFormat BasicFormatMatcher"));
        }
        best_fit_format_matcher(&prefs, &format_options)
            .map_err(|what| agent.not_implemented(&format!("BestFitFormatMatcher: {what}")))?
    };
    // 32. Set _dateTimeFormat_.[[DateTimeFormat]] to _bestFormat_.
    // 33. If _bestFormat_ has a field [[hour]], set _dateTimeFormat_.[[HourCycle]] to _hc_.
    let has_hour = best_format.record.hour.is_some();
    let Some(slots) = agent.native_mut::<DateTimeFormatSlots>(date_time_format) else {
        return Err(agent.not_implemented("Intl.DateTimeFormat internal slots"));
    };
    slots.locale = locale;
    slots.calendar = resolved_calendar;
    slots.numbering_system = numbering_system;
    slots.time_zone = time_zone;
    slots.date_style = date_style;
    slots.time_style = time_style;
    slots.date_time_format = best_format.record;
    slots.fields = best_format.fields;
    if has_hour {
        slots.hour_cycle = hc;
    }
    // 34. Return _dateTimeFormat_.
    Ok(date_time_format)
}

/// FormatOffsetTimeZoneIdentifier ( _offsetMinutes_ )
fn format_offset_time_zone_identifier(offset_minutes: i32) -> String {
    // 1. If _offsetMinutes_ ≥ 0, let _sign_ be the code unit 0x002B (PLUS SIGN); otherwise, let _sign_ be the code unit 0x002D (HYPHEN-MINUS).
    let sign = if offset_minutes >= 0 { '+' } else { '-' };
    // 2. Let _absoluteMinutes_ be abs(_offsetMinutes_).
    let absolute_minutes = offset_minutes.abs();
    // 3. Let _hours_ be floor(_absoluteMinutes_ / 60).
    let hours = absolute_minutes / 60;
    // 4. Let _minutes_ be _absoluteMinutes_ modulo 60.
    let minutes = absolute_minutes % 60;
    // 5. Return the string-concatenation of _sign_, ToZeroPaddedDecimalString(_hours_, 2), the code unit 0x003A (COLON), and ToZeroPaddedDecimalString(_minutes_, 2).
    format!("{sign}{hours:02}:{minutes:02}")
}

/// GetAvailableNamedTimeZoneIdentifier ( _timeZoneIdentifier_ ): the
/// [[PrimaryIdentifier]] of the matching record, or `None` for ~empty~.
fn get_available_named_time_zone_identifier(time_zone_identifier: &str) -> Option<String> {
    // 1. For each element _record_ of AvailableNamedTimeZoneIdentifiers(), do
    //    a. If _record_.[[Identifier]] is an ASCII-case-insensitive match for _timeZoneIdentifier_, return _record_.
    // 2. Return ~empty~.
    let parsed = IanaParserExtended::new().parse(time_zone_identifier);
    if parsed.time_zone == TimeZone::UNKNOWN {
        return None;
    }
    // "UTC" is the primary identifier of Etc/UTC, Etc/GMT, GMT and the
    // other links to UTC (AvailableNamedTimeZoneIdentifiers step 3.b).
    if parsed.time_zone == icu::time::TimeZoneInfo::utc().id() {
        return Some("UTC".to_string());
    }
    Some(parsed.canonical.to_string())
}

/// UnwrapDateTimeFormat ( _dtf_ )
pub(crate) fn unwrap_date_time_format(agent: &mut Agent, dtf: Value) -> JsResult<Value> {
    // 1. If _dtf_ is not an Object, throw a *TypeError* exception.
    let Value::Object(object) = dtf else {
        return Err(agent.throw_type_error("not an Intl.DateTimeFormat"));
    };
    // 2. If _dtf_ does not have an [[InitializedDateTimeFormat]] internal slot and ? OrdinaryHasInstance(%Intl.DateTimeFormat%, _dtf_) is *true*, then
    if agent.native::<DateTimeFormatSlots>(object).is_none() {
        let ctor = agent.intrinsic("%Intl.DateTimeFormat%");
        if ops::ordinary_has_instance(agent, Value::Object(ctor), dtf)? {
            // a. Return ? Get(_dtf_, %Intl%.[[FallbackSymbol]]).
            let symbol = crate::support::intl::fallback_symbol(agent);
            return ops::get(agent, object, symbol.into());
        }
    }
    // 3. Return _dtf_.
    Ok(dtf)
}
