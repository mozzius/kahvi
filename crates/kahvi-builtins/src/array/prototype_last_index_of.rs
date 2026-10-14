//! Array.prototype.lastIndexOf ( _searchElement_ [ , _fromIndex_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.lastindexof

use kahvi_core::prelude::*;

/// Array.prototype.lastIndexOf
pub(crate) fn prototype_last_index_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // NOTE:
    // This method compares _searchElement_ to the elements of the array in descending order using the IsStrictlyEqual algorithm, and if found at one or more indices, returns the largest such index; otherwise, it returns *-1*_𝔽.
    // The optional second argument _fromIndex_ defaults to the array's length minus one (i.e. the whole array is searched). If it is greater than or equal to the length of the array, the whole array will be searched. If it is less than *-0*_𝔽, it is used to compute an offset from the end of the array. If the computed index is less than *-0*_𝔽, *-1*_𝔽 is returned.
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be ? ToObject(*this* value).
    // 2. Let _length_ be ? LengthOfArrayLike(_obj_).
    // 3. If _length_ = 0, return *-1*_𝔽.
    // 4. If _fromIndex_ is not present, let _k_ be _length_ - 1; else let _k_ be min(? ToAbsoluteIndex(_fromIndex_, _length_), _length_ - 1).
    // 5. Repeat, while _k_ ≥ 0,
    //    a. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    //    b. Let _kPresent_ be ? HasProperty(_obj_, _propertyKey_).
    //    c. If _kPresent_ is *true*, then
    //       i. Let _elementK_ be ? Get(_obj_, _propertyKey_).
    //       ii. If IsStrictlyEqual(_searchElement_, _elementK_) is *true*, return 𝔽(_k_).
    //    d. Set _k_ to _k_ - 1.
    // 6. Return *-1*_𝔽.
    //
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be an Array. Therefore it can be transferred to other kinds of objects for use as a method.
    // 1. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    // 2. Let len be ? LengthOfArrayLike(O).
    let len = ops::length_of_array_like(agent, o)?;
    // 3. If len = 0, return -1𝔽.
    if len == 0.0 {
        return Ok(Value::Number(-1.0));
    }
    // 4. If fromIndex is not present, let k be len - 1; else let k be min(? ToAbsoluteIndex(fromIndex, len), len - 1).
    let mut k = if args.len() < 2 {
        len - 1.0
    } else {
        let from_index = args.get_or_undefined(1);
        to_absolute_index(agent, from_index, len)?.min(len - 1.0)
    };
    // 5. Repeat, while k ≥ 0,
    while k >= 0.0 {
        // a. Let Pk be ! ToString(𝔽(k)).
        let pk = agent.key_from_index(k);
        // b. Let kPresent be ? HasProperty(O, Pk).
        // c. If kPresent is true, then
        if ops::has_property(agent, o, pk)? {
            // i. Let elementK be ? Get(O, Pk).
            let element_k = ops::get(agent, o, pk)?;
            // ii. If IsStrictlyEqual(searchElement, elementK) is true, return 𝔽(k).
            if ops::is_strictly_equal(agent, args.get_or_undefined(0), element_k) {
                return Ok(Value::Number(k));
            }
        }
        // d. Set k to k - 1.
        k -= 1.0;
    }
    // 6. Return -1𝔽.
    Ok(Value::Number(-1.0))
}

/// ToAbsoluteIndex(value, length) (§7.1.22)
fn to_absolute_index(agent: &mut Agent, value: Value, length: f64) -> JsResult<f64> {
    // 1. Let int be ? ToIntegerOrInfinity(value).
    let mut int = ops::to_integer_or_infinity(agent, value)?;
    // 2. If int is finite and int < 0, set int to length + int.
    if int.is_finite() && int < 0.0 {
        int += length;
    }
    // 3. Return int.
    Ok(int)
}
