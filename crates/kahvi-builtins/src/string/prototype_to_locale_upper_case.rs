//! String.prototype.toLocaleUpperCase ( [ _reserved1_ [ , _reserved2_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.tolocaleuppercase

use kahvi_core::prelude::*;
use kahvi_unicode::to_uppercase;

use crate::support::intl::{
    canonicalize_locale_list, default_locale, lookup_matching_locale_by_prefix,
};

/// String.prototype.toLocaleUpperCase
pub(crate) fn prototype_to_locale_upper_case(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    if agent.intl {
        let locales = args.first().copied().unwrap_or(Value::Undefined);
        // 1. Let _thisValue_ be ? RequireObjectCoercible(*this* value).
        ops::require_object_coercible(agent, this)?;
        // 2. Let _string_ be ? ToString(_thisValue_).
        let string = ops::to_string(agent, this)?;
        // 3. Return ? TransformCase(_string_, _locales_, ~upper~).
        return transform_case(agent, string, locales, false);
    }
    // An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used:
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
    // It works exactly the same as `toUpperCase` except that it is intended to yield a locale-sensitive result corresponding with conventions of the host environment's current locale. There will only be a difference in the few cases (such as Turkish) where the rules for that language conflict with the regular Unicode case mappings.
    // The meaning of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.
    //
    // This engine does not include ECMA-402, so the steps below are those of `String.prototype.toUpperCase`.
    // 1. Let _thisValue_ be the *this* value.
    let this_value = this;
    // 2. Perform ? RequireObjectCoercible(_thisValue_).
    ops::require_object_coercible(agent, this_value)?;
    // 3. Let _string_ be ? ToString(_thisValue_).
    let string = ops::to_string(agent, this_value)?;
    // 4. Let _sText_ be StringToCodePoints(_string_).
    let s_text = string_to_code_points(agent.str(string));
    // 5. Let _upperText_ be toUppercase(_sText_), according to the Unicode Default Case Conversion algorithm.
    let upper_text = to_uppercase_code_points(&s_text);
    // 6. Let _uppercaseString_ be CodePointsToString(_upperText_).
    let uppercase_string = code_points_to_string(&upper_text);
    // 7. Return _uppercaseString_.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (args, new_target);
    Ok(Value::String(agent.intern(&uppercase_string)))
}

/// TransformCase ( _string_, _locales_, _targetCase_ ); `lower` is
/// _targetCase_ = ~lower~, otherwise ~upper~.
pub(super) fn transform_case(
    agent: &mut Agent,
    string: JsString,
    locales: Value,
    lower: bool,
) -> JsResult<Value> {
    // 1. Let _requestedLocales_ be ? CanonicalizeLocaleList(_locales_).
    let requested_locales = canonicalize_locale_list(agent, locales)?;
    // 2. If _requestedLocales_ is not an empty List, then
    let requested_locale = match requested_locales.first() {
        // a. Let _requestedLocale_ be _requestedLocales_[0].
        Some(l) => l.clone(),
        // 3. Else,
        // a. Let _requestedLocale_ be DefaultLocale().
        None => default_locale().to_string(),
    };
    // 4. Let _availableLocales_ be an Available Locales List which includes the language tags for which the Unicode Character Database contains language-sensitive case mappings.
    let available_locales: Vec<String> = ["az", "el", "lt", "tr"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    // 5. Let _match_ be LookupMatchingLocaleByPrefix(_availableLocales_, « _requestedLocale_ »).
    let r#match = lookup_matching_locale_by_prefix(&available_locales, &[requested_locale]);
    // 6. If _match_ is not *undefined*, let _locale_ be _match_.[[locale]]; else let _locale_ be *"und"*.
    let locale = match r#match {
        Some(m) => m.locale,
        None => "und".to_string(),
    };
    let Ok(language) = icu::locale::LanguageIdentifier::try_from_str(&locale) else {
        return Err(agent.not_implemented("TransformCase: unparsable resolved locale"));
    };
    // 7. Let _codePoints_ be StringToCodePoints(_string_).
    // (Runs of scalar values are mapped with ICU4X; a lone surrogate has no
    // case mapping and is neither cased nor case-ignorable, so it ends a run.)
    let mapper = icu::casemap::CaseMapper::new();
    let mut out: Vec<u16> = Vec::new();
    let mut run = String::new();
    let flush = |run: &mut String, out: &mut Vec<u16>| {
        // 8. If _targetCase_ is ~lower~, then
        // a. Let _newCodePoints_ be ... a lowercase transformation of _codePoints_ ... using _locale_
        // 9. Else,
        // b. Let _newCodePoints_ be ... an uppercase transformation of _codePoints_ ... using _locale_
        let mapped = if lower {
            mapper.lowercase_to_string(run, &language)
        } else {
            mapper.uppercase_to_string(run, &language)
        };
        out.extend(mapped.encode_utf16());
        run.clear();
    };
    for r in char::decode_utf16(agent.str(string).iter().copied()) {
        match r {
            Ok(c) => run.push(c),
            Err(e) => {
                flush(&mut run, &mut out);
                out.push(e.unpaired_surrogate());
            }
        }
    }
    flush(&mut run, &mut out);
    // 10. Return CodePointsToString(_newCodePoints_).
    Ok(Value::String(agent.intern(&out)))
}

/// toUppercase ( _sText_ ): the full uppercase mapping of each code point.
/// No context-sensitive mappings apply to uppercasing in the default algorithm.
fn to_uppercase_code_points(s_text: &[u32]) -> Vec<u32> {
    let mut upper_text = Vec::with_capacity(s_text.len());
    for &c in s_text {
        match to_uppercase(c) {
            Some(mapping) => upper_text.extend_from_slice(mapping),
            None => upper_text.push(c),
        }
    }
    upper_text
}

/// StringToCodePoints ( _string_ ): a lone surrogate stands for itself.
fn string_to_code_points(units: &[u16]) -> Vec<u32> {
    char::decode_utf16(units.iter().copied())
        .map(|r| match r {
            Ok(c) => c as u32,
            Err(e) => u32::from(e.unpaired_surrogate()),
        })
        .collect()
}

/// CodePointsToString ( _text_ ): UTF16EncodeCodePoint over each code point.
fn code_points_to_string(code_points: &[u32]) -> Vec<u16> {
    let mut out = Vec::with_capacity(code_points.len());
    for &cp in code_points {
        match char::from_u32(cp) {
            Some(c) => {
                let mut buf = [0u16; 2];
                out.extend_from_slice(c.encode_utf16(&mut buf));
            }
            // A lone surrogate.
            None => out.push(cp as u16),
        }
    }
    out
}
