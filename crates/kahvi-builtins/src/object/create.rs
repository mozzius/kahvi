//! Object.create ( _proto_, _properties_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.create

use kahvi_core::prelude::*;

/// Object.create
pub(crate) fn create(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This function creates a new object with a specified prototype.
    // It performs the following steps when called:
    //
    // 1. If _proto_ is not an Object and _proto_ is not *null*, throw a *TypeError* exception.
    let proto = match args.get_or_undefined(0) {
        Value::Object(p) => Some(p.clone()),
        Value::Null => None,
        _ => {
            return Err(agent.throw_type_error("Object prototype may only be an Object or null"));
        }
    };
    // 2. Let _obj_ be OrdinaryObjectCreate(_proto_).
    let obj = agent.ordinary_object_create(proto);
    // 3. If _properties_ is not *undefined*, then
    let properties = args.get_or_undefined(1);
    if !matches!(properties, Value::Undefined) {
        // a. Return ? ObjectDefineProperties(_obj_, _properties_).
        return object_define_properties(agent, obj, properties.clone());
    }
    // 4. Return _obj_.
    Ok(Value::Object(obj))
}

/// ObjectDefineProperties ( _obj_, _properties_ )
///
/// [subclause sec-objectdefineproperties: ObjectDefineProperties (
/// _obj_: an Object,
/// _properties_: an ECMAScript language value,
/// ): either a normal completion containing an Object or a throw completion]
fn object_define_properties(agent: &mut Agent, obj: Object, properties: Value) -> JsResult<Value> {
    // 1. Set _properties_ to ? ToObject(_properties_).
    let properties = ops::to_object(agent, properties)?;
    // 2. Let _keys_ be ? _properties_.[[OwnPropertyKeys]]().
    let keys = properties.internal_own_property_keys(agent)?;
    // 3. Let _propertyDescs_ be a new empty List.
    let mut property_descs = Vec::new();
    // 4. For each element _nextKey_ of _keys_, do
    for next_key in keys {
        // a. Let _currentPropertyDesc_ be ? _properties_.[[GetOwnProperty]](_nextKey_).
        let current_property_desc = properties.internal_get_own_property(agent, next_key)?;
        // b. If _currentPropertyDesc_ is not *undefined* and _currentPropertyDesc_.[[Enumerable]] is *true*, then
        let enumerable = current_property_desc.is_some_and(|d| d.enumerable == Some(true));
        if enumerable {
            // i. Let _propertyDescObj_ be ? Get(_properties_, _nextKey_).
            let property_desc_obj = ops::get(agent, properties, next_key)?;
            // ii. Let _propertyDesc_ be ? ToPropertyDescriptor(_propertyDescObj_).
            let property_desc = ops::to_property_descriptor(agent, property_desc_obj)?;
            // iii. Append the Record { [[Key]]: _nextKey_, [[Descriptor]]: _propertyDesc_ } to _propertyDescs_.
            property_descs.push((next_key, property_desc));
        }
    }
    // 5. For each element _property_ of _propertyDescs_, do
    for (key, desc) in property_descs {
        // a. Perform ? DefinePropertyOrThrow(_obj_, _property_.[[Key]], _property_.[[Descriptor]]).
        ops::define_property_or_throw(agent, obj, key, desc)?;
    }
    // 6. Return _obj_.
    Ok(Value::Object(obj))
}
