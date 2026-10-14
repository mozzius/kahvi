//! Float64Array ( ..._args_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-typedarray

use kahvi_core::object::typed_array::ElementType;
use kahvi_core::prelude::*;

use crate::support::typed_array::typed_array_constructor;

/// Float64Array
pub(crate) fn constructor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // The steps of _TypedArray_ ( ...args ) (§23.2.5.1), with this constructor's element type.
    typed_array_constructor(agent, args, new_target, ElementType::Float64)
}
