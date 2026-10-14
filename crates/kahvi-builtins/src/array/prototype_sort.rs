//! Array.prototype.sort ( _comparator_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.prototype.sort

use kahvi_core::prelude::*;

/// Array.prototype.sort
pub(crate) fn prototype_sort(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This method sorts the elements of this array. If _comparator_ is not *undefined*, it should be a function that accepts two arguments _x_ and _y_ and returns a negative Number if _x_ < _y_, a positive Number if _x_ > _y_, or a zero otherwise.
    // It performs the following steps when called:
    //
    // 1. If _comparator_ is not *undefined* and IsCallable(_comparator_) is *false*, throw a *TypeError* exception.
    let comparator = args.get_or_undefined(0);
    if !matches!(comparator, Value::Undefined) && !ops::is_callable(agent, comparator) {
        return Err(agent
            .throw_type_error("Array.prototype.sort comparator must be a function or undefined"));
    }
    // 2. Let _obj_ be ? ToObject(*this* value).
    let obj = ops::to_object(agent, this)?;
    // 3. Let _length_ be ? LengthOfArrayLike(_obj_).
    let length = ops::length_of_array_like(agent, obj)?;
    // 4. Let _sortCompare_ be a new Abstract Closure with parameters (_x_, _y_) that captures _comparator_ and performs the following steps when called:
    //    a. Return ? CompareArrayElements(_x_, _y_, _comparator_).
    let mut sort_compare =
        |agent: &mut Agent, x: Value, y: Value| compare_array_elements(agent, x, y, comparator);
    // The comparator may collect garbage: obj and the comparator stay rooted
    // (SortIndexedProperties roots the items).
    agent.with_roots(&[Value::Object(obj), comparator], |agent| {
        // 5. Let _sortedList_ be ? SortIndexedProperties(_obj_, _length_, _sortCompare_, ~skip-holes~).
        let sorted_list = sort_indexed_properties(agent, obj, length, &mut sort_compare, true)?;
        // 6. Let _itemCount_ be the number of elements in _sortedList_.
        // 7. Let _j_ be 0.
        let mut j = 0.0;
        // 8. Repeat, while _j_ < _itemCount_,
        //    (the loop runs once per element of _sortedList_, so _j_ < _itemCount_ holds throughout)
        for item in sorted_list {
            // a. Perform ? Set(_obj_, ! ToString(𝔽(_j_)), _sortedList_[_j_], *true*).
            let pj = agent.key_from_index(j);
            ops::set(agent, obj, pj, item, true)?;
            // b. Set _j_ to _j_ + 1.
            j += 1.0;
        }
        // 9. NOTE: The call to SortIndexedProperties in step [step-array-sortindexedproperties] uses ~skip-holes~. The remaining indices are deleted to preserve the number of holes that were detected and excluded from the sort.
        // 10. Repeat, while _j_ < _length_,
        while j < length {
            // a. Perform ? DeletePropertyOrThrow(_obj_, ! ToString(𝔽(_j_))).
            let pj = agent.key_from_index(j);
            ops::delete_property_or_throw(agent, obj, pj)?;
            // b. Set _j_ to _j_ + 1.
            j += 1.0;
        }
        // 11. Return _obj_.
        Ok(Value::Object(obj))
    })
}

/// SortIndexedProperties ( _obj_, _length_, _sortCompare_, _holes_ )
///
/// Spec: https://tc39.es/ecma262/#sec-sortindexedproperties
///
/// `skip_holes` is `true` for ~skip-holes~ and `false` for ~read-through-holes~.
pub(crate) fn sort_indexed_properties(
    agent: &mut Agent,
    obj: Object,
    length: f64,
    sort_compare: &mut dyn FnMut(&mut Agent, Value, Value) -> JsResult<f64>,
    skip_holes: bool,
) -> JsResult<Vec<Value>> {
    // 1. Let _items_ be a new empty List.
    let mut items = Vec::new();
    // 2. Let _k_ be 0.
    let mut k = 0.0;
    // 3. Repeat, while _k_ < _length_,
    while k < length {
        // a. Let _Pk_ be ! ToString(𝔽(_k_)).
        let pk = agent.key_from_index(k);
        // b. If _holes_ is ~skip-holes~, then
        //    i. Let _kRead_ be ? HasProperty(_obj_, _Pk_).
        // c. Else,
        //    i. Assert: _holes_ is ~read-through-holes~.
        //    ii. Let _kRead_ be *true*.
        let k_read = if skip_holes {
            ops::has_property(agent, obj, pk)?
        } else {
            true
        };
        // d. If _kRead_ is *true*, then
        if k_read {
            // i. Let _kValue_ be ? Get(_obj_, _Pk_).
            let k_value = ops::get(agent, obj, pk)?;
            // ii. Append _kValue_ to _items_.
            items.push(k_value);
        }
        // e. Set _k_ to _k_ + 1.
        k += 1.0;
    }
    // 4. Sort _items_ using an implementation-defined sequence of calls to _SortCompare_. If any such call returns an abrupt completion, stop before performing any further calls to _SortCompare_ and return that Completion Record.
    // The comparator may collect garbage: the items stay rooted while sorting.
    let roots = items.clone();
    let sorted = agent.with_roots(&roots, |agent| merge_sort(agent, items, sort_compare))?;
    // 5. Return _items_.
    Ok(sorted)
}

/// CompareArrayElements ( _x_, _y_, _comparator_ )
///
/// Spec: https://tc39.es/ecma262/#sec-comparearrayelements
fn compare_array_elements(
    agent: &mut Agent,
    x: Value,
    y: Value,
    comparator: Value,
) -> JsResult<f64> {
    // 1. If _x_ is *undefined* and _y_ is *undefined*, return *+0*𝔽.
    if matches!(x, Value::Undefined) && matches!(y, Value::Undefined) {
        return Ok(0.0);
    }
    // 2. If _x_ is *undefined*, return *1*𝔽.
    if matches!(x, Value::Undefined) {
        return Ok(1.0);
    }
    // 3. If _y_ is *undefined*, return *-1*𝔽.
    if matches!(y, Value::Undefined) {
        return Ok(-1.0);
    }
    // 4. If _comparator_ is not *undefined*, then
    if !matches!(comparator, Value::Undefined) {
        // a. Let _v_ be ? ToNumber(? Call(_comparator_, *undefined*, « _x_, _y_ »)).
        // x and y are among the items SortIndexedProperties keeps rooted.
        let v = ops::call_rooted(agent, comparator, Value::Undefined, &[x, y])?;
        let result = ops::to_number(agent, v)?;
        // b. If _v_ is *NaN*, return *+0*𝔽.
        if result.is_nan() {
            return Ok(0.0);
        }
        // c. Return _v_.
        return Ok(result);
    }
    // 5. Let _xString_ be ? ToString(_x_).
    let x_string = Value::String(ops::to_string(agent, x)?);
    // 6. Let _yString_ be ? ToString(_y_).
    let y_string = Value::String(ops::to_string(agent, y)?);
    // 7. Let _xSmaller_ be ! IsLessThan(_xString_, _yString_, *true*).
    let x_smaller = ops::is_less_than(agent, x_string, y_string, true)?;
    // 8. If _xSmaller_ is *true*, return *-1*𝔽.
    if x_smaller == Some(true) {
        return Ok(-1.0);
    }
    // 9. Let _ySmaller_ be ! IsLessThan(_yString_, _xString_, *true*).
    let y_smaller = ops::is_less_than(agent, y_string, x_string, true)?;
    // 10. If _ySmaller_ is *true*, return *1*𝔽.
    if y_smaller == Some(true) {
        return Ok(1.0);
    }
    // 11. Return *+0*𝔽.
    Ok(0.0)
}

/// A stable merge sort whose comparisons may throw. The first abrupt
/// completion from `sort_compare` is returned and no further calls are made.
fn merge_sort(
    agent: &mut Agent,
    mut items: Vec<Value>,
    sort_compare: &mut dyn FnMut(&mut Agent, Value, Value) -> JsResult<f64>,
) -> JsResult<Vec<Value>> {
    if items.len() <= 1 {
        return Ok(items);
    }
    let right_part = items.split_off(items.len() / 2);
    let left = merge_sort(agent, items, sort_compare)?;
    let right = merge_sort(agent, right_part, sort_compare)?;

    let mut merged = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    loop {
        match (left.get(i).copied(), right.get(j).copied()) {
            (Some(a), Some(b)) => {
                if sort_compare(agent, a, b)? <= 0.0 {
                    merged.push(a);
                    i += 1;
                } else {
                    merged.push(b);
                    j += 1;
                }
            }
            (Some(a), None) => {
                merged.push(a);
                i += 1;
            }
            (None, Some(b)) => {
                merged.push(b);
                j += 1;
            }
            (None, None) => break,
        }
    }
    Ok(merged)
}
