//! Float16Array ( ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-typedarray

use kahvi_core::prelude::*;

/// Float16Array
pub(crate) fn constructor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // Each _TypedArray_ constructor performs the following steps when called:
    //
    // 1. If NewTarget is *undefined*, throw a *TypeError* exception.
    // 2. Let _ctorName_ be the String value of the Constructor Name value specified in [table-the-typedarray-constructors] for this TypedArray constructor.
    // 3. Let _proto_ be "%TypedArray.prototype%".
    // 4. Let _numberOfArgs_ be the number of elements in _args_.
    // 5. If _numberOfArgs_ = 0, return ? AllocateTypedArray(_ctorName_, NewTarget, _proto_, 0).
    // 6. Let _firstArg_ be _args_[0].
    // 7. If _firstArg_ is an Object, then
    //    a. Let _obj_ be ? AllocateTypedArray(_ctorName_, NewTarget, _proto_).
    //    b. If _firstArg_ has a [[TypedArrayName]] internal slot, then
    //       i. Perform ? InitializeTypedArrayFromTypedArray(_obj_, _firstArg_).
    //    c. Else if _firstArg_ has an [[ArrayBufferData]] internal slot, then
    //       i. If _numberOfArgs_ > 1, let _byteOffset_ be _args_[1]; else let _byteOffset_ be *undefined*.
    //       ii. If _numberOfArgs_ > 2, let _length_ be _args_[2]; else let _length_ be *undefined*.
    //       iii. Perform ? InitializeTypedArrayFromArrayBuffer(_obj_, _firstArg_, _byteOffset_, _length_).
    //    d. Else,
    //       i. Assert: _firstArg_ is an Object and _firstArg_ does not have either a [[TypedArrayName]] or an [[ArrayBufferData]] internal slot.
    //       ii. Let _usingIterator_ be ? GetMethod(_firstArg_, %Symbol.iterator%).
    //       iii. If _usingIterator_ is not *undefined*, then
    //          1. Let _values_ be ? IteratorToList(? GetIteratorFromMethod(_firstArg_, _usingIterator_)).
    //          2. Perform ? InitializeTypedArrayFromList(_obj_, _values_).
    //       iv. Else,
    //          1. NOTE: _firstArg_ is not an iterable object, so assume it is already an array-like object.
    //          2. Perform ? InitializeTypedArrayFromArrayLike(_obj_, _firstArg_).
    //    e. Return _obj_.
    // 8. Assert: _firstArg_ is not an Object.
    // 9. Let _elementLength_ be ? ToIndex(_firstArg_).
    // 10. Return ? AllocateTypedArray(_ctorName_, NewTarget, _proto_, _elementLength_).
    //
    // [subclause sec-allocatetypedarray: AllocateTypedArray (
    // _ctorName_: a String which is the name of a TypedArray constructor in [table-the-typedarray-constructors],
    // _newTarget_: a constructor,
    // _defaultProto_: a String,
    // optional _length_: a non-negative integer,
    // ): either a normal completion containing a TypedArray or a throw completion]
    // [subclause sec-initializetypedarrayfromtypedarray: InitializeTypedArrayFromTypedArray (
    // _obj_: a TypedArray,
    // _sourceArray_: a TypedArray,
    // ): either a normal completion containing ~unused~ or a throw completion]
    // [subclause sec-initializetypedarrayfromarraybuffer: InitializeTypedArrayFromArrayBuffer (
    // _obj_: a TypedArray,
    // _buffer_: an ArrayBuffer or a SharedArrayBuffer,
    // _byteOffset_: an ECMAScript language value,
    // _length_: an ECMAScript language value,
    // ): either a normal completion containing ~unused~ or a throw completion]
    // [subclause sec-initializetypedarrayfromlist: InitializeTypedArrayFromList (
    // _obj_: a TypedArray,
    // _values_: a List of ECMAScript language values,
    // ): either a normal completion containing ~unused~ or a throw completion]
    // [subclause sec-initializetypedarrayfromarraylike: InitializeTypedArrayFromArrayLike (
    // _obj_: a TypedArray,
    // _arrayLike_: an Object, but not a TypedArray or an ArrayBuffer,
    // ): either a normal completion containing ~unused~ or a throw completion]
    // [subclause sec-allocatetypedarraybuffer: AllocateTypedArrayBuffer (
    // _obj_: a TypedArray,
    // _length_: a non-negative integer,
    // ): either a normal completion containing ~unused~ or a throw completion]
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Float16Array"))
}
