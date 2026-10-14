//! Object.prototype.__defineSetter__ ( _key_, _setter_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.__defineSetter__

use kahvi_core::prelude::*;

/// Object.prototype.__defineSetter__
pub(crate) fn prototype_define_setter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 2. If IsCallable(_setter_) is *false*, throw a *TypeError* exception.
    let setter = args.get_or_undefined(1);
    if !ops::is_callable(agent, setter) {
        return Err(
            agent.throw_type_error("Object.prototype.__defineSetter__ setter is not a function")
        );
    }
    // 3. Let _propertyDesc_ be PropertyDescriptor { [[Setter]]: _setter_, [[Enumerable]]: *true*, [[Configurable]]: *true* }.
    let property_desc = PropertyDescriptor {
        set: Some(setter),
        enumerable: Some(true),
        configurable: Some(true),
        ..Default::default()
    };
    // 4. Let _propertyKey_ be ? ToPropertyKey(_key_).
    let property_key = ops::to_property_key(agent, args.get_or_undefined(0))?;
    // 5. Perform ? DefinePropertyOrThrow(_obj_, _propertyKey_, _propertyDesc_).
    ops::define_property_or_throw(agent, obj, property_key, property_desc)?;
    // 6. Return *undefined*.
    Ok(Value::Undefined)
}
