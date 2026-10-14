//! RegExp internal slots and the abstract operations shared by the RegExp
//! and String built-ins (§22.2.3, §22.2.7). The pattern engine itself is
//! the `kahvi-regexp` crate.
#![allow(dead_code)] // helpers for the RegExp and String tickets

use kahvi_core::object::array::array_create;
use kahvi_core::prelude::*;

/// [[OriginalSource]], [[OriginalFlags]], [[RegExpRecord]] and [[RegExpMatcher]].
pub(crate) struct RegExpData {
    pub original_source: JsString,
    pub original_flags: JsString,
    /// The compiled pattern; `None` between RegExpAlloc and RegExpInitialize.
    pub matcher: Option<kahvi_regexp::Regex>,
}

impl std::fmt::Debug for RegExpData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegExpData").finish_non_exhaustive()
    }
}

/// Whether `o` has a [[RegExpMatcher]] internal slot (installed as the
/// engine's hook for IsRegExp and Object.prototype.toString).
pub(crate) fn has_regexp_matcher(agent: &Agent, o: Object) -> bool {
    agent.native::<RegExpData>(o).is_some()
}

/// RequireInternalSlot(R, [[RegExpMatcher]]): `R` as an Object.
pub(crate) fn require_regexp(agent: &mut Agent, r: Value) -> JsResult<Object> {
    match r {
        Value::Object(o) if has_regexp_matcher(agent, o) => Ok(o),
        _ => Err(agent.throw_type_error("not a RegExp object")),
    }
}

/// RegExpAlloc(newTarget) (§22.2.3.1)
pub(crate) fn regexp_alloc(agent: &mut Agent, new_target: Object) -> JsResult<Object> {
    // 1. Let obj be ? OrdinaryCreateFromConstructor(newTarget, "%RegExp.prototype%",
    //    « [[OriginalSource]], [[OriginalFlags]], [[RegExpRecord]], [[RegExpMatcher]] »).
    let empty = agent.names.empty;
    let data = RegExpData {
        original_source: empty,
        original_flags: empty,
        matcher: None,
    };
    let obj = ops::ordinary_create_from_constructor(
        agent,
        new_target,
        "%RegExp.prototype%",
        ObjectKind::Native(Box::new(data)),
    )?;
    // 2. Perform ! DefinePropertyOrThrow(obj, "lastIndex", PropertyDescriptor
    //    { [[Writable]]: true, [[Enumerable]]: false, [[Configurable]]: false }).
    let key = agent.key("lastIndex");
    let desc = PropertyDescriptor {
        value: None,
        writable: Some(true),
        get: None,
        set: None,
        enumerable: Some(false),
        configurable: Some(false),
    };
    ops::define_property_or_throw(agent, obj, key, desc)?;
    // 3. Return obj.
    Ok(obj)
}

/// RegExpInitialize(obj, pattern, flags) (§22.2.3.3)
pub(crate) fn regexp_initialize(
    agent: &mut Agent,
    obj: Object,
    pattern: Value,
    flags: Value,
) -> JsResult<Object> {
    // 1-2. If pattern is undefined, let P be the empty String; else let P be ? ToString(pattern).
    let p = match pattern {
        Value::Undefined => agent.names.empty,
        v => ops::to_string(agent, v)?,
    };
    // 3-4. If flags is undefined, let F be the empty String; else let F be ? ToString(flags).
    let f = match flags {
        Value::Undefined => agent.names.empty,
        v => ops::to_string(agent, v)?,
    };
    // 5. If F contains any code unit other than "d", "g", "i", "m", "s", "u", "v", or "y",
    //    throw a SyntaxError exception.
    // 6. If F contains any code unit more than once, throw a SyntaxError exception.
    let mut seen = String::new();
    for &u in agent.str(f) {
        let c = char::from_u32(u as u32).filter(|c| "dgimsuvy".contains(*c));
        match c {
            Some(c) if !seen.contains(c) => seen.push(c),
            _ => {
                let msg = format!("invalid regular expression flags '{}'", agent.str_lossy(f));
                return Err(agent.throw_syntax_error(&msg));
            }
        }
    }
    // 7-11. The i, m, s, u and v flags.
    let flags = kahvi_regexp::Flags {
        ignore_case: seen.contains('i'),
        multiline: seen.contains('m'),
        dot_all: seen.contains('s'),
        unicode: seen.contains('u'),
        unicode_sets: seen.contains('v'),
    };
    // 12-16. Let parseResult be ParsePattern(patternText, u, v); a List of errors is a SyntaxError.
    //        (ParsePattern rejects u and v together.)
    if flags.unicode && flags.unicode_sets {
        return Err(agent.throw_syntax_error("regular expression flags 'u' and 'v' are exclusive"));
    }
    let matcher = match kahvi_regexp::compile(agent.str(p), flags) {
        Ok(m) => m,
        Err(e) => {
            let msg = format!(
                "Invalid regular expression: /{}/{}: {}",
                agent.str_lossy(p),
                seen,
                e
            );
            return Err(agent.throw_syntax_error(&msg));
        }
    };
    // 17-22. Set obj.[[OriginalSource]], [[OriginalFlags]], [[RegExpRecord]] and [[RegExpMatcher]].
    let data = agent
        .native_mut::<RegExpData>(obj)
        .expect("RegExpInitialize on an object from RegExpAlloc");
    data.original_source = p;
    data.original_flags = f;
    data.matcher = Some(matcher);
    // 23. Perform ? Set(obj, "lastIndex", +0𝔽, true).
    let key = agent.key("lastIndex");
    ops::set(agent, obj, key, Value::Number(0.0), true)?;
    // 24. Return obj.
    Ok(obj)
}

/// RegExpCreate(P, F) (§22.2.3.2)
pub(crate) fn regexp_create(agent: &mut Agent, pattern: Value, flags: Value) -> JsResult<Object> {
    // 1. Let obj be ! RegExpAlloc(%RegExp%).
    let ctor = agent.intrinsic("%RegExp%");
    let obj = regexp_alloc(agent, ctor)?;
    // 2. Return ? RegExpInitialize(obj, P, F).
    regexp_initialize(agent, obj, pattern, flags)
}

/// RegExpExec(R, S) (§22.2.7.1)
pub(crate) fn regexp_exec(agent: &mut Agent, r: Object, s: JsString) -> JsResult<Value> {
    // 1. Let exec be ? Get(R, "exec").
    let key = agent.key("exec");
    let exec = ops::get(agent, r, key)?;
    // 2. If IsCallable(exec) is true, then
    if ops::is_callable(agent, exec) {
        // a. Let result be ? Call(exec, R, « S »).
        let result = ops::call(agent, exec, Value::Object(r), &[Value::String(s)])?;
        // b. If result is not an Object and result is not null, throw a TypeError exception.
        if !matches!(result, Value::Object(_) | Value::Null) {
            return Err(agent.throw_type_error("exec result must be an object or null"));
        }
        // c. Return result.
        return Ok(result);
    }
    // 3. Perform ? RequireInternalSlot(R, [[RegExpMatcher]]).
    let r = require_regexp(agent, Value::Object(r))?;
    // 4. Return ? RegExpBuiltinExec(R, S).
    regexp_builtin_exec(agent, r, s)
}

/// AdvanceStringIndex(S, index, unicode) (§22.2.7.3)
pub(crate) fn advance_string_index(s: &[u16], index: f64, unicode: bool) -> f64 {
    // 2. If unicode is false, return index + 1.
    if !unicode {
        return index + 1.0;
    }
    // 3-4. If index + 1 ≥ length, return index + 1.
    let length = s.len() as f64;
    if index + 1.0 >= length {
        return index + 1.0;
    }
    // 5-6. Return index + CodePointAt(S, index).[[CodeUnitCount]].
    let i = index as usize;
    let pair = (0xD800..=0xDBFF).contains(&s[i]) && (0xDC00..=0xDFFF).contains(&s[i + 1]);
    index + if pair { 2.0 } else { 1.0 }
}

fn set_last_index(agent: &mut Agent, r: Object, v: f64) -> JsResult<()> {
    let key = agent.key("lastIndex");
    ops::set(agent, r, key, Value::Number(v), true)
}

/// RegExpBuiltinExec(R, S) (§22.2.7.2)
pub(crate) fn regexp_builtin_exec(agent: &mut Agent, r: Object, s: JsString) -> JsResult<Value> {
    // 1. Let length be the length of S.
    let length = agent.str(s).len();
    // 2. Let lastIndex be ℝ(? ToLength(! Get(R, "lastIndex"))).
    let key = agent.key("lastIndex");
    let li = ops::get(agent, r, key)?;
    let mut last_index = ops::to_length(agent, li)?;
    // 3-6. The flags.
    let data = agent
        .native::<RegExpData>(r)
        .expect("RegExpBuiltinExec on a RegExp");
    let flags = agent.str_lossy(data.original_flags).to_string();
    let global = flags.contains('g');
    let sticky = flags.contains('y');
    let has_indices = flags.contains('d');
    // 7. If global is false and sticky is false, set lastIndex to 0.
    if !global && !sticky {
        last_index = 0.0;
    }
    // 9. If flags contains "u" or flags contains "v", let fullUnicode be true.
    let full_unicode = flags.contains('u') || flags.contains('v');
    // 10-13. Repeat, while matchSucceeded is false,
    let (captures, group_names) = loop {
        // a. If lastIndex > length, then
        if last_index > length as f64 {
            // i. If global is true or sticky is true, perform ? Set(R, "lastIndex", +0𝔽, true).
            if global || sticky {
                set_last_index(agent, r, 0.0)?;
            }
            // ii. Return null.
            return Ok(Value::Null);
        }
        // b. Let inputIndex be the index into input of the character that was obtained from
        //    element lastIndex of S (the start of a surrogate pair if lastIndex is inside one).
        let units = agent.str(s);
        let mut input_index = last_index as usize;
        if full_unicode
            && input_index > 0
            && input_index < units.len()
            && (0xDC00..=0xDFFF).contains(&units[input_index])
            && (0xD800..=0xDBFF).contains(&units[input_index - 1])
        {
            input_index -= 1;
        }
        // c. Let r be matcher(input, inputIndex).
        let data = agent
            .native::<RegExpData>(r)
            .expect("RegExpBuiltinExec on a RegExp");
        let Some(matcher) = &data.matcher else {
            return Err(agent.throw_type_error("RegExp is not initialized"));
        };
        match matcher.match_at(agent.str(s), input_index) {
            // e. Else, set matchSucceeded to true.
            Some(caps) => {
                let names: Vec<Option<String>> = (1..caps.len())
                    .map(|i| {
                        matcher
                            .group_names()
                            .iter()
                            .find(|(_, idx)| idx.contains(&i))
                            .map(|(n, _)| n.clone())
                    })
                    .collect();
                let has_names = !matcher.group_names().is_empty();
                break (caps, has_names.then_some(names));
            }
            // d. If r is failure, then
            None => {
                // i. If sticky is true, then set lastIndex to 0 and return null.
                if sticky {
                    set_last_index(agent, r, 0.0)?;
                    return Ok(Value::Null);
                }
                // ii. Set lastIndex to AdvanceStringIndex(S, lastIndex, fullUnicode).
                last_index = advance_string_index(agent.str(s), last_index, full_unicode);
            }
        }
    };
    // 14-15. Let e be r's endIndex (already a code unit index).
    let (_, end_index) = captures[0].expect("the whole match");
    // 16. If global is true or sticky is true, perform ? Set(R, "lastIndex", 𝔽(e), true).
    if global || sticky {
        set_last_index(agent, r, end_index as f64)?;
    }
    // 17-20. Let A be ! ArrayCreate(n + 1).
    let n = captures.len() - 1;
    let array = array_create(agent, (n + 1) as f64, None)?;
    // 22. Perform ! CreateDataPropertyOrThrow(A, "index", 𝔽(lastIndex)).
    let key = agent.key("index");
    ops::create_data_property_or_throw(agent, array, key, Value::Number(last_index))?;
    // 23. Perform ! CreateDataPropertyOrThrow(A, "input", S).
    let key = agent.key("input");
    ops::create_data_property_or_throw(agent, array, key, Value::String(s))?;
    // 24-29. The match: from lastIndex to e.
    let mut indices: Vec<Option<(usize, usize)>> = vec![Some((last_index as usize, end_index))];
    let matched = substring(agent, s, last_index as usize, end_index);
    let key = agent.key_from_index(0.0);
    ops::create_data_property_or_throw(agent, array, key, matched)?;
    // 30-31. If R contains any GroupName, let groups be OrdinaryObjectCreate(null).
    let groups = match group_names {
        Some(_) => Value::Object(agent.ordinary_object_create(None)),
        None => Value::Undefined,
    };
    // 32. Perform ! CreateDataPropertyOrThrow(A, "groups", groups).
    let key = agent.key("groups");
    ops::create_data_property_or_throw(agent, array, key, groups)?;
    // 33-34. For each capture i, in ascending order:
    let mut group_name_list: Vec<Option<String>> = Vec::with_capacity(n);
    let mut matched_group_names: Vec<String> = Vec::new();
    for (i, capture) in captures.iter().enumerate().skip(1) {
        // a-c. The captured value (undefined if the group did not participate) and its indices.
        let captured = match *capture {
            None => Value::Undefined,
            Some((cs, ce)) => substring(agent, s, cs, ce),
        };
        indices.push(*capture);
        // d. Perform ! CreateDataPropertyOrThrow(A, ! ToString(𝔽(i)), capturedValue).
        let key = agent.key_from_index(i as f64);
        ops::create_data_property_or_throw(agent, array, key, captured)?;
        // e. If the ith capture of R was defined with a GroupName, then
        match group_names.as_ref().and_then(|g| g[i - 1].clone()) {
            Some(name) => {
                // ii. If matchedGroupNames contains groupName, append undefined to groupNames.
                if matched_group_names.contains(&name) {
                    group_name_list.push(None);
                } else {
                    // iii.1. If capturedValue is not undefined, append groupName to matchedGroupNames.
                    if !captured.is_undefined() {
                        matched_group_names.push(name.clone());
                    }
                    // iii.3. Perform ! CreateDataPropertyOrThrow(groups, groupName, capturedValue).
                    if let Value::Object(g) = groups {
                        let key = agent.key(&name);
                        ops::create_data_property_or_throw(agent, g, key, captured)?;
                    }
                    // iii.4. Append groupName to groupNames.
                    group_name_list.push(Some(name));
                }
            }
            // f. Else, append undefined to groupNames.
            None => group_name_list.push(None),
        }
    }
    // 35. If hasIndices is true, then
    if has_indices {
        // a. Let indicesArray be MakeMatchIndicesIndexPairArray(S, indices, groupNames, hasGroups).
        let indices_array = make_match_indices_index_pair_array(
            agent,
            &indices,
            &group_name_list,
            !groups.is_undefined(),
        )?;
        // b. Perform ! CreateDataPropertyOrThrow(A, "indices", indicesArray).
        let key = agent.key("indices");
        ops::create_data_property_or_throw(agent, array, key, Value::Object(indices_array))?;
    }
    // 36. Return A.
    Ok(Value::Object(array))
}

/// GetMatchString(S, match) (§22.2.7.6)
fn substring(agent: &mut Agent, s: JsString, start: usize, end: usize) -> Value {
    let units = agent.str(s)[start..end].to_vec();
    Value::String(agent.intern(&units))
}

/// MakeMatchIndicesIndexPairArray(S, indices, groupNames, hasGroups) (§22.2.7.8)
fn make_match_indices_index_pair_array(
    agent: &mut Agent,
    indices: &[Option<(usize, usize)>],
    group_names: &[Option<String>],
    has_groups: bool,
) -> JsResult<Object> {
    // 1. Let n be the number of elements in indices.
    let n = indices.len();
    // 5. Let A be ! ArrayCreate(n).
    let array = array_create(agent, n as f64, None)?;
    // 6-7. If hasGroups is true, let groups be OrdinaryObjectCreate(null); else undefined.
    let groups = if has_groups {
        Value::Object(agent.ordinary_object_create(None))
    } else {
        Value::Undefined
    };
    // 8. Perform ! CreateDataPropertyOrThrow(A, "groups", groups).
    let key = agent.key("groups");
    ops::create_data_property_or_throw(agent, array, key, groups)?;
    // 9. For each integer i such that 0 ≤ i < n, in ascending order, do
    for (i, m) in indices.iter().enumerate() {
        // a-c. Let matchIndexPair be GetMatchIndexPair(S, matchIndices), or undefined.
        let pair = match *m {
            Some((s, e)) => Value::Object(ops::create_array_from_list(
                agent,
                &[Value::Number(s as f64), Value::Number(e as f64)],
            )),
            None => Value::Undefined,
        };
        // d. Perform ! CreateDataPropertyOrThrow(A, ! ToString(𝔽(i)), matchIndexPair).
        let key = agent.key_from_index(i as f64);
        ops::create_data_property_or_throw(agent, array, key, pair)?;
        // e. If i > 0 and groupNames[i - 1] is not undefined, set groups[name] to matchIndexPair.
        if i > 0
            && let Some(name) = &group_names[i - 1]
            && let Value::Object(g) = groups
        {
            let key = agent.key(name);
            ops::create_data_property_or_throw(agent, g, key, pair)?;
        }
    }
    // 10. Return A.
    Ok(array)
}

impl kahvi_core::gc::Trace for RegExpData {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        t.string(self.original_source);
        t.string(self.original_flags);
    }
}
