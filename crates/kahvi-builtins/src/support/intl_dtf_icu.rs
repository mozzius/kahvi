//! Intl.DateTimeFormat over ICU4X: the locale data CreateDateTimeFormat
//! reads (calendars, hour cycles) and the formats it picks.
//!
//! ICU4X does not list a locale's date-time patterns; it builds one from a
//! field set (which fields, a length, a time precision, a zone style). So the
//! format matcher maps the requested components to a field set, lets ICU4X
//! choose the pattern, and reads the DateTime Format Record back out of that
//! pattern: resolvedOptions reports what is actually formatted.

use icu::calendar::AnyCalendarKind;
use icu::calendar::Iso;
use icu::datetime::fieldsets::builder::{DateFields, FieldSetBuilder, ZoneStyle};
use icu::datetime::options::{Length, SubsecondDigits, TimePrecision, YearStyle};
use icu::datetime::preferences::{CalendarAlgorithm, HourCycle, NumberingSystem};
use icu::datetime::{DateTimeFormatter, DateTimeFormatterPreferences};
use icu::locale::Locale;
use icu::locale::extensions::unicode::Value as UValue;
use icu::time::ZonedDateTime;
use icu::time::zone::IanaParser;

use crate::support::intl_date_time_format::FormatRecord;

/// The calendars of %Intl.DateTimeFormat%.[[LocaleData]].[[ca]] (the
/// canonical CLDR type of each ICU4X calendar with one).
pub(crate) const CALENDARS: &[(AnyCalendarKind, &str)] = &[
    (AnyCalendarKind::Gregorian, "gregory"),
    (AnyCalendarKind::Buddhist, "buddhist"),
    (AnyCalendarKind::Chinese, "chinese"),
    (AnyCalendarKind::Coptic, "coptic"),
    (AnyCalendarKind::Dangi, "dangi"),
    (AnyCalendarKind::EthiopianAmeteAlem, "ethioaa"),
    (AnyCalendarKind::Ethiopian, "ethiopic"),
    (AnyCalendarKind::Hebrew, "hebrew"),
    (AnyCalendarKind::Indian, "indian"),
    (AnyCalendarKind::HijriTabularTypeIIFriday, "islamic-civil"),
    (AnyCalendarKind::HijriTabularTypeIIThursday, "islamic-tbla"),
    (AnyCalendarKind::HijriUmmAlQura, "islamic-umalqura"),
    (AnyCalendarKind::Iso, "iso8601"),
    (AnyCalendarKind::Japanese, "japanese"),
    (AnyCalendarKind::Persian, "persian"),
    (AnyCalendarKind::Roc, "roc"),
];

/// The locale without its extensions.
fn base_locale(locale: &str) -> Option<Locale> {
    let mut parsed = Locale::try_from_str(locale).ok()?;
    parsed.extensions = Default::default();
    Some(parsed)
}

/// [[LocaleData]].[[<_locale_>]].[[ca]]: the locale's default calendar first,
/// then the others. *None* when ICU4X has no calendar preference data.
pub(crate) fn calendars_of(locale: &str) -> Option<Vec<Option<String>>> {
    let parsed = base_locale(locale)?;
    let kind = AnyCalendarKind::try_new((&parsed).into()).ok()?;
    let default = CALENDARS.iter().find(|(k, _)| *k == kind)?.1;
    let mut list = vec![Some(default.to_string())];
    list.extend(
        CALENDARS
            .iter()
            .filter(|(_, n)| *n != default)
            .map(|(_, n)| Some(n.to_string())),
    );
    Some(list)
}

/// The resolved values a formatter is built for.
pub(crate) struct Prefs<'a> {
    pub locale: &'a str,
    pub calendar: &'a str,
    pub numbering_system: &'a str,
    /// An [[HourCycle]] value ("h11", "h12" or "h23"), or *None* for the
    /// locale's.
    pub hour_cycle: Option<&'a str>,
}

fn preferences(prefs: &Prefs) -> Option<DateTimeFormatterPreferences> {
    let parsed = base_locale(prefs.locale)?;
    let mut p = DateTimeFormatterPreferences::from(&parsed);
    let value = |s: &str| UValue::try_from_str(s).ok();
    p.calendar_algorithm = Some(CalendarAlgorithm::try_from(&value(prefs.calendar)?).ok()?);
    p.numbering_system = Some(NumberingSystem::try_from(value(prefs.numbering_system)?).ok()?);
    p.hour_cycle = match prefs.hour_cycle {
        None => None,
        Some(hc) => Some(HourCycle::try_from(&value(hc)?).ok()?),
    };
    Some(p)
}

/// The formatter for a field set.
pub(crate) fn formatter(
    prefs: &Prefs,
    fields: &FieldSetBuilder,
) -> Result<DateTimeFormatter<icu::datetime::fieldsets::enums::CompositeFieldSet>, String> {
    let p = preferences(prefs).ok_or_else(|| {
        format!(
            "ICU4X preferences for {:?} (calendar {:?}, numbering system {:?}, hour cycle {:?})",
            prefs.locale, prefs.calendar, prefs.numbering_system, prefs.hour_cycle
        )
    })?;
    let set = fields
        .clone()
        .build_composite()
        .map_err(|e| format!("an ICU4X field set for {fields:?}: {e}"))?;
    DateTimeFormatter::try_new(p, set)
        .map_err(|e| format!("an ICU4X formatter for {fields:?}: {e}"))
}

/// The UTS 35 pattern ICU4X uses for a field set.
fn pattern_of(prefs: &Prefs, fields: &FieldSetBuilder) -> Result<String, String> {
    let formatter = formatter(prefs, fields)?;
    // Any instant gives the pattern; the zone is needed for zone fields.
    let sample = ZonedDateTime::try_strict_from_str(
        "2024-01-15T13:05:09.123+00:00[UTC]",
        Iso,
        IanaParser::new(),
    )
    .map_err(|e| format!("the sample date-time: {e:?}"))?;
    Ok(formatter.format(&sample).pattern().to_string())
}

/// The DateTime Format Record a UTS 35 pattern formats, and the hour cycle
/// its hour field uses.
fn record_of_pattern(pattern: &str) -> (FormatRecord, Option<&'static str>) {
    let mut record = FormatRecord::default();
    let mut hour_cycle = None;
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            // A quoted literal ('' is a quote).
            i += 1;
            while i < chars.len() && chars[i] != '\'' {
                i += 1;
            }
            i += 1;
            continue;
        }
        if !c.is_ascii_alphabetic() {
            i += 1;
            continue;
        }
        let mut n = 1;
        while i + n < chars.len() && chars[i + n] == c {
            n += 1;
        }
        i += n;
        let digits = if n == 2 { "2-digit" } else { "numeric" };
        let text = match n {
            4 => "long",
            5 => "narrow",
            _ => "short",
        };
        let s = |v: &str| Some(v.to_string());
        match c {
            'G' => record.era = s(text),
            'y' | 'u' | 'r' | 'U' => record.year = s(digits),
            'M' | 'L' => {
                record.month = s(match n {
                    1 => "numeric",
                    2 => "2-digit",
                    _ => text,
                })
            }
            'd' => record.day = s(digits),
            'E' | 'c' | 'e' => record.weekday = s(text),
            'b' | 'B' => record.day_period = s(text),
            'h' | 'H' | 'K' | 'k' => {
                record.hour = s(digits);
                hour_cycle = Some(match c {
                    'h' => "h12",
                    'H' => "h23",
                    'K' => "h11",
                    _ => "h24",
                });
            }
            'm' => record.minute = s(digits),
            's' => record.second = s(digits),
            'S' => record.fractional_second_digits = Some(n.min(3) as u8),
            'z' => record.time_zone_name = s(if n == 4 { "long" } else { "short" }),
            'O' => record.time_zone_name = s(if n == 4 { "longOffset" } else { "shortOffset" }),
            'v' => {
                record.time_zone_name = s(if n == 4 {
                    "longGeneric"
                } else {
                    "shortGeneric"
                })
            }
            'V' => record.time_zone_name = s("longGeneric"),
            'Z' => record.time_zone_name = s(if n == 4 { "longOffset" } else { "shortOffset" }),
            'X' | 'x' => record.time_zone_name = s("shortOffset"),
            _ => {}
        }
    }
    (record, hour_cycle)
}

/// [[LocaleData]].[[<_locale_>]] hour cycles: [[hourCycle]], [[hourCycle12]]
/// and [[hourCycle24]], from the patterns ICU4X uses for a time with no hour
/// cycle preference, a 12-hour one and a 24-hour one.
pub(crate) fn hour_cycles_of(locale: &str) -> Option<[&'static str; 3]> {
    let mut fields = FieldSetBuilder::default();
    fields.time_precision = Some(TimePrecision::Minute);
    let cycle = |hc: Option<&str>| {
        let prefs = Prefs {
            locale,
            calendar: "gregory",
            numbering_system: "latn",
            hour_cycle: hc,
        };
        record_of_pattern(&pattern_of(&prefs, &fields).ok()?).1
    };
    Some([cycle(None)?, cycle(Some("h12"))?, cycle(Some("h23"))?])
}

/// The field set for requested components (the best-fit format matcher).
fn field_set_for(options: &FormatRecord) -> Result<FieldSetBuilder, String> {
    let mut fields = FieldSetBuilder::default();
    let (e, y, m, d) = (
        options.weekday.is_some(),
        options.year.is_some() || options.era.is_some(),
        options.month.is_some(),
        options.day.is_some(),
    );
    // The smallest ICU4X date field set with every requested date field.
    fields.date_fields = match (y, m, d, e) {
        (false, false, false, false) => None,
        (false, false, false, true) => Some(DateFields::E),
        (true, false, false, false) => Some(DateFields::Y),
        (false, true, false, false) => Some(DateFields::M),
        (false, false, true, false) => Some(DateFields::D),
        (true, true, false, false) => Some(DateFields::YM),
        (false, true, true, false) => Some(DateFields::MD),
        (true, _, true, false) => Some(DateFields::YMD),
        (false, false, true, true) => Some(DateFields::DE),
        (false, true, _, true) => Some(DateFields::MDE),
        (true, _, _, true) => Some(DateFields::YMDE),
    };
    let texts = [&options.month, &options.weekday, &options.era];
    let has = |v: &str| texts.iter().any(|t| t.as_deref() == Some(v));
    fields.length = Some(if has("long") {
        Length::Long
    } else if has("short") || has("narrow") {
        Length::Medium
    } else {
        Length::Short
    });
    if fields.date_fields.is_some_and(|f| {
        matches!(
            f,
            DateFields::Y | DateFields::YM | DateFields::YMD | DateFields::YMDE
        )
    }) {
        fields.year_style = if options.era.is_some() {
            Some(YearStyle::WithEra)
        } else if options.year.as_deref() == Some("numeric") {
            Some(YearStyle::Full)
        } else {
            None
        };
    }
    fields.time_precision = if let Some(n) = options.fractional_second_digits {
        Some(TimePrecision::Subsecond(match n {
            1 => SubsecondDigits::S1,
            2 => SubsecondDigits::S2,
            _ => SubsecondDigits::S3,
        }))
    } else if options.second.is_some() {
        Some(TimePrecision::Second)
    } else if options.minute.is_some() {
        Some(TimePrecision::Minute)
    } else if options.hour.is_some() {
        Some(TimePrecision::Hour)
    } else if options.day_period.is_some() {
        return Err("a format with a day period and no hour".to_string());
    } else {
        None
    };
    fields.zone_style = options.time_zone_name.as_deref().map(zone_style);
    Ok(fields)
}

fn zone_style(name: &str) -> ZoneStyle {
    match name {
        "long" => ZoneStyle::SpecificLong,
        "short" => ZoneStyle::SpecificShort,
        "longOffset" => ZoneStyle::LocalizedOffsetLong,
        "shortOffset" => ZoneStyle::LocalizedOffsetShort,
        "longGeneric" => ZoneStyle::GenericLong,
        _ => ZoneStyle::GenericShort,
    }
}

/// A chosen format: the record CreateDateTimeFormat stores as
/// [[DateTimeFormat]] and the field set that formats it.
#[derive(Debug, Clone, Default)]
pub(crate) struct ChosenFormat {
    pub record: FormatRecord,
    pub fields: FieldSetBuilder,
}

/// BestFitFormatMatcher ( _options_, _formats_ ), with _formats_ the
/// patterns ICU4X makes for its field sets.
pub(crate) fn best_fit_format_matcher(
    prefs: &Prefs,
    options: &FormatRecord,
) -> Result<ChosenFormat, String> {
    let fields = field_set_for(options)?;
    let (record, _) = record_of_pattern(&pattern_of(prefs, &fields)?);
    Ok(ChosenFormat { record, fields })
}

/// DateTimeStyleFormat ( _dateStyle_, _timeStyle_, _styles_ ), with _styles_
/// the ICU4X field sets for each style: full is a long date with the
/// weekday and a long zone name, long a long date and a short zone name.
pub(crate) fn date_time_style_format(
    prefs: &Prefs,
    date_style: Option<&str>,
    time_style: Option<&str>,
) -> Result<ChosenFormat, String> {
    let length = |style: &str| match style {
        "full" | "long" => Length::Long,
        "medium" => Length::Medium,
        _ => Length::Short,
    };
    let mut fields = FieldSetBuilder::default();
    if let Some(style) = date_style {
        fields.date_fields = Some(if style == "full" {
            DateFields::YMDE
        } else {
            DateFields::YMD
        });
        fields.length = Some(length(style));
    }
    if let Some(style) = time_style {
        fields.time_precision = Some(if style == "short" {
            TimePrecision::Minute
        } else {
            TimePrecision::Second
        });
        fields.zone_style = match style {
            "full" => Some(ZoneStyle::SpecificLong),
            "long" => Some(ZoneStyle::SpecificShort),
            _ => None,
        };
        if fields.length.is_none() {
            fields.length = Some(length(style));
        }
    }
    let (record, _) = record_of_pattern(&pattern_of(prefs, &fields)?);
    Ok(ChosenFormat { record, fields })
}
