//! Object.prototype.__defineSetter__ ( _key_, _setter_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.__defineSetter__

use kahvi_core::prelude::*;

/// Object.prototype.__defineSetter__
pub(crate) fn prototype_define_setter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. If IsCallable(_setter_) is *false*, throw a *TypeError* exception.
    // 3. Let _propertyDesc_ be PropertyDescriptor { [[Setter]]: _setter_, [[Enumerable]]: *true*, [[Configurable]]: *true* }.
    // 4. Let _propertyKey_ be ? ToPropertyKey(_key_).
    // 5. Perform ? DefinePropertyOrThrow(_obj_, _propertyKey_, _propertyDesc_).
    // 6. Return *undefined*.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.prototype.__defineSetter__"))
}
