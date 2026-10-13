//! Iterator.prototype.toArray ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.toarray

use kahvi_core::prelude::*;

/// Iterator.prototype.toArray
pub(crate) fn prototype_to_array(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    // 3. Let _iterated_ be ? GetIteratorDirect(_obj_).
    // 4. Let _items_ be a new empty List.
    // 5. Repeat,
    //    a. Let _value_ be ? IteratorStepValue(_iterated_).
    //    b. If _value_ is ~done~, return CreateArrayFromList(_items_).
    //    c. Append _value_ to _items_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Iterator.prototype.toArray"))
}
