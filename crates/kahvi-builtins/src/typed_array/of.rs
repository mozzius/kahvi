//! %TypedArray%.of ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.of

use kahvi_core::prelude::*;

/// TypedArray.of
pub(crate) fn of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _length_ be the number of elements in _items_.
    // 2. Let _ctor_ be the *this* value.
    // 3. If IsConstructor(_ctor_) is *false*, throw a *TypeError* exception.
    // 4. Let _newObj_ be ? TypedArrayCreateFromConstructor(_ctor_, « 𝔽(_length_) »).
    // 5. Let _k_ be 0.
    // 6. Repeat, while _k_ < _length_,
    //    a. Let _kValue_ be _items_[_k_].
    //    b. Perform ? TypedArraySetElement(_newObj_, 𝔽(_k_), _kValue_).
    //    c. Set _k_ to _k_ + 1.
    // 7. Return _newObj_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.of"))
}
