//! Intl.Collator: the [[InitializedCollator]] internal slots and the
//! abstract operations shared by its built-ins.
#![allow(dead_code)] // helpers for the Intl tickets

use std::cmp::Ordering;

use icu::collator::options::{
    AlternateHandling, CaseLevel, CollatorOptions, MaxVariable, Strength,
};
use icu::collator::preferences::{CollationCaseFirst, CollationNumericOrdering, CollationType};
use icu::collator::provider::{Baked, CollationMetadataV1};
use icu::collator::{CollatorBorrowed, CollatorPreferences};
use icu::locale::Locale;
use icu::locale::extensions::unicode::Value as UValue;
use icu_provider::prelude::*;
use kahvi_core::gc::{Trace, Tracer};
use kahvi_core::prelude::*;

/// The internal slots of an Intl.Collator instance, plus the ICU4X collator
/// built from them.
#[derive(Debug)]
pub(crate) struct CollatorSlots {
    /// [[Locale]]
    pub locale: String,
    /// [[Usage]]
    pub usage: String,
    /// [[Collation]]
    pub collation: String,
    /// [[Numeric]]
    pub numeric: bool,
    /// [[CaseFirst]]
    pub case_first: String,
    /// [[Sensitivity]]
    pub sensitivity: String,
    /// [[IgnorePunctuation]]
    pub ignore_punctuation: bool,
    /// [[BoundCompare]]
    pub bound_compare: Option<Object>,
    /// The collator CompareStrings consults.
    pub collator: CollatorBorrowed<'static>,
}

impl Trace for CollatorSlots {
    fn trace(&self, t: &mut Tracer) {
        self.bound_compare.trace(t);
    }
}

/// The collation types CLDR defines other than the ones ECMA-402 excludes
/// ("standard" and "search"); each is probed against ICU4X's data.
const COLLATION_TYPES: &[&str] = &[
    "big5han", "compat", "dict", "direct", "ducet", "emoji", "eor", "gb2312", "phonebk",
    "phonetic", "pinyin", "reformed", "searchjl", "stroke", "trad", "unihan", "zhuyin",
];

/// The locale without its extensions, as ICU4X data lookups want it.
fn base_locale(locale: &str) -> Option<Locale> {
    let mut parsed = Locale::try_from_str(locale).ok()?;
    parsed.extensions.unicode.clear();
    parsed.extensions.transform.clear();
    parsed.extensions.private.clear();
    parsed.extensions.other.clear();
    Some(parsed)
}

/// Whether ICU4X has collation data of type _collation_ for _locale_ (or one
/// of its ancestors).
pub(crate) fn has_collation(locale: &str, collation: &str) -> bool {
    let Some(parsed) = base_locale(locale) else {
        return false;
    };
    let Ok(attributes) = DataMarkerAttributes::try_from_str(collation) else {
        return false;
    };
    let mut candidates: Vec<icu::locale::DataLocale> = Vec::new();
    candidates.push(icu::locale::DataLocale::from(&parsed));
    let mut shorter = parsed.clone();
    shorter.id.variants.clear();
    shorter.id.region = None;
    candidates.push(icu::locale::DataLocale::from(&shorter));
    shorter.id.script = None;
    candidates.push(icu::locale::DataLocale::from(&shorter));
    candidates.push(icu::locale::DataLocale::default());
    candidates.iter().any(|data_locale| {
        let request = DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, data_locale),
            metadata: {
                let mut metadata = DataRequestMetadata::default();
                metadata.silent = true;
                metadata
            },
        };
        DataProvider::<CollationMetadataV1>::load(&Baked, request).is_ok()
    })
}

/// The collations [[SortLocaleData]] and [[SearchLocaleData]] have for
/// _locale_: the default (*null*) first, then those ICU4X has data for.
pub(crate) fn collations_of(locale: &str) -> Vec<Option<String>> {
    let mut list = vec![None];
    list.extend(
        COLLATION_TYPES
            .iter()
            .filter(|c| has_collation(locale, c))
            .map(|c| Some(c.to_string())),
    );
    list
}

/// The ICU4X collator for a locale and the resolved options.
fn build_collator(
    locale: &str,
    collation: Option<&str>,
    numeric: bool,
    case_first: Option<&str>,
    sensitivity: Option<&str>,
    ignore_punctuation: Option<bool>,
) -> Option<CollatorBorrowed<'static>> {
    let parsed = base_locale(locale)?;
    let mut prefs = CollatorPreferences::from(&parsed);
    prefs.collation_type = collation
        .and_then(|c| UValue::try_from_str(c).ok())
        .and_then(|v| CollationType::try_from(&v).ok());
    prefs.numeric_ordering = Some(if numeric {
        CollationNumericOrdering::True
    } else {
        CollationNumericOrdering::False
    });
    prefs.case_first = case_first.map(|c| match c {
        "upper" => CollationCaseFirst::Upper,
        "lower" => CollationCaseFirst::Lower,
        _ => CollationCaseFirst::False,
    });
    let mut options = CollatorOptions::default();
    match sensitivity {
        Some("base") => options.strength = Some(Strength::Primary),
        Some("accent") => options.strength = Some(Strength::Secondary),
        Some("case") => {
            options.strength = Some(Strength::Primary);
            options.case_level = Some(CaseLevel::On);
        }
        Some(_) => options.strength = Some(Strength::Tertiary),
        None => {}
    }
    match ignore_punctuation {
        Some(true) => {
            options.alternate_handling = Some(AlternateHandling::Shifted);
            options.max_variable = Some(MaxVariable::Punctuation);
        }
        Some(false) => options.alternate_handling = Some(AlternateHandling::NonIgnorable),
        None => {}
    }
    icu::collator::Collator::try_new(prefs, options).ok()
}

/// The defaults ICU4X has for _locale_ with the given collation: the
/// resolved [[CaseFirst]], [[Numeric]] and [[IgnorePunctuation]].
pub(crate) fn locale_defaults(locale: &str, collation: Option<&str>) -> (String, bool, bool) {
    let default = base_locale(locale).and_then(|parsed| {
        let mut prefs = CollatorPreferences::from(&parsed);
        prefs.collation_type = collation
            .and_then(|c| UValue::try_from_str(c).ok())
            .and_then(|v| CollationType::try_from(&v).ok());
        icu::collator::Collator::try_new(prefs, CollatorOptions::default()).ok()
    });
    match default {
        Some(collator) => {
            let resolved = collator.resolved_options();
            (
                match resolved.case_first {
                    CollationCaseFirst::Upper => "upper",
                    CollationCaseFirst::Lower => "lower",
                    _ => "false",
                }
                .to_string(),
                resolved.numeric == CollationNumericOrdering::True,
                resolved.alternate_handling == AlternateHandling::Shifted,
            )
        }
        None => ("false".to_string(), false, false),
    }
}

/// The sensitivity ICU4X's collator of type _collation_ for _locale_ has
/// by default, as a [[Sensitivity]] value.
pub(crate) fn default_sensitivity(locale: &str, collation: Option<&str>) -> Option<&'static str> {
    let collator = build_collator(locale, collation, false, None, None, None)?;
    let resolved = collator.resolved_options();
    Some(match (resolved.strength, resolved.case_level) {
        (Strength::Primary, CaseLevel::On) => "case",
        (Strength::Primary, _) => "base",
        (Strength::Secondary, _) => "accent",
        _ => "variant",
    })
}

/// Builds the collator for the resolved options of a constructed Collator.
pub(crate) fn create_icu_collator(
    locale: &str,
    collation: &str,
    numeric: bool,
    case_first: &str,
    sensitivity: &str,
    ignore_punctuation: bool,
) -> Option<CollatorBorrowed<'static>> {
    let collation = (collation != "default").then_some(collation);
    build_collator(
        locale,
        collation,
        numeric,
        Some(case_first),
        Some(sensitivity),
        Some(ignore_punctuation),
    )
}

/// CompareStrings ( _collator_, _x_, _y_ )
pub(crate) fn compare_strings(slots: &CollatorSlots, x: &[u16], y: &[u16]) -> f64 {
    // Strings are compared as UTF-16 code unit sequences.
    match slots.collator.compare_utf16(x, y) {
        Ordering::Less => -1.0,
        Ordering::Equal => 0.0,
        Ordering::Greater => 1.0,
    }
}
