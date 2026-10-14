//! Object.prototype.__defineGetter__ ( _key_, _getter_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.__defineGetter__

use kahvi_core::prelude::*;

/// Object.prototype.__defineGetter__
pub(crate) fn prototype_define_getter(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    let key = args.get_or_undefined(0);
    let getter = args.get_or_undefined(1);
    // The key conversion may collect garbage: obj, key and getter stay rooted.
    agent.with_roots(&[Value::Object(obj), key, getter], |agent| {
        // 2. If IsCallable(_getter_) is *false*, throw a *TypeError* exception.
        if !ops::is_callable(agent, getter) {
            return Err(agent
                .throw_type_error("Object.prototype.__defineGetter__ getter is not a function"));
        }
        // 3. Let _propertyDesc_ be PropertyDescriptor { [[Getter]]: _getter_, [[Enumerable]]: *true*, [[Configurable]]: *true* }.
        let property_desc = PropertyDescriptor {
            get: Some(getter),
            enumerable: Some(true),
            configurable: Some(true),
            ..Default::default()
        };
        // 4. Let _propertyKey_ be ? ToPropertyKey(_key_).
        let property_key = ops::to_property_key(agent, key)?;
        // 5. Perform ? DefinePropertyOrThrow(_obj_, _propertyKey_, _propertyDesc_).
        ops::define_property_or_throw(agent, obj, property_key, property_desc)?;
        // 6. Return *undefined*.
        Ok(Value::Undefined)
    })
}
