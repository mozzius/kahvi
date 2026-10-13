//! Date.UTC ( _year_ [ , _month_ [ , _day_ [ , _hour_ [ , _minute_ [ , _second_ [ , _millisecond_ ] ] ] ] ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-date.utc

use kahvi_core::prelude::*;

/// Date.UTC
pub(crate) fn utc(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. Let _yearNumber_ be ? ToNumber(_year_).
    // 2. If _month_ is present, let _monthNumber_ be ? ToNumber(_month_); else let _monthNumber_ be *+0*_𝔽.
    // 3. If _day_ is present, let _dayNumber_ be ? ToNumber(_day_); else let _dayNumber_ be *1*_𝔽.
    // 4. If _hour_ is present, let _hourNumber_ be ? ToNumber(_hour_); else let _hourNumber_ be *+0*_𝔽.
    // 5. If _minute_ is present, let _minuteNumber_ be ? ToNumber(_minute_); else let _minuteNumber_ be *+0*_𝔽.
    // 6. If _second_ is present, let _secondNumber_ be ? ToNumber(_second_); else let _secondNumber_ be *+0*_𝔽.
    // 7. If _millisecond_ is present, let _millisecondNumber_ be ? ToNumber(_millisecond_); else let _millisecondNumber_ be *+0*_𝔽.
    // 8. Set _yearNumber_ to MakeFullYear(_yearNumber_).
    // 9. Return TimeClip(MakeDate(MakeDay(_yearNumber_, _monthNumber_, _dayNumber_), MakeTime(_hourNumber_, _minuteNumber_, _secondNumber_, _millisecondNumber_))).
    //
    // The *"length"* property of this function is *7*_𝔽.
    // NOTE:
    // This function differs from the Date constructor in two ways: it returns a time value as a Number, rather than creating a Date, and it interprets the arguments in UTC rather than as local time.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Date.UTC"))
}
