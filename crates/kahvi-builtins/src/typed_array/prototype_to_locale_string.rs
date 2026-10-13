//! %TypedArray%.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-%typedarray%.prototype.tolocalestring

use kahvi_core::prelude::*;

/// TypedArray.prototype.toLocaleString
pub(crate) fn prototype_to_locale_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This is a distinct method that implements the same algorithm as `Array.prototype.toLocaleString` as defined in [sec-array.prototype.tolocalestring] except that TypedArrayLength is called in place of performing a [[Get]] of *"length"*. The implementation of the algorithm may be optimized with the knowledge that the *this* value has a fixed length when the underlying buffer is not resizable and whose integer-indexed properties are not sparse. However, such optimization must not introduce any observable changes in the specified behaviour of the algorithm.
    // This method is not generic. ValidateTypedArray is called with the *this* value and ~seq-cst~ as arguments prior to evaluating the algorithm. If its result is an abrupt completion that exception is thrown instead of evaluating the algorithm.
    // NOTE:
    // If the ECMAScript implementation includes the ECMA-402 Internationalization API this method is based upon the algorithm for `Array.prototype.toLocaleString` that is in ECMA-402.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("TypedArray.prototype.toLocaleString"))
}
