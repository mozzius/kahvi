//! get ArrayBuffer.prototype.detached
//!
//! Spec: https://tc39.es/ecma262/#sec-get-arraybuffer.prototype.detached

use kahvi_core::object::typed_array::{array_buffer_data, is_detached_buffer};
use kahvi_core::prelude::*;

/// get ArrayBuffer.prototype.detached
pub(crate) fn prototype_get_detached(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // `ArrayBuffer.prototype.detached` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_obj_, [[ArrayBufferData]]).
    let _ = (args, new_target);
    let obj = match this {
        Value::Object(o) if array_buffer_data(agent, o).is_some() => o,
        _ => {
            return Err(agent.throw_type_error(
                "get ArrayBuffer.prototype.detached called on a non-ArrayBuffer",
            ));
        }
    };
    // 3. If IsSharedArrayBuffer(_obj_) is *true*, throw a *TypeError* exception.
    //    The engine has no Shared Data Blocks: a SharedArrayBuffer is not an
    //    `ObjectKind::ArrayBuffer`, so it already threw a TypeError in step 2.
    // 4. Return IsDetachedBuffer(_obj_).
    Ok(Value::Boolean(is_detached_buffer(agent, obj)))
}
