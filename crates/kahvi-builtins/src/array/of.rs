//! Array.of ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.of

use kahvi_core::prelude::*;

/// Array.of
pub(crate) fn of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
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
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.of"))
}
