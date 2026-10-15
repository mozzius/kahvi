//! Legacy RegExp features (the Stage 3 proposal, scored by test262 as
//! `legacy-regexp`): the static properties of %RegExp% such as
//! `RegExp.$1` and `RegExp.lastMatch`.
//!
//! Spec: https://github.com/tc39/proposal-regexp-legacy-features
//! (vendor/proposals/regexp-legacy-features.md)
//!
//! The internal slots live on the realm (`Realm::legacy_regexp`), which owns
//! %RegExp%. They keep the matched string and indices; the getters cut the
//! substrings.

use kahvi_core::prelude::*;
use kahvi_core::realm::LegacyLastMatch;

use crate::table::{FnEntry, Key, Kind};

/// The internal slot a legacy static property reads.
#[derive(Clone, Copy)]
enum Slot {
    Input,
    LastMatch,
    LastParen,
    LeftContext,
    RightContext,
    Paren(usize),
}

/// UpdateLegacyRegExpStaticProperties ( C, S, startIndex, endIndex, capturedValues )
pub(crate) fn update_legacy_regexp_static_properties(
    agent: &mut Agent,
    realm: RealmId,
    s: JsString,
    start_index: usize,
    end_index: usize,
    captured_values: &[Value],
) {
    // 1-5. Assert: C has a [[RegExpInput]] internal slot, S is a String, 0 ≤ startIndex ≤
    //      endIndex ≤ len, and capturedValues is a List of Strings.
    // (A capture that did not participate is undefined here; it counts as the empty String.)
    let empty = agent.names.empty;
    let as_string = |v: &Value| match v {
        Value::String(s) => *s,
        _ => empty,
    };
    // 6. Let n be the number of elements in capturedValues.
    // 12. For each integer i such that 1 ≤ i ≤ 9: if i ≤ n, set C.[[RegExpParen<i>]] to the
    //     ith element of capturedValues; else set it to the empty String.
    let mut parens = [empty; 9];
    for (p, v) in parens.iter_mut().zip(captured_values) {
        *p = as_string(v);
    }
    // 9-10. If n > 0, set C.[[RegExpLastParen]] to the last element of capturedValues;
    //       else set it to the empty String.
    let last_paren = captured_values.last().map(as_string).unwrap_or(empty);
    // 7. Set C.[[RegExpInput]] to S.
    // 8. Set C.[[RegExpLastMatch]] to the substring of S from startIndex to endIndex.
    // 11. Set C.[[RegExpLeftContext]] to the substring of S from 0 to startIndex.
    // 12. Set C.[[RegExpRightContext]] to the substring of S from endIndex to len.
    let statics = &mut agent.realm_mut(realm).legacy_regexp;
    statics.input = Some(s);
    statics.last_match = Some(LegacyLastMatch {
        matched_in: s,
        start: start_index,
        end: end_index,
        last_paren,
        parens,
    });
}

/// InvalidateLegacyRegExpStaticProperties ( C )
pub(crate) fn invalidate_legacy_regexp_static_properties(agent: &mut Agent, realm: RealmId) {
    // 1. Assert: C is an Object that has a [[RegExpInput]] internal slot.
    // 2. Set the value of the following internal slots of C to empty: [[RegExpInput]] ...
    //    [[RegExpParen9]].
    let statics = &mut agent.realm_mut(realm).legacy_regexp;
    statics.input = None;
    statics.last_match = None;
}

/// GetLegacyRegExpStaticProperty ( C, thisValue, internalSlotName )
fn get_legacy_regexp_static_property(
    agent: &mut Agent,
    this_value: Value,
    slot: Slot,
) -> JsResult<Value> {
    // C is %RegExp% of the current realm.
    let realm = agent.current_realm();
    let c = agent.intrinsic_in(realm, "%RegExp%");
    // 1. Assert C is an object that has an internal slot named internalSlotName.
    // 2. If SameValue(C, thisValue) is false, throw a TypeError exception.
    if !matches!(this_value, Value::Object(o) if o == c) {
        return Err(
            agent.throw_type_error("RegExp legacy static accessor called on another object")
        );
    }
    // 3. Let val be the value of the internal slot of C named internalSlotName.
    // 4. If val is empty, throw a TypeError exception.
    let statics = agent.realm(realm).legacy_regexp.clone();
    let unavailable = |agent: &mut Agent| {
        agent.throw_type_error("RegExp legacy static property is not available")
    };
    if let Slot::Input = slot {
        return statics
            .input
            .map(Value::String)
            .ok_or_else(|| unavailable(agent));
    }
    let Some(l) = statics.last_match else {
        return Err(unavailable(agent));
    };
    let cut = |agent: &mut Agent, from: usize, to: usize| {
        let units = agent.str(l.matched_in)[from..to].to_vec();
        agent.intern(&units)
    };
    let val = match slot {
        Slot::Input => unreachable!("handled above"),
        Slot::LastMatch => cut(agent, l.start, l.end),
        Slot::LastParen => l.last_paren,
        Slot::LeftContext => cut(agent, 0, l.start),
        Slot::RightContext => {
            let len = agent.str(l.matched_in).len();
            cut(agent, l.end, len)
        }
        Slot::Paren(i) => l.parens[i - 1],
    };
    // 5. Return val.
    Ok(Value::String(val))
}

/// SetLegacyRegExpStaticProperty ( C, thisValue, internalSlotName, val ), for
/// [[RegExpInput]] (the only slot with a setter).
fn set_legacy_regexp_input(agent: &mut Agent, this_value: Value, val: Value) -> JsResult<()> {
    let realm = agent.current_realm();
    let c = agent.intrinsic_in(realm, "%RegExp%");
    // 1. Assert C is an object that has an internal slot named internalSlotName.
    // 2. If SameValue(C, thisValue) is false, throw a TypeError exception.
    if !matches!(this_value, Value::Object(o) if o == c) {
        return Err(
            agent.throw_type_error("RegExp legacy static accessor called on another object")
        );
    }
    // 3. Let strVal be ? ToString(val).
    let str_val = ops::to_string(agent, val)?;
    // 4. Set the value of the internal slot of C named internalSlotName to strVal.
    agent.realm_mut(realm).legacy_regexp.input = Some(str_val);
    Ok(())
}

macro_rules! getter {
    ($name:ident, $slot:expr) => {
        fn $name(
            agent: &mut Agent,
            this: Value,
            _: &[Value],
            _: Option<Object>,
        ) -> JsResult<Value> {
            get_legacy_regexp_static_property(agent, this, $slot)
        }
    };
}

// get RegExp.input / get RegExp.$_: Return ? GetLegacyRegExpStaticProperty(%RegExp%, this value, [[RegExpInput]]).
getter!(get_input, Slot::Input);
// get RegExp.lastMatch / get RegExp.$&
getter!(get_last_match, Slot::LastMatch);
// get RegExp.lastParen / get RegExp.$+
getter!(get_last_paren, Slot::LastParen);
// get RegExp.leftContext / get RegExp.$`
getter!(get_left_context, Slot::LeftContext);
// get RegExp.rightContext / get RegExp.$'
getter!(get_right_context, Slot::RightContext);
// get RegExp.$1 ... get RegExp.$9
getter!(get_paren1, Slot::Paren(1));
getter!(get_paren2, Slot::Paren(2));
getter!(get_paren3, Slot::Paren(3));
getter!(get_paren4, Slot::Paren(4));
getter!(get_paren5, Slot::Paren(5));
getter!(get_paren6, Slot::Paren(6));
getter!(get_paren7, Slot::Paren(7));
getter!(get_paren8, Slot::Paren(8));
getter!(get_paren9, Slot::Paren(9));

/// set RegExp.input = val / set RegExp.$_ = val
fn set_input(agent: &mut Agent, this: Value, args: &[Value], _: Option<Object>) -> JsResult<Value> {
    // 1. Perform ? SetLegacyRegExpStaticProperty(%RegExp%, this value, [[RegExpInput]], val).
    set_legacy_regexp_input(agent, this, args.get_or_undefined(0))?;
    Ok(Value::Undefined)
}

macro_rules! accessor {
    ($key:expr, $kind:ident, $length:expr, $f:ident) => {
        FnEntry {
            owner: "%RegExp%",
            key: Key::Str($key),
            kind: Kind::$kind,
            length: $length,
            writable: false,
            // { [[Enumerable]]: false, [[Configurable]]: true }
            enumerable: false,
            configurable: true,
            behaviour: $f,
        }
    };
}

/// The legacy static accessor properties of %RegExp%.
pub(crate) static FUNCTIONS: &[FnEntry] = &[
    accessor!("input", Getter, 0, get_input),
    accessor!("input", Setter, 1, set_input),
    accessor!("$_", Getter, 0, get_input),
    accessor!("$_", Setter, 1, set_input),
    accessor!("lastMatch", Getter, 0, get_last_match),
    accessor!("$&", Getter, 0, get_last_match),
    accessor!("lastParen", Getter, 0, get_last_paren),
    accessor!("$+", Getter, 0, get_last_paren),
    accessor!("leftContext", Getter, 0, get_left_context),
    accessor!("$`", Getter, 0, get_left_context),
    accessor!("rightContext", Getter, 0, get_right_context),
    accessor!("$'", Getter, 0, get_right_context),
    accessor!("$1", Getter, 0, get_paren1),
    accessor!("$2", Getter, 0, get_paren2),
    accessor!("$3", Getter, 0, get_paren3),
    accessor!("$4", Getter, 0, get_paren4),
    accessor!("$5", Getter, 0, get_paren5),
    accessor!("$6", Getter, 0, get_paren6),
    accessor!("$7", Getter, 0, get_paren7),
    accessor!("$8", Getter, 0, get_paren8),
    accessor!("$9", Getter, 0, get_paren9),
];
