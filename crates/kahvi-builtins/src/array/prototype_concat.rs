//! Array.prototype.concat ( ..._items_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.concat

use kahvi_core::prelude::*;

/// Array.prototype.concat
pub(crate) fn prototype_concat(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method returns an array containing the array elements of the object followed by the array elements of each argument.
    // It performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
    // 3. Let _nextIndex_ be 0.
    // 4. Prepend _obj_ to _items_.
    // 5. For each element _item_ of _items_, do
    //    a. Let _spreadable_ be ? IsConcatSpreadable(_item_).
    //    b. If _spreadable_ is *true*, then
    //       i. Let _length_ be ? LengthOfArrayLike(_item_).
    //       ii. If _nextIndex_ + _length_ > 2^53 - 1, throw a *TypeError* exception.
    //       iii. Let _sourceIndex_ be 0.
    //       iv. Repeat, while _sourceIndex_ < _length_,
    //          1. Let _propertyKey_ be ! ToString(𝔽(_sourceIndex_)).
    //          2. Let _exists_ be ? HasProperty(_item_, _propertyKey_).
    //          3. If _exists_ is *true*, then
    //             a. Let _subElement_ be ? Get(_item_, _propertyKey_).
    //             b. Perform ? CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_nextIndex_)), _subElement_).
    //          4. Set _nextIndex_ to _nextIndex_ + 1.
    //          5. Set _sourceIndex_ to _sourceIndex_ + 1.
    //    c. Else,
    //       i. NOTE: _item_ is added as a single item rather than spread.
    //       ii. If _nextIndex_ ≥ 2^53 - 1, throw a *TypeError* exception.
    //       iii. Perform ? CreateDataPropertyOrThrow(_array_, ! ToString(𝔽(_nextIndex_)), _item_).
    //       iv. Set _nextIndex_ to _nextIndex_ + 1.
    // 6. Perform ? Set(_array_, *"length"*, 𝔽(_nextIndex_), *true*).
    // 7. Return _array_.
    //
    // The *"length"* property of this method is *1*_𝔽.
    // NOTE:
    // The explicit setting of the *"length"* property in step [step-array-proto-concat-set-length] is intended to ensure the length is correct when the final non-empty element of _items_ has trailing holes or when _array_ is not a built-in Array.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    // [subclause sec-isconcatspreadable: IsConcatSpreadable (
    // _obj_: an ECMAScript language value,
    // ): either a normal completion containing a Boolean or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Array.prototype.concat"))
}
