//! Array.of ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.of

use kahvi_core::object::array::array_create;
use kahvi_core::prelude::*;

/// Array.of
pub(crate) fn of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _length_ be the number of elements in _items_.
    // 2. Let _lengthNumber_ be 𝔽(_length_).
    // 3. Let _ctor_ be the *this* value.
    // 4. If IsConstructor(_ctor_) is *true*, then
    //    a. Let _array_ be ? Construct(_ctor_, « _lengthNumber_ »).
    // 5. Else,
    //    a. Let _array_ be ? ArrayCreate(_length_).
    // 6. Let _k_ be 0.
    // 7. Repeat, while _k_ < _length_,
    //    a. Let _kValue_ be _items_[_k_].
    //    b. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    c. Perform ? CreateDataPropertyOrThrow(_array_, _propertyKey_, _kValue_).
    //    d. Set _k_ to _k_ + 1.
    // 8. Perform ? Set(_array_, *"length"*, _lengthNumber_, *true*).
    // 9. Return _array_.
    //
    // NOTE:
    // This method is an intentionally generic factory method; it does not require that its *this* value be the Array constructor. Therefore it can be transferred to or inherited by other constructors that may be called with a single numeric argument.
    // 1. Let _length_ be the number of elements in _items_.
    let length = args.len();
    // 2. Let _lengthNumber_ be 𝔽(_length_).
    let length_number = length as f64;
    // 3. Let _ctor_ be the *this* value.
    let ctor = this;
    // 4. If IsConstructor(_ctor_) is *true*, then
    let array = match ctor {
        Value::Object(c) if ops::is_constructor(agent, ctor) => {
            // a. Let _array_ be ? Construct(_ctor_, « _lengthNumber_ »).
            ops::construct(agent, c, &[Value::Number(length_number)], None)?
        }
        // 5. Else,
        _ => {
            // a. Let _array_ be ? ArrayCreate(_length_).
            array_create(agent, length_number, None)?
        }
    };
    // The loop may collect garbage: _array_ stays rooted.
    agent.with_roots(&[Value::Object(array)], |agent| {
        // 6. Let _k_ be 0.
        let mut k = 0;
        // 7. Repeat, while _k_ < _length_,
        while k < length {
            // a. Let _kValue_ be _items_[_k_].
            let k_value = args.get_or_undefined(k);
            // b. Let _propertyKey_ be ! ToString(𝔽(_k_)).
            let property_key = agent.key_from_index(k as f64);
            // c. Perform ? CreateDataPropertyOrThrow(_array_, _propertyKey_, _kValue_).
            ops::create_data_property_or_throw(agent, array, property_key, k_value)?;
            // d. Set _k_ to _k_ + 1.
            k += 1;
        }
        // 8. Perform ? Set(_array_, *"length"*, _lengthNumber_, *true*).
        let length_key = agent.key("length");
        ops::set(agent, array, length_key, Value::Number(length_number), true)?;
        // 9. Return _array_.
        Ok(Value::Object(array))
    })
}
