//! get Intl.Collator.prototype.compare
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.collator.prototype.compare

use kahvi_core::function::{builtin_slots, create_builtin_function};
use kahvi_core::prelude::*;

use crate::support::intl_collator::{CollatorSlots, compare_strings};

/// get Intl.Collator.prototype.compare
pub(crate) fn prototype_get_compare(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (args, new_target);
    // This named accessor property returns a function that compares two strings according to the sort order of this Collator object.
    // Intl.Collator.prototype.compare is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps:
    //
    // 1. Let _collator_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_collator_, [[InitializedCollator]]).
    let collator = match this {
        Value::Object(o) if agent.native::<CollatorSlots>(o).is_some() => o,
        _ => return Err(agent.throw_type_error("this is not an Intl.Collator")),
    };
    let bound = agent
        .native::<CollatorSlots>(collator)
        .and_then(|s| s.bound_compare);
    // 3. If _collator_.[[BoundCompare]] is *undefined*, then
    let bound = match bound {
        Some(b) => b,
        None => {
            // a. Let _closure_ be a new Abstract Closure with parameters (_x_, _y_) that captures _collator_ and performs the following steps when called:
            // b. Set _collator_.[[BoundCompare]] to CreateBuiltinFunction(_closure_, 2, *""*, « »).
            let empty = PropertyKey::String(agent.names.empty);
            let f = create_builtin_function(
                agent,
                compare_closure,
                2,
                empty,
                vec![Value::Object(collator)],
                None,
                None,
                None,
                false,
            );
            if let Some(slots) = agent.native_mut::<CollatorSlots>(collator) {
                slots.bound_compare = Some(f);
            }
            f
        }
    };
    // 4. Return _collator_.[[BoundCompare]].
    Ok(Value::Object(bound))
}

/// The closure of step 3.a.
fn compare_closure(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let f = agent.active_function().expect("a running built-in");
    let Value::Object(collator) = builtin_slots(agent, f)[0] else {
        unreachable!("a compare function without its collator");
    };
    // i. Let _xString_ be ? ToString(_x_).
    let x_string = ops::to_string(agent, args.get_or_undefined(0))?;
    // ii. Let _yString_ be ? ToString(_y_).
    let y_string = ops::to_string(agent, args.get_or_undefined(1))?;
    // iii. Return CompareStrings(_collator_, _xString_, _yString_).
    let x = agent.str(x_string).to_vec();
    let y = agent.str(y_string).to_vec();
    let Some(slots) = agent.native::<CollatorSlots>(collator) else {
        unreachable!("a compare function for a non-collator");
    };
    Ok(Value::Number(compare_strings(slots, &x, &y)))
}
