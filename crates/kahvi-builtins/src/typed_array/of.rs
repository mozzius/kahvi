//! %TypedArray%.of ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.of

use kahvi_core::prelude::*;

use crate::support::typed_array::{typed_array_create_from_constructor, typed_array_set_element};

/// TypedArray.of
pub(crate) fn of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _length_ be the number of elements in _items_.
    let length = args.len();
    // 2. Let _ctor_ be the *this* value.
    let ctor = this;
    // 3. If IsConstructor(_ctor_) is *false*, throw a *TypeError* exception.
    if !ops::is_constructor(agent, ctor) {
        return Err(agent.throw_type_error("TypedArray.of: this is not a constructor"));
    }
    let Value::Object(ctor) = ctor else {
        return Err(agent.throw_type_error("TypedArray.of: this is not a constructor"));
    };
    // 4. Let _newObj_ be ? TypedArrayCreateFromConstructor(_ctor_, « 𝔽(_length_) »).
    let new_obj =
        typed_array_create_from_constructor(agent, ctor, &[Value::Number(length as f64)])?;
    // The element conversions may collect garbage: _newObj_ and _items_ stay rooted.
    let mut roots = vec![Value::Object(new_obj)];
    roots.extend_from_slice(args);
    agent.with_roots(&roots, |agent| {
        // 5. Let _k_ be 0.
        let mut k = 0.0;
        let len = length as f64;
        // 6. Repeat, while _k_ < _length_,
        while k < len {
            // a. Let _kValue_ be _items_[_k_].
            let k_value = args.get_or_undefined(k as usize);
            // b. Perform ? TypedArraySetElement(_newObj_, 𝔽(_k_), _kValue_).
            typed_array_set_element(agent, new_obj, k, k_value)?;
            // c. Set _k_ to _k_ + 1.
            k += 1.0;
        }
        // 7. Return _newObj_.
        Ok(Value::Object(new_obj))
    })
}
