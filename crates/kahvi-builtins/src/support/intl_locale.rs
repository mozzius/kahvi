//! Intl.Locale: abstract operations shared by the Intl.Locale built-ins.
//!
//! The [[InitializedLocale]] internal slots are `support::intl::IntlLocaleSlots`
//! (CanonicalizeLocaleList reads them). The Intl.Locale ticket owns this
//! file. See docs/INTL.md.
#![allow(dead_code)] // helpers for the Intl tickets

use kahvi_core::prelude::*;

use crate::support::intl::{
    IntlLocaleSlots, Keyword, OptionType, canonicalize_u_value, canonicalize_unicode_locale_id,
    coerce_options_to_object, get_option, get_string_option, initialized_locale,
    insert_unicode_extension_and_canonicalize, is_unicode_type, is_well_formed_language_tag,
    remove_unicode_extension, unicode_extension_components, unicode_extension_of,
};

/// The [[InitializedLocale]] slots of _loc_, for RequireInternalSlot(_loc_, [[InitializedLocale]]).
pub(crate) fn require_locale(agent: &mut Agent, loc: Value) -> JsResult<IntlLocaleSlots> {
    if let Value::Object(o) = loc
        && let Some(slots) = agent.native::<IntlLocaleSlots>(o)
    {
        return Ok(slots.clone());
    }
    Err(agent.throw_type_error("receiver is not an Intl.Locale"))
}

/// Steps 3-4 of maximize and minimize: apply Add Likely Subtags (_maximize_)
/// or Remove Likely Subtags to the [[Locale]] of _loc_ and
/// `Construct(%Intl.Locale%, « result »)`.
pub(crate) fn construct_with_likely_subtags(
    agent: &mut Agent,
    loc: &IntlLocaleSlots,
    maximize: bool,
) -> JsResult<Value> {
    // [[Locale]] is well-formed and canonical; a tag ICU4X cannot parse is a
    // gap in ICU4X, not an error of the Likely Subtags algorithm.
    let Ok(mut parsed) = icu::locale::Locale::try_from_str(&loc.locale) else {
        return Err(agent.not_implemented(&format!(
            "Intl.Locale likely subtags of {:?} (ICU4X cannot parse it)",
            loc.locale
        )));
    };
    // ICU4X's algorithms do not signal errors, so the "if an error is
    // signaled" fallback never applies.
    let expander = icu::locale::LocaleExpander::new_extended();
    if maximize {
        // ICU4X leaves a bare "und" as it is rather than apply CLDR's
        // und -> en-Latn-US entry.
        if parsed.id.language.is_unknown()
            && parsed.id.script.is_none()
            && parsed.id.region.is_none()
        {
            return Err(agent.not_implemented(
                "Intl.Locale.prototype.maximize: likely subtags of the bare \"und\" language (ICU4X leaves it unchanged)",
            ));
        }
        expander.maximize(&mut parsed.id);
    } else {
        expander.minimize(&mut parsed.id);
    }
    let result = parsed.to_string();
    let ctor = agent.intrinsic("%Intl.Locale%");
    let tag = Value::String(agent.string(&result));
    Ok(Value::Object(ops::construct(agent, ctor, &[tag], None)?))
}

/// A JS string for an optional slot value (*undefined* when absent).
pub(crate) fn optional_string(agent: &mut Agent, s: Option<&str>) -> Value {
    match s {
        Some(s) => Value::String(agent.string(s)),
        None => Value::Undefined,
    }
}

/// The accessor shared by the string-valued getters: RequireInternalSlot(_loc_, [[InitializedLocale]]),
/// then the String (or *undefined*) that _get_ reads from its slots.
pub(crate) fn locale_string_getter(
    agent: &mut Agent,
    this: Value,
    get: impl FnOnce(&IntlLocaleSlots) -> Option<String>,
) -> JsResult<Value> {
    let loc = require_locale(agent, this)?;
    let s = get(&loc);
    Ok(optional_string(agent, s.as_deref()))
}

/// GetLocaleBaseName ( _locale_ )
pub(crate) fn get_locale_base_name(locale: &str) -> String {
    // 1. Assert: _locale_ can be matched by the unicode_locale_id Unicode locale nonterminal.
    // 2. Return the longest prefix of _locale_ matched by the unicode_language_id Unicode locale nonterminal.
    let mut end = locale.len();
    let mut pos = 0;
    for (i, subtag) in locale.split('-').enumerate() {
        // A singleton subtag starts an extension.
        if i > 0 && subtag.len() == 1 {
            end = pos - 1;
            break;
        }
        pos += subtag.len() + 1;
    }
    locale[..end].to_string()
}

fn is_alpha(s: &str) -> bool {
    s.bytes().all(|b| b.is_ascii_alphabetic())
}

fn is_language_subtag(s: &str) -> bool {
    matches!(s.len(), 2 | 3 | 5..=8) && is_alpha(s)
}

fn is_script_subtag(s: &str) -> bool {
    s.len() == 4 && is_alpha(s)
}

fn is_region_subtag(s: &str) -> bool {
    (s.len() == 2 && is_alpha(s)) || (s.len() == 3 && s.bytes().all(|b| b.is_ascii_digit()))
}

fn is_variant_subtag(s: &str) -> bool {
    let alnum = s.bytes().all(|c| c.is_ascii_alphanumeric());
    (5..=8).contains(&s.len()) && alnum || s.len() == 4 && alnum && s.as_bytes()[0].is_ascii_digit()
}

/// GetLocaleLanguage ( _locale_ )
pub(crate) fn get_locale_language(locale: &str) -> String {
    // 1. Let _baseName_ be GetLocaleBaseName(_locale_).
    let base_name = get_locale_base_name(locale);
    // 2. Let _subtags_ be StringSplitToList(_baseName_, *"-"*).
    // 3. Assert: _subtags_[0] is matched by the unicode_language_subtag Unicode locale nonterminal.
    // 4. Return _subtags_[0].
    base_name.split('-').next().unwrap_or("").to_string()
}

/// GetLocaleScript ( _locale_ )
pub(crate) fn get_locale_script(locale: &str) -> Option<String> {
    // 1. Let _baseName_ be GetLocaleBaseName(_locale_).
    let base_name = get_locale_base_name(locale);
    // 3. Let _subtags_ be StringSplitToList(_baseName_, *"-"*).
    let subtags: Vec<&str> = base_name.split('-').collect();
    // 5. If _subtags_ has at least 2 elements and _subtags_[1] is matched by the unicode_script_subtag Unicode locale nonterminal, return _subtags_[1].
    if subtags.len() >= 2 && is_script_subtag(subtags[1]) {
        return Some(subtags[1].to_string());
    }
    // 6. Return *undefined*.
    None
}

/// GetLocaleRegion ( _locale_ )
pub(crate) fn get_locale_region(locale: &str) -> Option<String> {
    // 1. Let _baseName_ be GetLocaleBaseName(_locale_).
    let base_name = get_locale_base_name(locale);
    // 3. Let _subtags_ be StringSplitToList(_baseName_, *"-"*).
    let subtags: Vec<&str> = base_name.split('-').collect();
    // 5. If _subtags_ has at least 2 elements and _subtags_[1] is matched by the unicode_region_subtag Unicode locale nonterminal, return _subtags_[1].
    if subtags.len() >= 2 && is_region_subtag(subtags[1]) {
        return Some(subtags[1].to_string());
    }
    // 6. If _subtags_ has at least 3 elements and _subtags_[2] is matched by the unicode_region_subtag Unicode locale nonterminal, return _subtags_[2].
    if subtags.len() >= 3 && is_region_subtag(subtags[2]) {
        return Some(subtags[2].to_string());
    }
    // 7. Return *undefined*.
    None
}

/// GetLocaleVariants ( _locale_ )
pub(crate) fn get_locale_variants(locale: &str) -> Option<String> {
    // 1. Let _baseName_ be GetLocaleBaseName(_locale_).
    let base_name = get_locale_base_name(locale);
    // 3. Let _subtags_ be StringSplitToList(_baseName_, *"-"*).
    let mut subtags = base_name.split('-');
    // 5. Let _dashIndex_ be the length of _subtags_[0].
    let mut dash_index = subtags.next().map_or(0, str::len);
    // 6. Remove the first element from _subtags_.
    // 7. For each String _subtag_ of _subtags_, do
    for subtag in subtags {
        // a. If _subtag_ is matched by the unicode_variant_subtag Unicode locale nonterminal, then
        if is_variant_subtag(subtag) {
            // i. Return the substring of _baseName_ from _dashIndex_ + 1.
            return Some(base_name[dash_index + 1..].to_string());
        }
        // b. Let _subtagLength_ be the length of _subtag_.
        // c. Set _dashIndex_ to _dashIndex_ + _subtagLength_ + 1.
        dash_index += subtag.len() + 1;
    }
    // 8. Return *undefined*.
    None
}

/// WeekdayToUValue ( _fw_ )
pub(crate) fn weekday_to_u_value(fw: &str) -> String {
    // 1. For each row of [table-locale-weekday-string-value], except the header row, in table order, do
    let string = match fw {
        "0" | "7" => "sun",
        "1" => "mon",
        "2" => "tue",
        "3" => "wed",
        "4" => "thu",
        "5" => "fri",
        "6" => "sat",
        // 2. Return _fw_.
        _ => return fw.to_string(),
    };
    string.to_string()
}

/// GetOption with ~string~, ~empty~ values and an *undefined* default.
fn string_option(agent: &mut Agent, options: Object, key: &str) -> JsResult<Option<String>> {
    get_string_option(agent, options, key, &[], None)
}

/// UpdateLanguageId ( _tag_, _options_ )
pub(crate) fn update_language_id(
    agent: &mut Agent,
    tag: &str,
    options: Object,
) -> JsResult<String> {
    // 1. Let _baseName_ be GetLocaleBaseName(_tag_).
    let base_name = get_locale_base_name(tag);
    // 2. Let _language_ be ? GetOption(_options_, *"language"*, ~string~, ~empty~, GetLocaleLanguage(_baseName_)).
    let default = get_locale_language(&base_name);
    let language =
        get_string_option(agent, options, "language", &[], Some(&default))?.unwrap_or(default);
    // 3. If _language_ cannot be matched by the unicode_language_subtag Unicode locale nonterminal, throw a *RangeError* exception.
    if !is_language_subtag(&language) {
        return Err(agent.throw_range_error("invalid language"));
    }
    // 4. Let _script_ be ? GetOption(_options_, *"script"*, ~string~, ~empty~, GetLocaleScript(_baseName_)).
    let default = get_locale_script(&base_name);
    let script = get_string_option(agent, options, "script", &[], default.as_deref())?;
    // 5. If _script_ is not *undefined*, then
    if let Some(script) = &script {
        // a. If _script_ cannot be matched by the unicode_script_subtag Unicode locale nonterminal, throw a *RangeError* exception.
        if !is_script_subtag(script) {
            return Err(agent.throw_range_error("invalid script"));
        }
    }
    // 6. Let _region_ be ? GetOption(_options_, *"region"*, ~string~, ~empty~, GetLocaleRegion(_baseName_)).
    let default = get_locale_region(&base_name);
    let region = get_string_option(agent, options, "region", &[], default.as_deref())?;
    // 7. If _region_ is not *undefined*, then
    if let Some(region) = &region {
        // a. If _region_ cannot be matched by the unicode_region_subtag Unicode locale nonterminal, throw a *RangeError* exception.
        if !is_region_subtag(region) {
            return Err(agent.throw_range_error("invalid region"));
        }
    }
    // 8. Let _variants_ be ? GetOption(_options_, *"variants"*, ~string~, ~empty~, GetLocaleVariants(_baseName_)).
    let default = get_locale_variants(&base_name);
    let variants = get_string_option(agent, options, "variants", &[], default.as_deref())?;
    // 9. If _variants_ is not *undefined*, then
    if let Some(variants) = &variants {
        // a. If _variants_ is the empty String, throw a *RangeError* exception.
        if variants.is_empty() {
            return Err(agent.throw_range_error("invalid variants"));
        }
        // b. Let _lowerVariants_ be the ASCII-lowercase of _variants_.
        let lower_variants = variants.to_ascii_lowercase();
        // c. Let _variantSubtags_ be StringSplitToList(_lowerVariants_, *"-"*).
        let variant_subtags: Vec<&str> = lower_variants.split('-').collect();
        // d. For each element _variant_ of _variantSubtags_, do
        for variant in &variant_subtags {
            // i. If _variant_ cannot be matched by the unicode_variant_subtag Unicode locale nonterminal, throw a *RangeError* exception.
            if !is_variant_subtag(variant) {
                return Err(agent.throw_range_error("invalid variants"));
            }
        }
        // e. If _variantSubtags_ contains any duplicate elements, throw a *RangeError* exception.
        for (i, variant) in variant_subtags.iter().enumerate() {
            if variant_subtags[..i].contains(variant) {
                return Err(agent.throw_range_error("duplicate variants"));
            }
        }
    }
    // 10. Let _allExtensions_ be the suffix of _tag_ following _baseName_.
    let all_extensions = &tag[base_name.len()..];
    // 11. Let _newTag_ be _language_.
    let mut new_tag = language;
    // 12. If _script_ is not *undefined*, set _newTag_ to the string-concatenation of _newTag_, *"-"*, and _script_.
    if let Some(script) = script {
        new_tag = format!("{new_tag}-{script}");
    }
    // 13. If _region_ is not *undefined*, set _newTag_ to the string-concatenation of _newTag_, *"-"*, and _region_.
    if let Some(region) = region {
        new_tag = format!("{new_tag}-{region}");
    }
    // 14. If _variants_ is not *undefined*, set _newTag_ to the string-concatenation of _newTag_, *"-"*, and _variants_.
    if let Some(variants) = variants {
        new_tag = format!("{new_tag}-{variants}");
    }
    // 15. Set _newTag_ to the string-concatenation of _newTag_ and _allExtensions_.
    new_tag.push_str(all_extensions);
    // 16. Return _newTag_.
    Ok(new_tag)
}

/// The _opt_ Record of the Intl.Locale constructor: [[ca]], [[co]], [[fw]], [[hc]], [[kf]], [[kn]], [[nu]].
#[derive(Debug, Default)]
struct LocaleOptions {
    ca: Option<String>,
    co: Option<String>,
    fw: Option<String>,
    hc: Option<String>,
    kf: Option<String>,
    kn: Option<String>,
    nu: Option<String>,
}

/// MakeLocaleRecord ( _tag_, _options_, _localeExtensionKeys_ ), with
/// %Intl.Locale%.[[LocaleExtensionKeys]] « "ca", "co", "fw", "hc", "kf", "kn", "nu" »,
/// returned as the slots steps 36-43 of the constructor set from it.
fn make_locale_record(tag: &str, options: &LocaleOptions) -> IntlLocaleSlots {
    // 1. If _tag_ contains a substring that is a Unicode locale extension sequence, then
    let (attributes, mut keywords) = match unicode_extension_of(tag) {
        // a. Let _extension_ be the String value consisting of the substring of the Unicode locale extension sequence within _tag_.
        // b. Let _components_ be UnicodeExtensionComponents(_extension_).
        Some(extension) => unicode_extension_components(extension),
        // 2. Else, let _attributes_ and _keywords_ be new empty Lists.
        None => (Vec::new(), Vec::new()),
    };
    // 3. Let _result_ be a new Record.
    let mut values: Vec<Option<String>> = Vec::new();
    // 4. For each element _key_ of _localeExtensionKeys_, do
    for (key, override_value) in [
        ("ca", &options.ca),
        ("co", &options.co),
        ("fw", &options.fw),
        ("hc", &options.hc),
        ("kf", &options.kf),
        ("kn", &options.kn),
        ("nu", &options.nu),
    ] {
        // a. If _keywords_ contains an element whose [[Key]] is _key_, then
        let entry = keywords.iter().position(|k| k.key == key);
        // i.-ii. Let _value_ be _entry_.[[Value]].
        // b. Else, let _value_ be *undefined*.
        let mut value = entry.map(|i| keywords[i].value.clone());
        // e. If _overrideValue_ is not *undefined*, then
        if let Some(override_value) = override_value {
            // i. Set _value_ to CanonicalizeUValue(_key_, _overrideValue_).
            let mut canonical = canonicalize_u_value(key, override_value);
            // (UTS #35 canonical syntax: the type "true" is dropped from a keyword.)
            if canonical == "true" {
                canonical.clear();
            }
            match entry {
                // ii. If _entry_ is not ~empty~, set _entry_.[[Value]] to _value_.
                Some(i) => keywords[i].value = canonical.clone(),
                // iii. Else, append the Record { [[Key]]: _key_, [[Value]]: _value_ } to _keywords_.
                None => keywords.push(Keyword {
                    key: key.to_string(),
                    value: canonical.clone(),
                }),
            }
            value = Some(canonical);
        }
        // f. Set _result_.[[<_key_>]] to _value_.
        values.push(value);
    }
    // 5. Let _locale_ be the String value that is _tag_ with any Unicode locale extension sequences removed.
    let locale = remove_unicode_extension(tag);
    // 6. If _attributes_ is not empty or _keywords_ is not empty, then
    let locale = if !attributes.is_empty() || !keywords.is_empty() {
        // a. Set _result_.[[locale]] to InsertUnicodeExtensionAndCanonicalize(_locale_, _attributes_, _keywords_).
        insert_unicode_extension_and_canonicalize(&locale, &attributes, &keywords)
    } else {
        // 7. Else, set _result_.[[locale]] to CanonicalizeUnicodeLocaleId(_locale_).
        canonicalize_unicode_locale_id(&locale)
    };
    // 8. Return _result_.
    let mut it = values.into_iter();
    let mut next = || it.next().flatten();
    let (ca, co, fw, hc, kf, kn, nu) = (next(), next(), next(), next(), next(), next(), next());
    IntlLocaleSlots {
        // 36. Set _locale_.[[Locale]] to _localeRecord_.[[locale]].
        locale,
        // 37. Set _locale_.[[Calendar]] to _localeRecord_.[[ca]].
        calendar: ca,
        // 38. Set _locale_.[[Collation]] to _localeRecord_.[[co]].
        collation: co,
        // 40. Set _locale_.[[HourCycle]] to _localeRecord_.[[hc]].
        hour_cycle: hc,
        // 41. a. Set _locale_.[[CaseFirst]] to _localeRecord_.[[kf]].
        case_first: kf,
        // 42. a. If _localeRecord_.[[kn]] is either *"true"* or the empty String, then [[Numeric]] is *true*; b. else *false*.
        numeric: Some(kn.is_some_and(|kn| kn == "true" || kn.is_empty())),
        // 43. Set _locale_.[[NumberingSystem]] to _localeRecord_.[[nu]].
        numbering_system: nu,
        // 39. Set _locale_.[[FirstDayOfWeek]] to _localeRecord_.[[fw]].
        first_day_of_week: fw,
    }
}

fn check_type(agent: &mut Agent, value: &Option<String>, name: &str) -> JsResult<()> {
    match value {
        Some(v) if !is_unicode_type(v) => Err(agent.throw_range_error(&format!("invalid {name}"))),
        _ => Ok(()),
    }
}

/// Steps 7-43 of the Intl.Locale constructor: the slots for _tag_ and _options_.
pub(crate) fn initialize_locale(
    agent: &mut Agent,
    tag: Value,
    options: Value,
) -> JsResult<IntlLocaleSlots> {
    // 7. If _tag_ is not a String and _tag_ is not an Object, throw a *TypeError* exception.
    if !matches!(tag, Value::String(_) | Value::Object(_)) {
        return Err(agent.throw_type_error("locale tag must be a string or object"));
    }
    let tag = match initialized_locale(agent, tag) {
        // 8. If _tag_ is an Object and _tag_ has an [[InitializedLocale]] internal slot, then
        // a. Set _tag_ to _tag_.[[Locale]].
        Some(locale) => locale,
        // 9. Else,
        // a. Set _tag_ to ? ToString(_tag_).
        None => {
            let s = ops::to_string(agent, tag)?;
            agent.str_lossy(s)
        }
    };
    // 10. Set _options_ to ? CoerceOptionsToObject(_options_).
    let options = coerce_options_to_object(agent, options)?;
    // 11. If IsWellFormedLanguageTag(_tag_) is *false*, throw a *RangeError* exception.
    if !tag.is_ascii() || !is_well_formed_language_tag(&tag) {
        return Err(agent.throw_range_error("invalid language tag"));
    }
    // 13. Set _tag_ to CanonicalizeUnicodeLocaleId(_tag_).
    let tag = canonicalize_unicode_locale_id(&tag);
    // 14. Set _tag_ to ? UpdateLanguageId(_tag_, _options_).
    let tag = update_language_id(agent, &tag, options)?;
    // 15. Let _opt_ be a new Record.
    // 16. Let _calendar_ be ? GetOption(_options_, *"calendar"*, ~string~, ~empty~, *undefined*).
    let mut opt = LocaleOptions {
        ca: string_option(agent, options, "calendar")?,
        ..Default::default()
    };
    // 17. If _calendar_ is not *undefined*, then
    // a. If _calendar_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
    check_type(agent, &opt.ca, "calendar")?;
    // 19. Let _collation_ be ? GetOption(_options_, *"collation"*, ~string~, ~empty~, *undefined*).
    opt.co = string_option(agent, options, "collation")?;
    // 20. If _collation_ is not *undefined*, then
    // a. If _collation_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
    check_type(agent, &opt.co, "collation")?;
    // 22. Let _fw_ be ? GetOption(_options_, *"firstDayOfWeek"*, ~string~, ~empty~, *undefined*).
    opt.fw = string_option(agent, options, "firstDayOfWeek")?;
    // 23. If _fw_ is not *undefined*, then
    if let Some(fw) = &opt.fw {
        // a. Set _fw_ to WeekdayToUValue(_fw_).
        opt.fw = Some(weekday_to_u_value(fw));
        // b. If _fw_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
        check_type(agent, &opt.fw, "firstDayOfWeek")?;
    }
    // 25. Let _hc_ be ? GetOption(_options_, *"hourCycle"*, ~string~, « *"h11"*, *"h12"*, *"h23"*, *"h24"* », *undefined*).
    opt.hc = get_string_option(
        agent,
        options,
        "hourCycle",
        &["h11", "h12", "h23", "h24"],
        None,
    )?;
    // 27. Let _kf_ be ? GetOption(_options_, *"caseFirst"*, ~string~, « *"upper"*, *"lower"*, *"false"* », *undefined*).
    opt.kf = get_string_option(
        agent,
        options,
        "caseFirst",
        &["upper", "lower", "false"],
        None,
    )?;
    // 29. Let _kn_ be ? GetOption(_options_, *"numeric"*, ~boolean~, ~empty~, *undefined*).
    let kn = get_option(
        agent,
        options,
        "numeric",
        OptionType::Boolean,
        None,
        Some(Value::Undefined),
    )?;
    // 30. If _kn_ is not *undefined*, set _kn_ to ! ToString(_kn_).
    if let Value::Boolean(b) = kn {
        opt.kn = Some(b.to_string());
    }
    // 32. Let _numberingSystem_ be ? GetOption(_options_, *"numberingSystem"*, ~string~, ~empty~, *undefined*).
    opt.nu = string_option(agent, options, "numberingSystem")?;
    // 33. If _numberingSystem_ is not *undefined*, then
    // a. If _numberingSystem_ cannot be matched by the type Unicode locale nonterminal, throw a *RangeError* exception.
    check_type(agent, &opt.nu, "numberingSystem")?;
    // 35. Let _localeRecord_ be MakeLocaleRecord(_tag_, _opt_, _localeExtensionKeys_).
    Ok(make_locale_record(&tag, &opt))
}
