//! get Intl.NumberFormat.prototype.format
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.numberformat.prototype.format

use kahvi_core::prelude::*;

use crate::support::intl_number_format::{NumberFormatSlots, unwrap_number_format};

/// The closure of step 4.a: ToIntlMathematicalValue then FormatNumeric.
fn bound_format(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    Err(agent
        .not_implemented("Intl.NumberFormat format: ToIntlMathematicalValue and FormatNumeric"))
}

/// get Intl.NumberFormat.prototype.format
pub(crate) fn prototype_get_format(
    agent: &mut Agent,
    this: Value,
    _args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // 1. Let _nf_ be the *this* value.
    // 2. If the implementation supports the normative optional constructor mode of [legacy-constructor], then
    //    a. Set _nf_ to ? UnwrapNumberFormat(_nf_).
    let nf = unwrap_number_format(agent, this)?;
    // 3. Perform ? RequireInternalSlot(_nf_, [[InitializedNumberFormat]]).
    let Some(slots) = agent.native::<NumberFormatSlots>(nf) else {
        return Err(agent.throw_type_error("not an Intl.NumberFormat"));
    };
    // 4. If _nf_.[[BoundFormat]] is *undefined*, then
    let bound = slots.bound_format;
    let bound = match bound {
        Some(b) => b,
        None => {
            // a. Let _closure_ be a new Abstract Closure with parameters (_value_) that captures _nf_ and performs the following steps when called:
            //    i. Let _x_ be ? ToIntlMathematicalValue(_value_).
            //    ii. Return FormatNumeric(_nf_, _x_).
            // b. Set _nf_.[[BoundFormat]] to CreateBuiltinFunction(_closure_, 1, *""*, « »).
            let name = agent.key("");
            let f = create_builtin_function(
                agent,
                bound_format,
                1,
                name,
                vec![Value::Object(nf)],
                None,
                None,
                None,
                false,
            );
            if let Some(slots) = agent.native_mut::<NumberFormatSlots>(nf) {
                slots.bound_format = Some(f);
            }
            f
        }
    };
    // 5. Return _nf_.[[BoundFormat]].
    Ok(Value::Object(bound))
}
