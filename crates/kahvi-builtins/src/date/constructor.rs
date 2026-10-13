//! Date ( ..._values_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-date

use kahvi_core::prelude::*;

/// Date
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, then
    //    a. Let _now_ be the time value (UTC) identifying the current time.
    //    b. Return ToDateString(_now_).
    // 2. Let _numberOfArgs_ be the number of elements in _values_.
    // 3. If _numberOfArgs_ = 0, then
    //    a. Let _dv_ be the time value (UTC) identifying the current time.
    // 4. Else if _numberOfArgs_ = 1, then
    //    a. Let _value_ be _values_[0].
    //    b. If _value_ is an Object and _value_ has a [[DateValue]] internal slot, then
    //       i. Let _tv_ be _value_.[[DateValue]].
    //    c. Else,
    //       i. Let _primitive_ be ? ToPrimitive(_value_).
    //       ii. If _primitive_ is a String, then
    //          1. Assert: The next step never returns an abrupt completion because _primitive_ is a String.
    //          2. Let _tv_ be the result of parsing _primitive_ as a date, in exactly the same manner as for the `parse` method ([sec-date.parse]).
    //       iii. Else,
    //          1. Let _tv_ be ? ToNumber(_primitive_).
    //    d. Let _dv_ be TimeClip(_tv_).
    // 5. Else,
    //    a. Assert: _numberOfArgs_ ≥ 2.
    //    b. Let _yearNumber_ be ? ToNumber(_values_[0]).
    //    c. Let _monthNumber_ be ? ToNumber(_values_[1]).
    //    d. If _numberOfArgs_ > 2, let _dayNumber_ be ? ToNumber(_values_[2]); else let _dayNumber_ be *1*_𝔽.
    //    e. If _numberOfArgs_ > 3, let _hourNumber_ be ? ToNumber(_values_[3]); else let _hourNumber_ be *+0*_𝔽.
    //    f. If _numberOfArgs_ > 4, let _minuteNumber_ be ? ToNumber(_values_[4]); else let _minuteNumber_ be *+0*_𝔽.
    //    g. If _numberOfArgs_ > 5, let _secondNumber_ be ? ToNumber(_values_[5]); else let _secondNumber_ be *+0*_𝔽.
    //    h. If _numberOfArgs_ > 6, let _millisecondNumber_ be ? ToNumber(_values_[6]); else let _millisecondNumber_ be *+0*_𝔽.
    //    i. Set _yearNumber_ to MakeFullYear(_yearNumber_).
    //    j. Let _finalDate_ be MakeDate(MakeDay(_yearNumber_, _monthNumber_, _dayNumber_), MakeTime(_hourNumber_, _minuteNumber_, _secondNumber_, _millisecondNumber_)).
    //    k. Let _dv_ be TimeClip(UTC(_finalDate_)).
    // 6. Let _obj_ be ? OrdinaryCreateFromConstructor(NewTarget, *"%Date.prototype%"*, « [[DateValue]] »).
    // 7. Set _obj_.[[DateValue]] to _dv_.
    // 8. Return _obj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date"))
}
