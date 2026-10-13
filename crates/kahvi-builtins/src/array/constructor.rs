//! Array ( ..._values_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array

use kahvi_core::prelude::*;

/// Array
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, let _newTarget_ be the active function object; else let _newTarget_ be NewTarget.
    // 2. Let _proto_ be ? GetPrototypeFromConstructor(_newTarget_, *"%Array.prototype%"*).
    // 3. Let _numberOfArgs_ be the number of elements in _values_.
    // 4. If _numberOfArgs_ = 0, return ! ArrayCreate(0, _proto_).
    // 5. If _numberOfArgs_ = 1, then
    //    a. Let _length_ be _values_[0].
    //    b. Let _array_ be ! ArrayCreate(0, _proto_).
    //    c. If _length_ is not a Number, then
    //       i. Perform ! CreateDataPropertyOrThrow(_array_, *"0"*, _length_).
    //       ii. Let _intLength_ be *1*_𝔽.
    //    d. Else,
    //       i. Let _intLength_ be ! ToUint32(_length_).
    //       ii. If SameValueZero(_intLength_, _length_) is *false*, throw a *RangeError* exception.
    //    e. Perform ! Set(_array_, *"length"*, _intLength_, *true*).
    //    f. Return _array_.
    // 6. Assert: _numberOfArgs_ ≥ 2.
    // 7. Let _array_ be ? ArrayCreate(_numberOfArgs_, _proto_).
    // 8. Let _k_ be 0.
    // 9. Repeat, while _k_ < _numberOfArgs_,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _itemK_ be _values_[_k_].
    //    c. Perform ! CreateDataPropertyOrThrow(_array_, _propertyKey_, _itemK_).
    //    d. Set _k_ to _k_ + 1.
    // 10. Assert: The mathematical value of _array_'s *"length"* property is _numberOfArgs_.
    // 11. Return _array_.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array"))
}
