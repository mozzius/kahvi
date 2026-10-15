//! Intl.ListFormat: the [[InitializedListFormat]] internal slots and the
//! abstract operations shared by its built-ins.
#![allow(dead_code)] // helpers for the Intl tickets

use icu::list::ListFormatter;
use icu::list::options::{ListFormatterOptions, ListLength};
use icu::locale::Locale;
use kahvi_core::gc::{Trace, Tracer};
use kahvi_core::ops::iteration::{IteratorKind, get_iterator, iterator_close, iterator_step_value};
use kahvi_core::prelude::*;

/// The internal slots of an Intl.ListFormat: [[InitializedListFormat]],
/// [[Locale]], [[Type]], [[Style]], [[Templates]].
///
/// [[Templates]] is the ICU4X list formatter for the locale, type and style:
/// it holds the start, middle, end and pair patterns and selects among them
/// from the neighbouring elements (the implementation-defined index _n_).
#[derive(Debug, Default)]
pub(crate) struct ListFormatSlots {
    /// [[Locale]]
    pub locale: String,
    /// [[Type]]: "conjunction", "disjunction" or "unit".
    pub r#type: String,
    /// [[Style]]: "long", "short" or "narrow".
    pub style: String,
    /// [[Templates]]
    pub templates: Option<ListFormatter>,
}

impl Trace for ListFormatSlots {
    fn trace(&self, _: &mut Tracer) {}
}

/// A Record with fields [[Type]] ("element" or "literal") and [[Value]].
#[derive(Debug, Clone)]
pub(crate) struct ListPart {
    pub is_element: bool,
    pub value: Vec<u16>,
}

/// The [[Templates]] for a locale, type and style (steps 11-13 of the
/// constructor), from ICU4X data.
pub(crate) fn templates_for(
    agent: &mut Agent,
    locale: &str,
    r#type: &str,
    style: &str,
) -> JsResult<ListFormatter> {
    let Ok(locale) = Locale::try_from_str(locale) else {
        return Err(agent.throw_range_error("invalid locale"));
    };
    let length = match style {
        "short" => ListLength::Short,
        "narrow" => ListLength::Narrow,
        _ => ListLength::Wide,
    };
    let options = ListFormatterOptions::default().with_length(length);
    let formatter = match r#type {
        "disjunction" => ListFormatter::try_new_or((&locale).into(), options),
        "unit" => ListFormatter::try_new_unit((&locale).into(), options),
        _ => ListFormatter::try_new_and((&locale).into(), options),
    };
    match formatter {
        Ok(f) => Ok(f),
        Err(_) => Err(agent.not_implemented("Intl.ListFormat list patterns for this locale")),
    }
}

/// StringListFromIterable ( _iterable_ )
pub(crate) fn string_list_from_iterable(
    agent: &mut Agent,
    iterable: Value,
) -> JsResult<Vec<JsString>> {
    // 1. If _iterable_ is *undefined*, then
    if matches!(iterable, Value::Undefined) {
        // a. Return a new empty List.
        return Ok(Vec::new());
    }
    // 2. Let _iteratorRecord_ be ? GetIterator(_iterable_, ~sync~).
    let mut iterator_record = get_iterator(agent, iterable, IteratorKind::Sync)?;
    // 3. Let _list_ be a new empty List.
    let mut list = Vec::new();
    // 4. Repeat,
    loop {
        // a. Let _next_ be ? IteratorStepValue(_iteratorRecord_).
        let next = iterator_step_value(agent, &mut iterator_record)?;
        // b. If _next_ is ~done~, then
        let Some(next) = next else {
            // i. Return _list_.
            return Ok(list);
        };
        // c. If _next_ is not a String, then
        let Value::String(next) = next else {
            // i. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
            let error = Err(agent.throw_type_error("list elements must be strings"));
            // ii. Return ? IteratorClose(_iteratorRecord_, _error_).
            return iterator_close(agent, &iterator_record, error);
        };
        // d. Append _next_ to _list_.
        list.push(next);
    }
}

/// CreatePartsFromList ( _listFormat_, _list_ )
///
/// The ICU4X formatter chooses the patterns (the index _n_ and the
/// DeconstructPattern substitution) itself and gives only the formatted
/// string, so the parts are recovered from it: every element is formatted
/// followed by a marker character that occurs in no element, which makes
/// each element's end, and so the literal before it, unambiguous.
pub(crate) fn create_parts_from_list(
    agent: &mut Agent,
    list_format: Object,
    list: &[JsString],
) -> JsResult<Vec<ListPart>> {
    // 1. Let _size_ be the number of elements of _list_.
    let size = list.len();
    // 2. If _size_ is 0, then
    if size == 0 {
        // a. Return a new empty List.
        return Ok(Vec::new());
    }
    let elements: Vec<String> = list.iter().map(|s| agent.str_lossy(*s)).collect();
    let Some(slots) = agent.native::<ListFormatSlots>(list_format) else {
        return Err(agent.throw_type_error("not an Intl.ListFormat"));
    };
    let Some(formatter) = &slots.templates else {
        return Err(agent.not_implemented("Intl.ListFormat without templates"));
    };
    // Steps 3-8: the patterns for pair, start, middle and end, applied by ICU4X.
    let plain = formatter.format(elements.iter()).to_string();
    let Some(marker) = ('\u{E000}'..='\u{F8FF}').find(|m| !elements.iter().any(|e| e.contains(*m)))
    else {
        return Err(agent
            .not_implemented("Intl.ListFormat parts of lists using every private-use character"));
    };
    let marked: Vec<String> = elements.iter().map(|e| format!("{e}{marker}")).collect();
    let formatted = formatter.format(marked.iter()).to_string();
    // The text before each marker is the literal before the element, then the element.
    let mut segments: Vec<&str> = formatted.split(marker).collect();
    // The last segment follows the last element: only a literal.
    let tail = segments.pop().unwrap_or("");
    let mut parts = Vec::new();
    let mut rebuilt = String::new();
    for (segment, (element, original)) in segments.iter().zip(elements.iter().zip(list)) {
        let Some(literal) = segment.strip_suffix(element.as_str()) else {
            return Err(agent.not_implemented(
                "Intl.ListFormat parts: element not found in the formatted list",
            ));
        };
        if !literal.is_empty() {
            parts.push(ListPart {
                is_element: false,
                value: literal.encode_utf16().collect(),
            });
        }
        parts.push(ListPart {
            is_element: true,
            value: agent.str(*original).to_vec(),
        });
        rebuilt.push_str(literal);
        rebuilt.push_str(element);
    }
    rebuilt.push_str(tail);
    if !tail.is_empty() {
        parts.push(ListPart {
            is_element: false,
            value: tail.encode_utf16().collect(),
        });
    }
    // The marker must not have changed which patterns apply.
    if rebuilt != plain {
        return Err(agent.not_implemented("Intl.ListFormat parts for this list"));
    }
    // 9. Return _parts_.
    Ok(parts)
}

/// FormatList ( _listFormat_, _list_ )
pub(crate) fn format_list(
    agent: &mut Agent,
    list_format: Object,
    list: &[JsString],
) -> JsResult<Value> {
    // 1. Let _parts_ be CreatePartsFromList(_listFormat_, _list_).
    let parts = create_parts_from_list(agent, list_format, list)?;
    // 2. Let _result_ be the empty String.
    let mut result: Vec<u16> = Vec::new();
    // 3. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
    for part in &parts {
        // a. Set _result_ to the string-concatenation of _result_ and _part_.[[Value]].
        result.extend_from_slice(&part.value);
    }
    // 4. Return _result_.
    Ok(Value::String(agent.intern(&result)))
}

/// FormatListToParts ( _listFormat_, _list_ )
pub(crate) fn format_list_to_parts(
    agent: &mut Agent,
    list_format: Object,
    list: &[JsString],
) -> JsResult<Value> {
    // 1. Let _parts_ be CreatePartsFromList(_listFormat_, _list_).
    let parts = create_parts_from_list(agent, list_format, list)?;
    // 2. Let _result_ be ! ArrayCreate(0).
    // 3. Let _n_ be 0.
    let mut objects = Vec::new();
    // 4. For each Record { [[Type]], [[Value]] } _part_ of _parts_, do
    for part in &parts {
        // a. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
        let proto = agent.intrinsic("%Object.prototype%");
        let obj = agent.ordinary_object_create(Some(proto));
        // b. Perform ! CreateDataPropertyOrThrow(_obj_, *"type"*, _part_.[[Type]]).
        let key = agent.key("type");
        let type_name = agent.string(if part.is_element {
            "element"
        } else {
            "literal"
        });
        ops::create_data_property_or_throw(agent, obj, key, Value::String(type_name))?;
        // c. Perform ! CreateDataPropertyOrThrow(_obj_, *"value"*, _part_.[[Value]]).
        let key = agent.key("value");
        let value = agent.intern(&part.value);
        ops::create_data_property_or_throw(agent, obj, key, Value::String(value))?;
        // d. Perform ! CreateDataPropertyOrThrow(_result_, ! ToString(𝔽(_n_)), _obj_).
        objects.push(Value::Object(obj));
        // e. Set _n_ to _n_ + 1.
    }
    // 5. Return _result_.
    Ok(Value::Object(ops::create_array_from_list(agent, &objects)))
}
