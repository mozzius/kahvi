//! Array.prototype.flat ( [ _depth_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.flat

use kahvi_core::prelude::*;

/// Array.prototype.flat
pub(crate) fn prototype_flat(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _sourceLength_ be ? LengthOfArrayLike(_obj_).
    // 3. Let _depthNumber_ be 1.
    // 4. If _depth_ is not *undefined*, then
    //    a. Set _depthNumber_ to ? ToIntegerOrInfinity(_depth_).
    //    b. If _depthNumber_ < 0, set _depthNumber_ to 0.
    // 5. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
    // 6. Perform ? FlattenIntoArray(_array_, _obj_, _sourceLength_, 0, _depthNumber_).
    // 7. Return _array_.
    //
    // [subclause sec-flattenintoarray: FlattenIntoArray (
    // _target_: an Object,
    // _source_: an Object,
    // _sourceLength_: a non-negative integer,
    // _start_: a non-negative integer,
    // _depth_: a non-negative integer or +∞,
    // optional _mapperFunc_: a function object,
    // optional _thisArg_: an ECMAScript language value,
    // ): either a normal completion containing a non-negative integer or a throw completion]

    // 1. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    let depth = args.get_or_undefined(0);
    // The depth conversion may run JavaScript, so obj and depth stay rooted.
    agent.with_roots(&[Value::Object(obj), depth], |agent| {
        // 2. Let _sourceLength_ be ? LengthOfArrayLike(_obj_).
        let source_length = ops::length_of_array_like(agent, obj)?;
        // 3. Let _depthNumber_ be 1.
        let mut depth_number = 1.0;
        // 4. If _depth_ is not *undefined*, then
        if !matches!(depth, Value::Undefined) {
            // a. Set _depthNumber_ to ? ToIntegerOrInfinity(_depth_).
            depth_number = ops::to_integer_or_infinity(agent, depth)?;
            // b. If _depthNumber_ < 0, set _depthNumber_ to 0.
            if depth_number < 0.0 {
                depth_number = 0.0;
            }
        }
        // 5. Let _array_ be ? ArraySpeciesCreate(_obj_, 0).
        let array = ops::array_species_create(agent, obj, 0.0)?;
        // The flattening calls JavaScript, so array stays rooted.
        let mark = agent.push_root(Value::Object(array));
        // 6. Perform ? FlattenIntoArray(_array_, _obj_, _sourceLength_, 0, _depthNumber_).
        flatten_into_array(agent, array, obj, source_length, 0.0, depth_number)?;
        agent.pop_roots(mark);
        // 7. Return _array_.
        Ok(Value::Object(array))
    })
}

/// FlattenIntoArray ( _target_, _source_, _sourceLength_, _start_, _depth_ )
///
/// The optional _mapperFunc_ and _thisArg_ parameters are omitted: only
/// Array.prototype.flat calls this, and it never passes them.
///
/// The caller keeps _target_ and _source_ rooted.
fn flatten_into_array(
    agent: &mut Agent,
    target: Object,
    source: Object,
    source_length: f64,
    start: f64,
    depth: f64,
) -> JsResult<f64> {
    // 1. Assert: If _mapperFunc_ is present, then IsCallable(_mapperFunc_) is *true*,
    //    _thisArg_ is present, and _depth_ is 1.
    //    (Not applicable: no mapper is passed.)
    // 2. Let _targetIndex_ be _start_.
    let mut target_index = start;
    // 3. Let _sourceIndex_ be 0.
    let mut source_index = 0.0;
    // 4. Repeat, while _sourceIndex_ < _sourceLength_,
    while source_index < source_length {
        // a. Let _propertyKey_ be ! ToString(𝔽(_sourceIndex_)).
        let property_key = agent.key_from_index(source_index);
        // b. Let _exists_ be ? HasProperty(_source_, _propertyKey_).
        if ops::has_property(agent, source, property_key)? {
            // i. Let _element_ be ? Get(_source_, _propertyKey_).
            let element = ops::get(agent, source, property_key)?;
            // The element and key stay rooted while JavaScript runs below.
            let mark = agent.push_root(element);
            agent.push_root_key(property_key);
            // iv. If _depth_ > 0, then
            //     1. Set _shouldFlatten_ to ? IsArray(_element_).
            // (_shouldFlatten_ is false when _depth_ is not greater than 0.)
            let should_flatten = depth > 0.0 && ops::is_array(agent, element)?;
            // v. If _shouldFlatten_ is *true*, then
            if should_flatten {
                let Value::Object(element_obj) = element else {
                    unreachable!("IsArray(element) is true only for an object");
                };
                // 1. If _depth_ = +∞, let _newDepth_ be +∞.
                // 2. Else, let _newDepth_ be _depth_ - 1.
                let new_depth = if depth == f64::INFINITY {
                    f64::INFINITY
                } else {
                    depth - 1.0
                };
                // 3. Let _elementLength_ be ? LengthOfArrayLike(_element_).
                let element_length = ops::length_of_array_like(agent, element_obj)?;
                // 4. Set _targetIndex_ to ? FlattenIntoArray(_target_, _element_, _elementLength_, _targetIndex_, _newDepth_).
                target_index = flatten_into_array(
                    agent,
                    target,
                    element_obj,
                    element_length,
                    target_index,
                    new_depth,
                )?;
            } else {
                // vi. Else,
                //     1. If _targetIndex_ ≥ 2^53 - 1, throw a *TypeError* exception.
                if target_index >= 9_007_199_254_740_991.0 {
                    return Err(agent
                        .throw_type_error("Array.prototype.flat result length exceeds 2^53 - 1"));
                }
                // 2. Perform ? CreateDataPropertyOrThrow(_target_, ! ToString(𝔽(_targetIndex_)), _element_).
                let target_key = agent.key_from_index(target_index);
                ops::create_data_property_or_throw(agent, target, target_key, element)?;
                // 3. Set _targetIndex_ to _targetIndex_ + 1.
                target_index += 1.0;
            }
            agent.pop_roots(mark);
        }
        // d. Set _sourceIndex_ to _sourceIndex_ + 1.
        source_index += 1.0;
    }
    // 5. Return _targetIndex_.
    Ok(target_index)
}
