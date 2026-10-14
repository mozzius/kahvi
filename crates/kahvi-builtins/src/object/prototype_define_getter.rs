//! Object.prototype.__defineGetter__ ( _key_, _getter_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.__defineGetter__

use kahvi_core::prelude::*;

/// Object.prototype.__defineGetter__
pub(crate) fn prototype_define_getter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. If IsCallable(_getter_) is *false*, throw a *TypeError* exception.
    // 3. Let _propertyDesc_ be PropertyDescriptor { [[Getter]]: _getter_, [[Enumerable]]: *true*, [[Configurable]]: *true* }.
    // 4. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 5. Perform ? DefinePropertyOrThrow(_obj_, _propertyKey_, _propertyDesc_).
    // 6. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.prototype.__defineGetter__"))
}
