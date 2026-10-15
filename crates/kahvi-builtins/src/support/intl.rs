//! ECMA-402 foundations shared by every Intl service: locale lists and
//! canonicalization, locale negotiation and option reading. Locale data
//! comes from ICU4X (docs/INTL.md).
//!
//! Language tags are kept as Rust `String`s: a well-formed tag is ASCII.
#![allow(dead_code)] // helpers for the Intl tickets

use std::sync::OnceLock;

use icu::locale::LocaleCanonicalizer;
use icu::locale::extensions::unicode::Value as UValue;
use icu_provider::IterableDataProvider;
use kahvi_core::gc::{Trace, Tracer};
use kahvi_core::prelude::*;

// ---- %Intl% ---------------------------------------------------------------------

/// The internal slots of the %Intl% object.
#[derive(Debug)]
struct IntlSlots {
    /// [[FallbackSymbol]]: Symbol("IntlLegacyConstructedSymbol"), used by
    /// ChainDateTimeFormat, ChainNumberFormat and their Unwrap operations.
    fallback_symbol: Symbol,
}

impl Trace for IntlSlots {
    fn trace(&self, t: &mut Tracer) {
        self.fallback_symbol.trace(t);
    }
}

/// Gives the realm's %Intl% its internal slots.
pub(crate) fn install(agent: &mut Agent, realm: RealmId) {
    let intl = agent.intrinsic_in(realm, "%Intl%");
    let description = agent.string("IntlLegacyConstructedSymbol");
    let fallback_symbol = agent.new_symbol(Some(description));
    agent.obj_mut(intl).kind = ObjectKind::Native(Box::new(IntlSlots { fallback_symbol }));
}

/// %Intl%.[[FallbackSymbol]] of the current realm.
pub(crate) fn fallback_symbol(agent: &Agent) -> Symbol {
    let intl = agent.intrinsic("%Intl%");
    agent
        .native::<IntlSlots>(intl)
        .expect("%Intl% has its slots")
        .fallback_symbol
}

// ---- language tags -----------------------------------------------------------

/// The Rust string for a JS string holding a language tag, or `None` when it
/// contains a non-ASCII code unit (and so cannot be one).
pub(crate) fn ascii_string(agent: &Agent, s: JsString) -> Option<String> {
    let units = agent.str(s);
    units
        .iter()
        .all(|&u| u < 0x80)
        .then(|| units.iter().map(|&u| u as u8 as char).collect())
}

/// The [[InitializedLocale]] internal slots of an Intl.Locale object
///. Created by the Intl.Locale constructor.
#[derive(Debug, Clone, Default)]
pub(crate) struct IntlLocaleSlots {
    /// [[Locale]]
    pub locale: String,
    /// [[Calendar]]
    pub calendar: Option<String>,
    /// [[Collation]]
    pub collation: Option<String>,
    /// [[HourCycle]]
    pub hour_cycle: Option<String>,
    /// [[CaseFirst]]
    pub case_first: Option<String>,
    /// [[Numeric]]
    pub numeric: Option<bool>,
    /// [[NumberingSystem]]
    pub numbering_system: Option<String>,
    /// [[FirstDayOfWeek]]
    pub first_day_of_week: Option<String>,
}

impl Trace for IntlLocaleSlots {
    fn trace(&self, _: &mut Tracer) {}
}

/// _o_.[[Locale]] when _o_ has an [[InitializedLocale]] internal slot.
pub(crate) fn initialized_locale(agent: &Agent, v: Value) -> Option<String> {
    match v {
        Value::Object(o) => agent.native::<IntlLocaleSlots>(o).map(|l| l.locale.clone()),
        _ => None,
    }
}

/// CanonicalizeLocaleList ( _locales_ )
pub(crate) fn canonicalize_locale_list(agent: &mut Agent, locales: Value) -> JsResult<Vec<String>> {
    // 1. If _locales_ is *undefined*, then
    if matches!(locales, Value::Undefined) {
        // a. Return a new empty List.
        return Ok(Vec::new());
    }
    // 2. Let _seen_ be a new empty List.
    let mut seen: Vec<String> = Vec::new();
    // 3. If _locales_ is a String or _locales_ is an Object and _locales_ has an [[InitializedLocale]] internal slot, then
    let locales_obj =
        if matches!(locales, Value::String(_)) || initialized_locale(agent, locales).is_some() {
            // a. Let _localesObj_ be CreateArrayFromList(« _locales_ »).
            ops::create_array_from_list(agent, &[locales])
        } else {
            // 4. Else,
            // a. Let _localesObj_ be ? ToObject(_locales_).
            ops::to_object(agent, locales)?
        };
    // 5. Let _length_ be ? LengthOfArrayLike(_localesObj_).
    let length = ops::length_of_array_like(agent, locales_obj)?;
    // 6. Let _k_ be 0.
    let mut k = 0.0;
    // 7. Repeat, while _k_ < _length_,
    while k < length {
        // a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
        let property_key = agent.key_from_index(k);
        // b. Let _exists_ be ? HasProperty(_localesObj_, _propertyKey_).
        let exists = ops::has_property(agent, locales_obj, property_key)?;
        // c. If _exists_ is *true*, then
        if exists {
            // i. Let _element_ be ? Get(_localesObj_, _propertyKey_).
            let element = ops::get(agent, locales_obj, property_key)?;
            // ii. If _element_ is neither a String nor an Object, throw a *TypeError* exception.
            if !matches!(element, Value::String(_) | Value::Object(_)) {
                return Err(agent.throw_type_error("locale must be a string or an object"));
            }
            // iii. If _element_ is an Object and _element_ has an [[InitializedLocale]] internal slot, then
            let tag = match initialized_locale(agent, element) {
                // 1. Let _tag_ be _element_.[[Locale]].
                Some(locale) => Some(locale),
                // iv. Else,
                None => {
                    // 1. Let _tag_ be ? ToString(_element_).
                    let s = ops::to_string(agent, element)?;
                    ascii_string(agent, s)
                }
            };
            // v. If IsWellFormedLanguageTag(_tag_) is *false*, throw a *RangeError* exception.
            let Some(tag) = tag.filter(|t| is_well_formed_language_tag(t)) else {
                return Err(agent.throw_range_error("invalid language tag"));
            };
            // vi. Let _canonicalizedTag_ be CanonicalizeUnicodeLocaleId(_tag_).
            let canonicalized_tag = canonicalize_unicode_locale_id(&tag);
            // vii. If _seen_ does not contain _canonicalizedTag_, append _canonicalizedTag_ to _seen_.
            if !seen.contains(&canonicalized_tag) {
                seen.push(canonicalized_tag);
            }
        }
        // d. Set _k_ to _k_ + 1.
        k += 1.0;
    }
    // 8. Return _seen_.
    Ok(seen)
}

/// IsWellFormedLanguageTag ( _locale_ )
///
/// ICU4X's parser implements the unicode_locale_id grammar and rejects the
/// backwards compatibility syntax, duplicate variants (also in a
/// transformed_extensions tlang) and duplicate singletons; it accepts
/// duplicate ukeys and tkeys, as this operation does.
pub(crate) fn is_well_formed_language_tag(locale: &str) -> bool {
    // 1. Let _lowerLocale_ be the ASCII-lowercase of _locale_.
    let lower_locale = locale.to_ascii_lowercase();
    // 2. If _lowerLocale_ cannot be matched by the unicode_locale_id Unicode locale nonterminal, return *false*.
    // 3. If _lowerLocale_ uses any of the backwards compatibility syntax described in Unicode Technical Standard #35 Part 1 Core, Section 3.3 BCP 47 Conformance, return *false*.
    // 4.-11. (duplicate variants and singletons)
    // 12. Return *true*.
    icu::locale::Locale::try_from_str(&lower_locale).is_ok()
}

fn canonicalizer() -> &'static LocaleCanonicalizer {
    static C: OnceLock<LocaleCanonicalizer> = OnceLock::new();
    C.get_or_init(LocaleCanonicalizer::new_extended)
}

/// CanonicalizeUnicodeLocaleId ( _locale_ ). _locale_ must be well
/// formed.
pub(crate) fn canonicalize_unicode_locale_id(locale: &str) -> String {
    let mut parsed = icu::locale::Locale::try_from_str(locale).expect("a well-formed language tag");
    // 1. Let _localeId_ be the String value resulting from performing the Processing LocaleIds algorithm to transform _locale_ to canonical form per Unicode Technical Standard #35 Part 1 Core, Annex C LocaleId Canonicalization.
    canonicalizer().canonicalize(&mut parsed);
    // (Annex C also replaces deprecated -u- values with their preferred ones.)
    let keywords: Vec<_> = parsed
        .extensions
        .unicode
        .keywords
        .iter()
        .map(|(k, v)| (*k, v.to_string()))
        .collect();
    for (key, value) in keywords {
        let canonical = canonicalize_u_value(key.as_str(), &value);
        if canonical != value {
            let value = UValue::try_from_str(&canonical).expect("a CLDR type value");
            parsed.extensions.unicode.keywords.set(key, value);
        }
    }
    // 2. If _localeId_ contains a substring that is a Unicode locale extension sequence, then
    //   a.-g. Keep only the first instance of each attribute and the first keyword for each key.
    // ICU4X's parser already keeps only the first of each.
    // 3. Return _localeId_.
    parsed.to_string()
}

/// CanonicalizeUValue ( _ukey_, _uvalue_ )
///
/// The aliases are CLDR's common/bcp47 data for the keys whose values have
/// deprecated forms, except timezone (tz) aliases, which are not here yet.
pub(crate) fn canonicalize_u_value(ukey: &str, uvalue: &str) -> String {
    // 1. Let _lowerValue_ be the ASCII-lowercase of _uvalue_.
    let lower_value = uvalue.to_ascii_lowercase();
    // 2. Let _canonicalized_ be the String value resulting from canonicalizing _lowerValue_ as a value of key _ukey_ per Unicode Technical Standard #35 Part 1 Core, Annex C LocaleId Canonicalization Section 5 Canonicalizing Syntax, Processing LocaleIds.
    // 3. NOTE: It is recommended that implementations use the 'u' extension data in common/bcp47 provided by the Common Locale Data Repository (available at https://cldr.unicode.org/).
    let canonicalized = match (ukey, lower_value.as_str()) {
        ("ca", "ethiopic-amete-alem") => "ethioaa",
        ("ca", "islamicc") => "islamic-civil",
        ("ks", "primary") => "level1",
        ("ks", "tertiary") => "level3",
        ("ms", "imperial") => "uksystem",
        ("kb" | "kc" | "kh" | "kk" | "kn", "yes") => "true",
        _ => return lower_value,
    };
    // 4. Return _canonicalized_.
    canonicalized.to_string()
}

// ---- Unicode locale extensions -----------------------------------------------

/// A [[Key]]/[[Value]] keyword of a Unicode locale extension sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Keyword {
    pub key: String,
    pub value: String,
}

/// UnicodeExtensionComponents ( _extension_ ): the
/// ([[Attributes]], [[Keywords]]) of _extension_.
pub(crate) fn unicode_extension_components(extension: &str) -> (Vec<String>, Vec<Keyword>) {
    // 1. Assert: The ASCII-lowercase of _extension_ is _extension_.
    debug_assert_eq!(extension.to_ascii_lowercase(), extension);
    // 2. Assert: The substring of _extension_ from 0 to 3 is *"-u-"*.
    debug_assert!(extension.starts_with("-u-"));
    // 3. Let _attributes_ be a new empty List.
    let mut attributes: Vec<String> = Vec::new();
    // 4. Let _keywords_ be a new empty List.
    let mut keywords: Vec<Keyword> = Vec::new();
    // 5. Let _keyword_ be *undefined*.
    // (`keyword` is the index in _keywords_ of the keyword being read, or
    // None for one that duplicates an earlier key and is dropped.)
    let mut keyword: Option<Option<usize>> = None;
    // 6.-8. For each subtag after "-u-":
    for subtag in extension[3..].split('-') {
        let length = subtag.len();
        // e. Assert: _length_ ≥ 2.
        debug_assert!(length >= 2);
        // f. If _keyword_ is *undefined* and _length_ ≠ 2, then
        if keyword.is_none() && length != 2 {
            // i. If _subtag_ is not an element of _attributes_, append _subtag_ to _attributes_.
            if !attributes.iter().any(|a| a == subtag) {
                attributes.push(subtag.to_string());
            }
        } else if length == 2 {
            // g. Else if _length_ = 2, then
            // i. Set _keyword_ to the Record { [[Key]]: _subtag_, [[Value]]: the empty String }.
            // ii. If _keywords_ does not contain an element whose [[Key]] is _keyword_.[[Key]], append _keyword_ to _keywords_.
            if keywords.iter().any(|k| k.key == subtag) {
                keyword = Some(None);
            } else {
                keywords.push(Keyword {
                    key: subtag.to_string(),
                    value: String::new(),
                });
                keyword = Some(Some(keywords.len() - 1));
            }
        } else if let Some(Some(i)) = keyword {
            let value = &mut keywords[i].value;
            // h. Else if _keyword_.[[Value]] is the empty String, then
            if value.is_empty() {
                // i. Set _keyword_.[[Value]] to _subtag_.
                value.push_str(subtag);
            } else {
                // i. Else,
                // i. Set _keyword_.[[Value]] to the string-concatenation of _keyword_.[[Value]], *"-"*, and _subtag_.
                value.push('-');
                value.push_str(subtag);
            }
        }
        // j. Set _k_ to _k_ + _length_ + 1.
    }
    // 9. Return the Record { [[Attributes]]: _attributes_, [[Keywords]]: _keywords_ }.
    (attributes, keywords)
}

/// The Unicode locale extension sequence of a canonicalized _locale_ (the
/// "-u-…" part, up to the next singleton), if any.
pub(crate) fn unicode_extension_of(locale: &str) -> Option<&str> {
    // A private use sequence comes last; its "-u-" is not an extension.
    let end = locale.find("-x-").unwrap_or(locale.len());
    let start = locale[..end].find("-u-")?;
    let rest = &locale[start + 3..end];
    // The sequence ends at the next singleton subtag.
    let mut len = start + 2;
    for subtag in rest.split('-') {
        if subtag.len() == 1 {
            break;
        }
        len += 1 + subtag.len();
    }
    Some(&locale[start..len])
}

/// _locale_ with any Unicode locale extension sequence removed.
pub(crate) fn remove_unicode_extension(locale: &str) -> String {
    match unicode_extension_of(locale) {
        Some(ext) => locale.replacen(ext, "", 1),
        None => locale.to_string(),
    }
}

/// InsertUnicodeExtensionAndCanonicalize ( _locale_, _attributes_, _keywords_ )
pub(crate) fn insert_unicode_extension_and_canonicalize(
    locale: &str,
    attributes: &[String],
    keywords: &[Keyword],
) -> String {
    // 1. Assert: _locale_ does not contain a Unicode locale extension sequence.
    debug_assert!(unicode_extension_of(locale).is_none());
    // 2. Let _extension_ be *"-u"*.
    let mut extension = String::from("-u");
    // 3. For each element _attr_ of _attributes_, do
    for attr in attributes {
        // a. Set _extension_ to the string-concatenation of _extension_, *"-"*, and _attr_.
        extension.push('-');
        extension.push_str(attr);
    }
    // 4. For each Record { [[Key]], [[Value]] } _keyword_ of _keywords_, do
    for keyword in keywords {
        // a. Set _extension_ to the string-concatenation of _extension_, *"-"*, and _keyword_.[[Key]].
        extension.push('-');
        extension.push_str(&keyword.key);
        // b. If _keyword_.[[Value]] is not the empty String, set _extension_ to the string-concatenation of _extension_, *"-"*, and _keyword_.[[Value]].
        if !keyword.value.is_empty() {
            extension.push('-');
            extension.push_str(&keyword.value);
        }
    }
    // 5. If _extension_ is *"-u"*, return CanonicalizeUnicodeLocaleId(_locale_).
    if extension == "-u" {
        return canonicalize_unicode_locale_id(locale);
    }
    // 6. Let _privateIndex_ be StringIndexOf(_locale_, *"-x-"*, 0).
    let new_locale = match locale.find("-x-") {
        // 7. If _privateIndex_ is ~not-found~, then
        // a. Let _newLocale_ be the string-concatenation of _locale_ and _extension_.
        None => format!("{locale}{extension}"),
        // 8. Else,
        Some(private_index) => {
            // a. Let _preExtension_ be the substring of _locale_ from 0 to _privateIndex_.
            // b. Let _postExtension_ be the substring of _locale_ from _privateIndex_.
            // c. Let _newLocale_ be the string-concatenation of _preExtension_, _extension_, and _postExtension_.
            format!(
                "{}{extension}{}",
                &locale[..private_index],
                &locale[private_index..]
            )
        }
    };
    // 9. Assert: IsWellFormedLanguageTag(_newLocale_) is *true*.
    debug_assert!(is_well_formed_language_tag(&new_locale));
    // 10. Return CanonicalizeUnicodeLocaleId(_newLocale_).
    canonicalize_unicode_locale_id(&new_locale)
}

// ---- available locales -------------------------------------------------------

/// DefaultLocale ( ). kahvi has no host locale setting.
pub(crate) fn default_locale() -> &'static str {
    "en-US"
}

/// Lists the locales of ICU4X's baked data. Baked data leaves out a locale
/// whose data equals its parent's, so one marker misses locales: Gregorian
/// month names cover every language, number symbols add regional variants.
struct BakedLocales;

const _: () = {
    use icu_datetime_data::__impl_datetime_names_month_gregorian_v1;
    use icu_decimal_data::{__impl_decimal_symbols_v1, make_provider};
    make_provider!(BakedLocales);
    __impl_datetime_names_month_gregorian_v1!(BakedLocales, ITER);
    __impl_decimal_symbols_v1!(BakedLocales, ITER);
};

/// The Available Locales List of every service: the locales ICU4X's
/// compiled data has month names or number symbols for, plus
/// DefaultLocale(), plus the less narrow tags ECMA-402 requires for each
/// (language, language-script, language-region).
pub(crate) fn available_locales() -> &'static [String] {
    static LIST: OnceLock<Vec<String>> = OnceLock::new();
    LIST.get_or_init(|| {
        let months = IterableDataProvider::<
            icu::datetime::provider::names::DatetimeNamesMonthGregorianV1,
        >::iter_ids(&BakedLocales);
        let symbols = IterableDataProvider::<icu::decimal::provider::DecimalSymbolsV1>::iter_ids(
            &BakedLocales,
        );
        let ids = months.into_iter().chain(symbols).flatten();
        let mut list: Vec<String> = Vec::new();
        let mut add = |tag: String| {
            if !list.contains(&tag) {
                list.push(tag);
            }
        };
        add(default_locale().to_string());
        for id in ids {
            if id.locale.is_unknown() {
                continue;
            }
            let full = id.locale.into_locale();
            let lang = full.id.language;
            add(lang.to_string());
            if let Some(script) = full.id.script {
                add(format!("{lang}-{script}"));
            }
            if let Some(region) = full.id.region {
                add(format!("{lang}-{region}"));
            }
            add(canonicalize_unicode_locale_id(&full.to_string()));
        }
        list.sort();
        list
    })
}

// ---- negotiation ---------------------------------------------------------------

/// The Record { [[locale]], [[extension]] } of a locale match.
#[derive(Debug, Clone)]
pub(crate) struct LocaleMatch {
    pub locale: String,
    pub extension: Option<String>,
}

/// LookupMatchingLocaleByPrefix ( _availableLocales_, _requestedLocales_ )
pub(crate) fn lookup_matching_locale_by_prefix(
    available_locales: &[String],
    requested_locales: &[String],
) -> Option<LocaleMatch> {
    // 1. For each element _locale_ of _requestedLocales_, do
    for locale in requested_locales {
        // a. Let _extension_ be ~empty~.
        // b. If _locale_ contains a Unicode locale extension sequence, then
        //   i. Set _extension_ to the Unicode locale extension sequence of _locale_.
        let extension = unicode_extension_of(locale).map(str::to_string);
        //   ii. Set _locale_ to the String value that is _locale_ with any Unicode locale extension sequences removed.
        let locale = remove_unicode_extension(locale);
        // c. Let _prefix_ be _locale_.
        let mut prefix = locale.as_str();
        // d. Repeat, while _prefix_ is not the empty String,
        while !prefix.is_empty() {
            // i. If _availableLocales_ contains _prefix_, return the Record { [[locale]]: _prefix_, [[extension]]: _extension_ }.
            if available_locales.iter().any(|a| a == prefix) {
                return Some(LocaleMatch {
                    locale: prefix.to_string(),
                    extension,
                });
            }
            // ii. If _prefix_ contains *"-"* (code unit 0x002D HYPHEN-MINUS), let _pos_ be the index into _prefix_ of the last occurrence of *"-"*; else let _pos_ be 0.
            let mut pos = prefix.rfind('-').unwrap_or(0);
            // iii. Repeat, while _pos_ ≥ 2 and the substring of _prefix_ from _pos_ - 2 to _pos_ - 1 is *"-"*,
            while pos >= 2 && &prefix[pos - 2..pos - 1] == "-" {
                // 1. Set _pos_ to _pos_ - 2.
                pos -= 2;
            }
            // iv. Set _prefix_ to the substring of _prefix_ from 0 to _pos_.
            prefix = &prefix[..pos];
        }
    }
    // 2. Return *undefined*.
    None
}

/// LookupMatchingLocaleByBestFit ( _availableLocales_, _requestedLocales_ )
///
/// Implementation-defined. kahvi's best fit is the lookup algorithm.
pub(crate) fn lookup_matching_locale_by_best_fit(
    available_locales: &[String],
    requested_locales: &[String],
) -> Option<LocaleMatch> {
    lookup_matching_locale_by_prefix(available_locales, requested_locales)
}

/// A value in a resolution options Record or a localeData List: a String,
/// *null* or (options only) *undefined*.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KeyValue {
    Undefined,
    Null,
    String(String),
}

/// The _options_ Record ResolveLocale takes: { [[localeMatcher]], [[<key>]]… }.
#[derive(Debug, Clone)]
pub(crate) struct ResolutionOptions {
    /// "lookup" or "best fit".
    pub locale_matcher: String,
    /// [[<key>]] for each relevant extension key, in descriptor order.
    pub keys: Vec<(&'static str, KeyValue)>,
}

impl ResolutionOptions {
    pub(crate) fn get(&self, key: &str) -> Option<&KeyValue> {
        self.keys.iter().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    pub(crate) fn set(&mut self, key: &'static str, value: KeyValue) {
        match self.keys.iter_mut().find(|(k, _)| *k == key) {
            Some(entry) => entry.1 = value,
            None => self.keys.push((key, value)),
        }
    }
}

/// _localeData_ of a service: for a found locale and a relevant
/// extension key, the List of supported values, the default first. `None`
/// in the List is *null*.
pub(crate) type LocaleData<'a> = &'a dyn Fn(&str, &str) -> Vec<Option<String>>;

/// The Record ResolveLocale returns: [[Locale]] and [[<key>]] for each
/// relevant extension key ([[LocaleData]] is the service's own to look up
/// again from [[Locale]]'s data).
#[derive(Debug, Clone)]
pub(crate) struct ResolvedLocale {
    /// [[Locale]]
    pub locale: String,
    /// The locale the data was found for (_foundLocale_ before any
    /// extension was inserted).
    pub data_locale: String,
    /// [[<key>]]: the value, or `None` for *null*.
    pub keys: Vec<(&'static str, Option<String>)>,
}

impl ResolvedLocale {
    pub(crate) fn get(&self, key: &str) -> Option<&str> {
        self.keys
            .iter()
            .find(|(k, _)| *k == key)
            .and_then(|(_, v)| v.as_deref())
    }
}

/// ResolveLocale ( _availableLocales_, _requestedLocales_, _options_, _relevantExtensionKeys_ [ , _localeData_ ] )
pub(crate) fn resolve_locale(
    available_locales: &[String],
    requested_locales: &[String],
    options: &ResolutionOptions,
    relevant_extension_keys: &[&'static str],
    locale_data: Option<LocaleData>,
) -> ResolvedLocale {
    // 1. Let _matcher_ be _options_.[[localeMatcher]].
    // 2. If _matcher_ is *"lookup"*, then
    let r#match = if options.locale_matcher == "lookup" {
        // a. Let _match_ be LookupMatchingLocaleByPrefix(_availableLocales_, _requestedLocales_).
        lookup_matching_locale_by_prefix(available_locales, requested_locales)
    } else {
        // 3. Else,
        // a. Let _match_ be LookupMatchingLocaleByBestFit(_availableLocales_, _requestedLocales_).
        lookup_matching_locale_by_best_fit(available_locales, requested_locales)
    };
    // 4. If _match_ is *undefined*, set _match_ to the Record { [[locale]]: DefaultLocale(), [[extension]]: ~empty~ }.
    let r#match = r#match.unwrap_or_else(|| LocaleMatch {
        locale: default_locale().to_string(),
        extension: None,
    });
    // 5. Let _foundLocale_ be _match_.[[locale]].
    let mut found_locale = r#match.locale.clone();
    // 6. Let _result_ be a new Record.
    let mut result = ResolvedLocale {
        locale: String::new(),
        data_locale: found_locale.clone(),
        keys: Vec::new(),
    };
    // 7. If _localeData_ is not present, then
    //   a. Assert: _relevantExtensionKeys_ is an empty List.
    //   b. Set _result_.[[LocaleData]] to *undefined*.
    // 8. Else,
    //   a. Let _foundLocaleData_ be _localeData_.[[<_foundLocale_>]].
    //   b. Assert: _foundLocaleData_ is a Record.
    //   c. Set _result_.[[LocaleData]] to _foundLocaleData_.
    debug_assert!(locale_data.is_some() || relevant_extension_keys.is_empty());
    // 9. If _match_.[[extension]] is not ~empty~, then
    let keywords = match &r#match.extension {
        // a. Let _components_ be UnicodeExtensionComponents(_match_.[[extension]]).
        // b. Let _keywords_ be _components_.[[Keywords]].
        Some(extension) => unicode_extension_components(extension).1,
        // 10. Else,
        // a. Let _keywords_ be a new empty List.
        None => Vec::new(),
    };
    // 11. Let _supportedKeywords_ be a new empty List.
    let mut supported_keywords: Vec<Keyword> = Vec::new();
    // 12. For each element _key_ of _relevantExtensionKeys_, do
    for &key in relevant_extension_keys {
        // a. Let _keyLocaleData_ be _foundLocaleData_.[[<_key_>]].
        // b. Assert: _keyLocaleData_ is a List.
        let key_locale_data =
            locale_data.expect("localeData with relevant keys")(&found_locale, key);
        let contains = |v: &str| key_locale_data.iter().any(|d| d.as_deref() == Some(v));
        // c. Let _value_ be _keyLocaleData_[0].
        // d. Assert: _value_ is a String or _value_ is *null*.
        let mut value: Option<String> = key_locale_data.first().cloned().flatten();
        // e. Let _supportedKeyword_ be ~empty~.
        let mut supported_keyword: Option<Keyword> = None;
        // f. If _keywords_ contains an element whose [[Key]] is _key_, then
        if let Some(entry) = keywords.iter().find(|k| k.key == key) {
            // i. Let _entry_ be the element of _keywords_ whose [[Key]] is _key_.
            // ii. Let _requestedValue_ be _entry_.[[Value]].
            let requested_value = &entry.value;
            // iii. If _requestedValue_ is not the empty String, then
            if !requested_value.is_empty() {
                // 1. If _keyLocaleData_ contains _requestedValue_, then
                if contains(requested_value) {
                    // a. Set _value_ to _requestedValue_.
                    value = Some(requested_value.clone());
                    // b. Set _supportedKeyword_ to the Record { [[Key]]: _key_, [[Value]]: _value_ }.
                    supported_keyword = Some(Keyword {
                        key: key.to_string(),
                        value: requested_value.clone(),
                    });
                }
            } else if contains("true") {
                // iv. Else if _keyLocaleData_ contains *"true"*, then
                // 1. Set _value_ to *"true"*.
                value = Some("true".to_string());
                // 2. Set _supportedKeyword_ to the Record { [[Key]]: _key_, [[Value]]: the empty String }.
                supported_keyword = Some(Keyword {
                    key: key.to_string(),
                    value: String::new(),
                });
            }
        }
        // g. Assert: _options_ has a field [[<_key_>]].
        // h. Let _optionsValue_ be _options_.[[<_key_>]].
        let mut options_value = options
            .get(key)
            .cloned()
            .expect("options has a field for each relevant key");
        // i. Assert: _optionsValue_ is a String, or _optionsValue_ is either *undefined* or *null*.
        // j. If _optionsValue_ is a String, then
        if let KeyValue::String(s) = &options_value {
            // i. Let _ukey_ be the ASCII-lowercase of _key_.
            let ukey = key.to_ascii_lowercase();
            // ii. Set _optionsValue_ to CanonicalizeUValue(_ukey_, _optionsValue_).
            let mut canonical = canonicalize_u_value(&ukey, s);
            // iii. If _optionsValue_ is the empty String, then
            if canonical.is_empty() {
                // 1. Set _optionsValue_ to *"true"*.
                canonical = "true".to_string();
            }
            options_value = KeyValue::String(canonical);
        }
        // k. If SameValue(_optionsValue_, _value_) is *false* and _keyLocaleData_ contains _optionsValue_, then
        let options_as_data = match &options_value {
            KeyValue::Undefined => None,
            KeyValue::Null => Some(None),
            KeyValue::String(s) => Some(Some(s.clone())),
        };
        if let Some(options_as_data) = options_as_data
            && options_as_data != value
            && key_locale_data.contains(&options_as_data)
        {
            // i. Set _value_ to _optionsValue_.
            value = options_as_data;
            // ii. Set _supportedKeyword_ to ~empty~.
            supported_keyword = None;
        }
        // l. If _supportedKeyword_ is not ~empty~, append _supportedKeyword_ to _supportedKeywords_.
        if let Some(k) = supported_keyword {
            supported_keywords.push(k);
        }
        // m. Set _result_.[[<_key_>]] to _value_.
        result.keys.push((key, value));
    }
    // 13. If _supportedKeywords_ is not empty, then
    if !supported_keywords.is_empty() {
        // a. Let _supportedAttributes_ be a new empty List.
        // b. Set _foundLocale_ to InsertUnicodeExtensionAndCanonicalize(_foundLocale_, _supportedAttributes_, _supportedKeywords_).
        found_locale =
            insert_unicode_extension_and_canonicalize(&found_locale, &[], &supported_keywords);
    }
    // 14. Set _result_.[[Locale]] to _foundLocale_.
    result.locale = found_locale;
    // 15. Return _result_.
    result
}

/// A Resolution Option Descriptor.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ResolutionOptionDescriptor {
    /// [[Key]]
    pub key: &'static str,
    /// [[Property]]
    pub property: &'static str,
    /// [[Type]] (~string~ when absent)
    pub r#type: OptionType,
    /// [[Values]] (~empty~ when absent)
    pub values: Option<&'static [&'static str]>,
}

/// The internal slots of a service constructor that ResolveOptions reads.
pub(crate) struct ServiceConstructor<'a> {
    /// [[AvailableLocales]]
    pub available_locales: &'a [String],
    /// [[RelevantExtensionKeys]]
    pub relevant_extension_keys: &'a [&'static str],
    /// [[ResolutionOptionDescriptors]]
    pub resolution_option_descriptors: &'a [ResolutionOptionDescriptor],
}

/// The _specialBehaviours_ ResolveOptions takes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SpecialBehaviour {
    RequireOptions,
    CoerceOptions,
}

/// The Record ResolveOptions returns.
#[derive(Debug, Clone)]
pub(crate) struct ResolvedOptions {
    /// [[Options]]
    pub options: Object,
    /// [[ResolvedLocale]]
    pub resolved_locale: ResolvedLocale,
    /// [[ResolutionOptions]]
    pub resolution_options: ResolutionOptions,
}

/// ResolveOptions ( _ctor_, _localeData_, _locales_, _options_ [ , _specialBehaviours_ [ , _modifyResolutionOptions_ ] ] )
pub(crate) fn resolve_options(
    agent: &mut Agent,
    ctor: &ServiceConstructor,
    locale_data: LocaleData,
    locales: Value,
    options: Value,
    special_behaviours: &[SpecialBehaviour],
    modify_resolution_options: Option<&dyn Fn(&mut ResolutionOptions)>,
) -> JsResult<ResolvedOptions> {
    // 1. Let _requestedLocales_ be ? CanonicalizeLocaleList(_locales_).
    let requested_locales = canonicalize_locale_list(agent, locales)?;
    // 2. If _specialBehaviours_ is present and contains ~require-options~ and _options_ is *undefined*, throw a *TypeError* exception.
    if special_behaviours.contains(&SpecialBehaviour::RequireOptions)
        && matches!(options, Value::Undefined)
    {
        return Err(agent.throw_type_error("options are required"));
    }
    // 3. If _specialBehaviours_ is present and contains ~coerce-options~, set _options_ to ? CoerceOptionsToObject(_options_). Otherwise, set _options_ to ? GetOptionsObject(_options_).
    let options = if special_behaviours.contains(&SpecialBehaviour::CoerceOptions) {
        coerce_options_to_object(agent, options)?
    } else {
        get_options_object(agent, options)?
    };
    // 4. Let _matcher_ be ? GetOption(_options_, *"localeMatcher"*, ~string~, « *"lookup"*, *"best fit"* », *"best fit"*).
    let matcher = get_string_option(
        agent,
        options,
        "localeMatcher",
        &["lookup", "best fit"],
        Some("best fit"),
    )?
    .expect("a default");
    // 5. Let _opt_ be the Record { [[localeMatcher]]: _matcher_ }.
    let mut opt = ResolutionOptions {
        locale_matcher: matcher,
        keys: Vec::new(),
    };
    // 6. For each Resolution Option Descriptor _desc_ of _ctor_.[[ResolutionOptionDescriptors]], do
    for desc in ctor.resolution_option_descriptors {
        // a. If _desc_ has a [[Type]] field, let _type_ be _desc_.[[Type]]. Otherwise, let _type_ be ~string~.
        // b. If _desc_ has a [[Values]] field, let _values_ be _desc_.[[Values]]. Otherwise, let _values_ be ~empty~.
        // c. Let _value_ be ? GetOption(_options_, _desc_.[[Property]], _type_, _values_, *undefined*).
        let value = get_option(
            agent,
            options,
            desc.property,
            desc.r#type,
            desc.values,
            Some(Value::Undefined),
        )?;
        // d. If _value_ is not *undefined*, then
        let value = if !matches!(value, Value::Undefined) {
            // i. Set _value_ to ! ToString(_value_).
            let s = ops::to_string(agent, value)?;
            let s = ascii_string(agent, s);
            // ii. If _value_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
            match s.filter(|s| is_unicode_type(s)) {
                Some(s) => KeyValue::String(s),
                None => {
                    return Err(
                        agent.throw_range_error(&format!("invalid {} option", desc.property))
                    );
                }
            }
        } else {
            KeyValue::Undefined
        };
        // e. Let _key_ be _desc_.[[Key]].
        // f. Set _opt_.[[<_key_>]] to _value_.
        opt.set(desc.key, value);
    }
    // 7. If _modifyResolutionOptions_ is present, perform ! _modifyResolutionOptions_(_opt_).
    if let Some(modify) = modify_resolution_options {
        modify(&mut opt);
    }
    // 8. Let _resolution_ be ResolveLocale(_ctor_.[[AvailableLocales]], _requestedLocales_, _opt_, _ctor_.[[RelevantExtensionKeys]], _localeData_).
    let resolution = resolve_locale(
        ctor.available_locales,
        &requested_locales,
        &opt,
        ctor.relevant_extension_keys,
        Some(locale_data),
    );
    // 9. Assert: _resolution_.[[LocaleData]] is a Record.
    // 10. Return the Record { [[Options]]: _options_, [[ResolvedLocale]]: _resolution_, [[ResolutionOptions]]: _opt_ }.
    Ok(ResolvedOptions {
        options,
        resolved_locale: resolution,
        resolution_options: opt,
    })
}

/// Whether _s_ matches the `type` Unicode locale nonterminal:
/// (3*8alphanum) *("-" (3*8alphanum)).
pub(crate) fn is_unicode_type(s: &str) -> bool {
    s.split('-').all(|part| {
        (3..=8).contains(&part.len()) && part.bytes().all(|b| b.is_ascii_alphanumeric())
    })
}

/// FilterLocales ( _availableLocales_, _requestedLocales_, _options_ )
pub(crate) fn filter_locales(
    agent: &mut Agent,
    available_locales: &[String],
    requested_locales: &[String],
    options: Value,
) -> JsResult<Object> {
    // 1. Set _options_ to ? CoerceOptionsToObject(_options_).
    let options = coerce_options_to_object(agent, options)?;
    // 2. Let _matcher_ be ? GetOption(_options_, *"localeMatcher"*, ~string~, « *"lookup"*, *"best fit"* », *"best fit"*).
    let matcher = get_string_option(
        agent,
        options,
        "localeMatcher",
        &["lookup", "best fit"],
        Some("best fit"),
    )?
    .expect("a default");
    // 3. Let _subset_ be a new empty List.
    let mut subset: Vec<Value> = Vec::new();
    // 4. For each element _locale_ of _requestedLocales_, do
    for locale in requested_locales {
        let single = std::slice::from_ref(locale);
        // a. If _matcher_ is *"lookup"*, then
        let r#match = if matcher == "lookup" {
            // i. Let _match_ be LookupMatchingLocaleByPrefix(_availableLocales_, « _locale_ »).
            lookup_matching_locale_by_prefix(available_locales, single)
        } else {
            // b. Else,
            // i. Let _match_ be LookupMatchingLocaleByBestFit(_availableLocales_, « _locale_ »).
            lookup_matching_locale_by_best_fit(available_locales, single)
        };
        // c. If _match_ is not *undefined*, append _locale_ to _subset_.
        if r#match.is_some() {
            subset.push(Value::String(agent.string(locale)));
        }
    }
    // 5. Return CreateArrayFromList(_subset_).
    Ok(ops::create_array_from_list(agent, &subset))
}

// ---- options -------------------------------------------------------------------

/// The _type_ of GetOption: ~boolean~ or ~string~.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OptionType {
    Boolean,
    String,
}

/// GetOptionsObject ( _options_ )
pub(crate) fn get_options_object(agent: &mut Agent, options: Value) -> JsResult<Object> {
    match options {
        // 1. If _options_ is *undefined*, then
        // a. Return OrdinaryObjectCreate(*null*).
        Value::Undefined => Ok(agent.ordinary_object_create(None)),
        // 2. If _options_ is an Object, then
        // a. Return _options_.
        Value::Object(o) => Ok(o),
        // 3. Throw a *TypeError* exception.
        _ => Err(agent.throw_type_error("options must be an object")),
    }
}

/// CoerceOptionsToObject ( _options_ )
pub(crate) fn coerce_options_to_object(agent: &mut Agent, options: Value) -> JsResult<Object> {
    // 1. If _options_ is *undefined*, then
    if matches!(options, Value::Undefined) {
        // a. Return OrdinaryObjectCreate(*null*).
        return Ok(agent.ordinary_object_create(None));
    }
    // 2. Return ? ToObject(_options_).
    ops::to_object(agent, options)
}

/// GetOption ( _options_, _propertyKey_, _type_, _values_, _default_ )
///
/// `default` is `None` for ~required~. _values_ apply to ~string~ options.
pub(crate) fn get_option(
    agent: &mut Agent,
    options: Object,
    property_key: &str,
    r#type: OptionType,
    values: Option<&[&str]>,
    default: Option<Value>,
) -> JsResult<Value> {
    // 1. Let _value_ be ? Get(_options_, _propertyKey_).
    let key = agent.key(property_key);
    let value = ops::get(agent, options, key)?;
    // 2. If _value_ is *undefined*, then
    if matches!(value, Value::Undefined) {
        return match default {
            // a. If _default_ is ~required~, throw a *TypeError* exception.
            None => Err(agent.throw_type_error(&format!("{property_key} is required"))),
            // b. Return _default_.
            Some(d) => Ok(d),
        };
    }
    let value = match r#type {
        // 3. If _type_ is ~boolean~, then
        // a. Set _value_ to ToBoolean(_value_).
        OptionType::Boolean => Value::Boolean(ops::to_boolean(agent, value)),
        // 4. Else,
        // a. Assert: _type_ is ~string~.
        // b. Set _value_ to ? ToString(_value_).
        OptionType::String => Value::String(ops::to_string(agent, value)?),
    };
    // 5. If _values_ is not ~empty~ and _values_ does not contain _value_, throw a *RangeError* exception.
    if let Some(values) = values {
        let contained = match value {
            Value::String(s) => values
                .iter()
                .any(|v| agent.str(s).iter().copied().eq(v.encode_utf16())),
            Value::Boolean(b) => values.contains(&if b { "true" } else { "false" }),
            _ => false,
        };
        if !contained {
            return Err(agent.throw_range_error(&format!("invalid value for {property_key}")));
        }
    }
    // 6. Return _value_.
    Ok(value)
}

/// GetOption with _type_ ~string~, as a Rust string (`None` for an
/// *undefined* default). Allowed values are ASCII, so a value that passed
/// _values_ is too; without _values_, non-ASCII code units are kept lossily.
pub(crate) fn get_string_option(
    agent: &mut Agent,
    options: Object,
    property_key: &str,
    values: &[&str],
    default: Option<&str>,
) -> JsResult<Option<String>> {
    let default = match default {
        Some(d) => Value::String(agent.string(d)),
        None => Value::Undefined,
    };
    let values = (!values.is_empty()).then_some(values);
    match get_option(
        agent,
        options,
        property_key,
        OptionType::String,
        values,
        Some(default),
    )? {
        Value::String(s) => Ok(Some(agent.str_lossy(s))),
        _ => Ok(None),
    }
}

/// GetBooleanOrStringNumberFormatOption ( _options_, _propertyKey_, _stringValues_, _fallback_ )
pub(crate) fn get_boolean_or_string_number_format_option(
    agent: &mut Agent,
    options: Object,
    property_key: &str,
    string_values: &[&str],
    fallback: Value,
) -> JsResult<Value> {
    // 1. Let _value_ be ? Get(_options_, _propertyKey_).
    let key = agent.key(property_key);
    let value = ops::get(agent, options, key)?;
    // 2. If _value_ is *undefined*, return _fallback_.
    if matches!(value, Value::Undefined) {
        return Ok(fallback);
    }
    // 3. If _value_ is *true*, return *true*.
    if matches!(value, Value::Boolean(true)) {
        return Ok(value);
    }
    // 4. If ToBoolean(_value_) is *false*, return *false*.
    if !ops::to_boolean(agent, value) {
        return Ok(Value::Boolean(false));
    }
    // 5. Set _value_ to ? ToString(_value_).
    let s = ops::to_string(agent, value)?;
    // 6. If _stringValues_ does not contain _value_, throw a *RangeError* exception.
    if !string_values
        .iter()
        .any(|v| agent.str(s).iter().copied().eq(v.encode_utf16()))
    {
        return Err(agent.throw_range_error(&format!("invalid value for {property_key}")));
    }
    // 7. Return _value_.
    Ok(Value::String(s))
}

/// DefaultNumberOption ( _value_, _minimum_, _maximum_, _fallback_ )
pub(crate) fn default_number_option(
    agent: &mut Agent,
    value: Value,
    minimum: f64,
    maximum: f64,
    fallback: Option<f64>,
) -> JsResult<Option<f64>> {
    // 1. If _value_ is *undefined*, return _fallback_.
    if matches!(value, Value::Undefined) {
        return Ok(fallback);
    }
    // 2. Set _value_ to ? ToNumber(_value_).
    let value = ops::to_number(agent, value)?;
    // 3. If _value_ is not a finite Number or ℝ(_value_) < _minimum_ or ℝ(_value_) > _maximum_, throw a *RangeError* exception.
    if !value.is_finite() || value < minimum || value > maximum {
        return Err(agent.throw_range_error("number option out of range"));
    }
    // 4. Return floor(ℝ(_value_)).
    Ok(Some(value.floor()))
}

/// GetNumberOption ( _options_, _propertyKey_, _minimum_, _maximum_, _fallback_ )
pub(crate) fn get_number_option(
    agent: &mut Agent,
    options: Object,
    property_key: &str,
    minimum: f64,
    maximum: f64,
    fallback: Option<f64>,
) -> JsResult<Option<f64>> {
    // 1. Let _value_ be ? Get(_options_, _propertyKey_).
    let key = agent.key(property_key);
    let value = ops::get(agent, options, key)?;
    // 2. Return ? DefaultNumberOption(_value_, _minimum_, _maximum_, _fallback_).
    default_number_option(agent, value, minimum, maximum, fallback)
}
