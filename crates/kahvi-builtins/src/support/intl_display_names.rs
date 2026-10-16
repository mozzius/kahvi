//! Intl.DisplayNames: the [[InitializedDisplayNames]] internal slots and the
//! abstract operations shared by its built-ins.
//!
//! The ticket for this service owns this file: define the internal-slot
//! struct here (`ObjectKind::Native`, see docs/INTL.md) and the abstract
//! operations more than one of its built-ins needs.
#![allow(dead_code)] // helpers for the Intl tickets

use icu::experimental::displaynames::{
    DisplayNamesOptions, Fallback, LanguageDisplay, Style,
    multi::{
        LanguageDisplayNames, LocaleDisplayNamesFormatter, RegionDisplayNames, ScriptDisplayNames,
    },
};
use icu::locale::Locale;
use icu::locale::subtags::{Region, Script};
use kahvi_core::gc::{Trace, Tracer};
use kahvi_core::prelude::*;

use super::intl::{canonicalize_unicode_locale_id, is_unicode_type, is_well_formed_language_tag};

/// The internal slots of an Intl.DisplayNames: [[InitializedDisplayNames]],
/// [[Locale]], [[Style]], [[Type]], [[Fallback]], [[LanguageDisplay]].
///
/// [[Fields]] is not stored as a table: the names come from ICU4X data for
/// [[Locale]] when `of` is called.
#[derive(Debug, Clone, Default)]
pub(crate) struct DisplayNamesSlots {
    /// [[Locale]]
    pub locale: String,
    /// [[Style]]: "narrow", "short" or "long".
    pub style: String,
    /// [[Type]]
    pub r#type: String,
    /// [[Fallback]]: "code" or "none".
    pub fallback: String,
    /// [[LanguageDisplay]]: "dialect" or "standard" (only for type "language").
    pub language_display: Option<String>,
}

impl Trace for DisplayNamesSlots {
    fn trace(&self, _: &mut Tracer) {}
}

/// IsValidDateTimeFieldCode ( _field_ )
pub(crate) fn is_valid_date_time_field_code(field: &str) -> bool {
    // 1. If _field_ is listed in the Code column of [table-validcodefordatetimefield], return *true*.
    // 2. Return *false*.
    matches!(
        field,
        "era"
            | "year"
            | "quarter"
            | "month"
            | "weekOfYear"
            | "weekday"
            | "day"
            | "dayPeriod"
            | "hour"
            | "minute"
            | "second"
            | "timeZoneName"
    )
}

/// IsWellFormedCurrencyCode ( _currency_ )
pub(crate) fn is_well_formed_currency_code(currency: &str) -> bool {
    // 1. If the length of _currency_ is not 3, return *false*.
    // 2. Let _normalized_ be the ASCII-uppercase of _currency_.
    // 3. If _normalized_ contains any code unit outside of 0x0041 through 0x005A, return *false*.
    // 4. Return *true*.
    currency.len() == 3 && currency.bytes().all(|b| b.is_ascii_alphabetic())
}

/// Whether _code_ is matched by the unicode_language_id nonterminal of
/// UTS #35 section 3 (which IsWellFormedLanguageTag then narrows to the
/// BCP 47 conformant forms):
/// `"root" | (language (sep script)? | script) (sep region)? (sep variant)*`.
fn is_unicode_language_id(code: &str) -> bool {
    let alpha = |s: &str, lens: &[usize]| {
        lens.contains(&s.len()) && s.bytes().all(|b| b.is_ascii_alphabetic())
    };
    let is_script = |s: &str| alpha(s, &[4]);
    let is_region =
        |s: &str| alpha(s, &[2]) || (s.len() == 3 && s.bytes().all(|b| b.is_ascii_digit()));
    let is_variant = |s: &str| {
        let alnum = s.bytes().all(|b| b.is_ascii_alphanumeric());
        alnum && ((5..=8).contains(&s.len()) || (s.len() == 4 && s.as_bytes()[0].is_ascii_digit()))
    };
    if code.eq_ignore_ascii_case("root") {
        return true;
    }
    let mut parts = code.split('-').peekable();
    let Some(first) = parts.next() else {
        return false;
    };
    if alpha(first, &[2, 3, 5, 6, 7, 8]) {
        // unicode_language_subtag (sep unicode_script_subtag)?
        if parts.peek().is_some_and(|s| is_script(s)) {
            parts.next();
        }
    } else if !is_script(first) {
        return false;
    }
    if parts.peek().is_some_and(|s| is_region(s)) {
        parts.next();
    }
    parts.all(is_variant)
}

/// CanonicalCodeForDisplayNames ( _type_, _code_ )
pub(crate) fn canonical_code_for_display_names(
    agent: &mut Agent,
    r#type: &str,
    code: &str,
) -> JsResult<String> {
    match r#type {
        // 1. If _type_ is *"language"*, then
        "language" => {
            // a. If _code_ cannot be matched by the unicode_language_id Unicode locale nonterminal, throw a *RangeError* exception.
            if !is_unicode_language_id(code) {
                return Err(agent.throw_range_error("invalid language code"));
            }
            // b. If IsWellFormedLanguageTag(_code_) is *false*, throw a *RangeError* exception.
            if !is_well_formed_language_tag(code) {
                // ICU4X, which parses the tag, has only 2-3 letter language subtags.
                let language = code.split('-').next().unwrap_or_default();
                if (5..=8).contains(&language.len()) {
                    return Err(agent
                        .not_implemented("language subtags of 5-8 letters (ICU4X supports 2-3)"));
                }
                return Err(agent.throw_range_error("invalid language code"));
            }
            // c. Return CanonicalizeUnicodeLocaleId(_code_).
            Ok(canonicalize_unicode_locale_id(code))
        }
        // 2. If _type_ is *"region"*, then
        "region" => {
            // a. If _code_ cannot be matched by the unicode_region_subtag Unicode locale nonterminal, throw a *RangeError* exception.
            if Region::try_from_str(code).is_err() {
                return Err(agent.throw_range_error("invalid region code"));
            }
            // b. Return the ASCII-uppercase of _code_.
            Ok(code.to_ascii_uppercase())
        }
        // 3. If _type_ is *"script"*, then
        "script" => {
            // a. If _code_ cannot be matched by the unicode_script_subtag Unicode locale nonterminal, throw a *RangeError* exception.
            if Script::try_from_str(code).is_err() {
                return Err(agent.throw_range_error("invalid script code"));
            }
            // b. Assert: The length of _code_ is 4, and every code unit of _code_ represents an ASCII letter.
            // c. Let _first_ be the ASCII-uppercase of the substring of _code_ from 0 to 1.
            // d. Let _rest_ be the ASCII-lowercase of the substring of _code_ from 1.
            let (first, rest) = code.split_at(1);
            // e. Return the string-concatenation of _first_ and _rest_.
            Ok(format!(
                "{}{}",
                first.to_ascii_uppercase(),
                rest.to_ascii_lowercase()
            ))
        }
        // 4. If _type_ is *"calendar"*, then
        "calendar" => {
            // a. If _code_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
            if !is_unicode_type(code) {
                return Err(agent.throw_range_error("invalid calendar code"));
            }
            // b. If _code_ uses any of the backwards compatibility syntax described in Unicode Technical Standard #35 Part 1 Core, Section 3.3 BCP 47 Conformance, throw a *RangeError* exception.
            // (Of that syntax only the "_" separator could appear in a `type`, and the `type` grammar above already excludes it.)
            // c. Return the ASCII-lowercase of _code_.
            Ok(code.to_ascii_lowercase())
        }
        // 5. If _type_ is *"dateTimeField"*, then
        "dateTimeField" => {
            // a. If the result of IsValidDateTimeFieldCode(_code_) is *false*, throw a *RangeError* exception.
            if !is_valid_date_time_field_code(code) {
                return Err(agent.throw_range_error("invalid dateTimeField code"));
            }
            // b. Return _code_.
            Ok(code.to_string())
        }
        // 6. Assert: _type_ is *"currency"*.
        // 7. If IsWellFormedCurrencyCode(_code_) is *false*, throw a *RangeError* exception.
        _ => {
            if !is_well_formed_currency_code(code) {
                return Err(agent.throw_range_error("invalid currency code"));
            }
            // 8. Return the ASCII-uppercase of _code_.
            Ok(code.to_ascii_uppercase())
        }
    }
}

/// Looks up the name of the canonical _code_ in the ICU4X data for the
/// [[Locale]] of _slots_ (the lookup of [[Fields]].[[<code>]]). `Ok(None)` is
/// "no such field"; missing data throws not-implemented.
pub(crate) fn lookup_display_name(
    agent: &mut Agent,
    slots: &DisplayNamesSlots,
    code: &str,
) -> JsResult<Option<String>> {
    let Ok(locale) = Locale::try_from_str(&slots.locale) else {
        return Err(agent.throw_range_error("invalid locale"));
    };
    let mut options = DisplayNamesOptions::default();
    options.style = Some(match slots.style.as_str() {
        "narrow" => Style::Narrow,
        "short" => Style::Short,
        _ => Style::Long,
    });
    options.fallback = if slots.fallback == "none" {
        Fallback::None
    } else {
        Fallback::Code
    };
    options.language_display = if slots.language_display.as_deref() == Some("standard") {
        LanguageDisplay::Standard
    } else {
        LanguageDisplay::Dialect
    };
    // The code was validated and canonicalized by CanonicalCodeForDisplayNames,
    // so ICU4X parses it; a failure to parse or to load the names data is a
    // gap in this engine, not "no such field".
    let missing = |agent: &mut Agent, what: &str, e: &dyn std::fmt::Display| {
        agent.not_implemented(&format!(
            "Intl.DisplayNames: ICU4X {what} for \"{}\" ({e})",
            slots.locale
        ))
    };
    match slots.r#type.as_str() {
        "region" => {
            let region = Region::try_from_str(code).map_err(|e| missing(agent, "region", &e))?;
            let names = RegionDisplayNames::try_new(locale.into(), options)
                .map_err(|e| missing(agent, "region names", &e))?;
            Ok(names.of(region).map(str::to_string))
        }
        "script" => {
            let script = Script::try_from_str(code).map_err(|e| missing(agent, "script", &e))?;
            let names = ScriptDisplayNames::try_new(locale.clone().into(), options)
                .map_err(|e| missing(agent, "script names", &e))?;
            Ok(names.of(script).map(str::to_string))
        }
        "language" => {
            let target =
                Locale::try_from_str(code).map_err(|e| missing(agent, "language tag", &e))?;
            // [[Fields]] has a name for a language tag when the data names its
            // language subtag; the rest of the tag qualifies that name.
            let languages = LanguageDisplayNames::try_new(locale.clone().into(), options)
                .map_err(|e| missing(agent, "language names", &e))?;
            if languages.of(target.id.language).is_none() {
                return Ok(None);
            }
            let formatter = LocaleDisplayNamesFormatter::try_new(locale.into(), options)
                .map_err(|e| missing(agent, "locale display names", &e))?;
            Ok(Some(formatter.of(&target).into_owned()))
        }
        other => Err(agent.not_implemented(&format!(
            "Intl.DisplayNames names for type \"{other}\" (no ICU4X data wired up)"
        ))),
    }
}
