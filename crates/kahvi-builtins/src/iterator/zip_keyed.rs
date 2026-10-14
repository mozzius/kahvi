//! Iterator.zipKeyed ( _iterables_ [ , _options_ ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.zipkeyed

use kahvi_core::ops::IteratorRecord;
use kahvi_core::prelude::*;

use crate::support::iterator_helper::{HelperClosure, create_iterator_helper};

/// Iterator.zipKeyed
pub(crate) fn zip_keyed(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    let _ = (this, new_target);
    // 1. If _iterables_ is not an Object, throw a *TypeError* exception.
    let Value::Object(iterables) = args.get_or_undefined(0) else {
        return Err(agent.throw_type_error("Iterator.zipKeyed: iterables is not an object"));
    };
    // 2. Set _options_ to ? GetOptionsObject(_options_).
    let options = get_options_object(agent, args.get_or_undefined(1))?;
    // 3. Let _mode_ be ? Get(_options_, *"mode"*).
    let mode_key = agent.key("mode");
    let mode = ops::get(agent, options, mode_key)?;
    // 4. If _mode_ is *undefined*, set _mode_ to *"shortest"*.
    // 5. If _mode_ is not one of *"shortest"*, *"longest"*, or *"strict"*, throw a *TypeError* exception.
    let mode = match mode {
        Value::Undefined => Mode::Shortest,
        v if is_string_value(agent, v, "shortest") => Mode::Shortest,
        v if is_string_value(agent, v, "longest") => Mode::Longest,
        v if is_string_value(agent, v, "strict") => Mode::Strict,
        _ => {
            return Err(agent.throw_type_error(
                "Iterator.zipKeyed: mode must be \"shortest\", \"longest\" or \"strict\"",
            ));
        }
    };
    // 6. Let _paddingOption_ be *undefined*.
    let mut padding_option = Value::Undefined;
    // 7. If _mode_ is *"longest"*, then
    if mode == Mode::Longest {
        // a. Set _paddingOption_ to ? Get(_options_, *"padding"*).
        let padding_key = agent.key("padding");
        padding_option = ops::get(agent, options, padding_key)?;
        // b. If _paddingOption_ is not *undefined* and _paddingOption_ is not an Object, throw a *TypeError* exception.
        if !matches!(padding_option, Value::Undefined | Value::Object(_)) {
            return Err(agent.throw_type_error("Iterator.zipKeyed: padding must be an object"));
        }
    }
    // 8. Let _iters_ be a new empty List.
    let mut iters: Vec<IteratorRecord> = Vec::new();
    // 9. Let _padding_ be a new empty List.
    let mut padding: Vec<Value> = Vec::new();
    // 10. Let _allKeys_ be ? _iterables_.[[OwnPropertyKeys]]().
    let all_keys = iterables.internal_own_property_keys(agent)?;
    // 11. Let _keys_ be a new empty List.
    let mut keys: Vec<PropertyKey> = Vec::new();
    // 12. For each element _key_ of _allKeys_, do
    for key in all_keys {
        // a. Let _propertyDesc_ be Completion(_iterables_.[[GetOwnProperty]](_key_)).
        let property_desc = iterables.internal_get_own_property(agent, key);
        // b. IfAbruptCloseIterators(_propertyDesc_, _iters_).
        let property_desc = if_abrupt_close_iterators(agent, property_desc, &iters)?;
        // c. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Enumerable]] is *true*, then
        if property_desc.is_some_and(|d| d.enumerable == Some(true)) {
            // i. Let _value_ be Completion(Get(_iterables_, _key_)).
            let value = ops::get(agent, iterables, key);
            // ii. IfAbruptCloseIterators(_value_, _iters_).
            let value = if_abrupt_close_iterators(agent, value, &iters)?;
            // iii. If _value_ is not *undefined*, then
            if !matches!(value, Value::Undefined) {
                // 1. Append _key_ to _keys_.
                keys.push(key);
                // 2. Let _iter_ be Completion(GetIteratorFlattenable(_value_, ~reject-primitives~)).
                let iter = get_iterator_flattenable_reject_primitives(agent, value);
                // 3. IfAbruptCloseIterators(_iter_, _iters_).
                let iter = if_abrupt_close_iterators(agent, iter, &iters)?;
                // 4. Append _iter_ to _iters_.
                iters.push(iter);
            }
        }
    }
    // 13. Let _iterCount_ be the number of elements in _iters_.
    let iter_count = iters.len();
    // 14. If _mode_ is *"longest"*, then
    if mode == Mode::Longest {
        if let Value::Object(padding_object) = padding_option {
            // b. Else,
            // i. For each element _key_ of _keys_, do
            for &key in &keys {
                // 1. Let _value_ be Completion(Get(_paddingOption_, _key_)).
                let value = ops::get(agent, padding_object, key);
                // 2. IfAbruptCloseIterators(_value_, _iters_).
                let value = if_abrupt_close_iterators(agent, value, &iters)?;
                // 3. Append _value_ to _padding_.
                padding.push(value);
            }
        } else {
            // a. If _paddingOption_ is *undefined*, then
            // i. Repeat _iterCount_ times:
            //    1. Append *undefined* to _padding_.
            padding.resize(iter_count, Value::Undefined);
        }
    }
    // 15. Let _finishResults_ be a new Abstract Closure with parameters (_results_) that captures _keys_ and _iterCount_
    //     and performs the following steps when called:
    //    a. Let _obj_ be OrdinaryObjectCreate(*null*).
    //    b. For each integer _i_ such that 0 ≤ _i_ < _iterCount_, in ascending order, do
    //       i. Perform ! CreateDataPropertyOrThrow(_obj_, _keys_[_i_], _results_[_i_]).
    //    c. Return _obj_.
    let finish_results = ZipResults::Keyed(keys);
    // 16. Return IteratorZip(_iters_, _mode_, _padding_, _finishResults_).
    Ok(Value::Object(iterator_zip(
        agent,
        iters,
        mode,
        padding,
        finish_results,
    )))
}

/// The _mode_ of IteratorZip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Shortest,
    Longest,
    Strict,
}

/// The _finishResults_ Abstract Closure of IteratorZip, as data. Iterator.zip adds an Array variant.
#[derive(Clone, Debug)]
enum ZipResults {
    /// Step 15 of Iterator.zipKeyed: the keys captured by the closure, one per iterator.
    Keyed(Vec<PropertyKey>),
}

impl ZipResults {
    /// Calls _finishResults_(_results_).
    fn finish(&self, agent: &mut Agent, results: &[Value]) -> JsResult<Value> {
        match self {
            ZipResults::Keyed(keys) => {
                // a. Let _obj_ be OrdinaryObjectCreate(*null*).
                let obj = agent.ordinary_object_create(None);
                // b. For each integer _i_ such that 0 ≤ _i_ < _iterCount_, in ascending order, do
                for (key, value) in keys.iter().zip(results) {
                    // i. Perform ! CreateDataPropertyOrThrow(_obj_, _keys_[_i_], _results_[_i_]).
                    ops::create_data_property_or_throw(agent, obj, *key, *value)?;
                }
                // c. Return _obj_.
                Ok(Value::Object(obj))
            }
        }
    }
}

/// IteratorZip ( _iters_, _mode_, _padding_, _finishResults_ ) (§7.4.14 of the Iterator helpers)
///
/// Returns the Iterator Helper generator the spec describes. Its
/// [[UnderlyingIterators]] is `openIters`; `null` entries of `iters` are
/// the ones no longer in `openIters`, tracked by `open` below.
fn iterator_zip(
    agent: &mut Agent,
    iters: Vec<IteratorRecord>,
    mode: Mode,
    padding: Vec<Value>,
    finish_results: ZipResults,
) -> Object {
    // 1. Let _iterCount_ be the number of elements in _iters_.
    let iter_count = iters.len();
    // 2. Let _openIters_ be a copy of _iters_.
    // 3. Let _closure_ be a new Abstract Closure ... (see ZipClosure::resume)
    let closure = ZipClosure {
        iter_count,
        open: vec![true; iter_count],
        mode,
        padding,
        finish_results,
    };
    // 4. Let _gen_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 5. Set _gen_.[[UnderlyingIterators]] to _openIters_.
    // 6. Return _gen_.
    create_iterator_helper(agent, Box::new(closure), iters)
}

/// IteratorZip's closure: captures _iterCount_, _openIters_ (as `open`
/// flags over the helper's underlying iterators), _mode_, _padding_ and _finishResults_.
#[derive(Debug)]
struct ZipClosure {
    iter_count: usize,
    /// Whether `iters[i]` is in _openIters_ (false once it is removed, or is `null`).
    open: Vec<bool>,
    mode: Mode,
    padding: Vec<Value>,
    finish_results: ZipResults,
}

impl HelperClosure for ZipClosure {
    // 3.b. Repeat,
    fn resume(
        &mut self,
        agent: &mut Agent,
        underlying: &mut [IteratorRecord],
    ) -> JsResult<Option<Value>> {
        // a. If _iterCount_ = 0, return ReturnCompletion(*undefined*).
        if self.iter_count == 0 {
            return Ok(None);
        }
        // i. Let _results_ be a new empty List.
        let mut results = Vec::with_capacity(self.iter_count);
        // ii. Assert: _openIters_ is not empty.
        // iii. For each integer _i_ such that 0 ≤ _i_ < _iterCount_, in ascending order, do
        for i in 0..self.iter_count {
            // 1. Let _iter_ be _iters_[_i_].
            // 2. If _iter_ is *null*, then
            let result = if !self.open[i] {
                // a. Assert: _mode_ is *"longest"*.
                // b. Let _result_ be _padding_[_i_].
                self.padding[i]
            } else {
                // 3. Else,
                // a. Let _result_ be Completion(IteratorStepValue(_iter_)).
                let step = ops::iterator_step_value(agent, &mut underlying[i]);
                // b. If _result_ is an abrupt completion, then
                let next = match step {
                    Ok(next) => next,
                    Err(e) => {
                        // i. Remove _iter_ from _openIters_.
                        self.open[i] = false;
                        // ii. Return ? IteratorCloseAll(_openIters_, _result_).
                        return iterator_close_all(agent, underlying, &self.open, Err(e));
                    }
                };
                // c. Set _result_ to ! _result_.
                match next {
                    Some(value) => value,
                    None => {
                        // d. If _result_ is ~done~, then
                        // i. Remove _iter_ from _openIters_.
                        self.open[i] = false;
                        match self.mode {
                            // ii. If _mode_ is *"shortest"*, then
                            Mode::Shortest => {
                                // 1. Return ? IteratorCloseAll(_openIters_, ReturnCompletion(*undefined*)).
                                return iterator_close_all(agent, underlying, &self.open, Ok(None));
                            }
                            // iii. Else if _mode_ is *"strict"*, then
                            Mode::Strict => {
                                // 1. If _i_ ≠ 0, then
                                if i != 0 {
                                    // a. Return ? IteratorCloseAll(_openIters_, ThrowCompletion(a newly created *TypeError* object)).
                                    let error = agent.throw_type_error(
                                        "Iterator.zipKeyed: iterables have different lengths",
                                    );
                                    return iterator_close_all(
                                        agent,
                                        underlying,
                                        &self.open,
                                        Err(error),
                                    );
                                }
                                // 2. For each integer _k_ such that 1 ≤ _k_ < _iterCount_, in ascending order, do
                                for k in 1..self.iter_count {
                                    // a. Assert: _iters_[_k_] is not *null*.
                                    // b. Let _open_ be Completion(IteratorStep(_iters_[_k_])).
                                    let step = ops::iterator_step(agent, &mut underlying[k]);
                                    // c. If _open_ is an abrupt completion, then
                                    let open_result = match step {
                                        Ok(open_result) => open_result,
                                        Err(e) => {
                                            // i. Remove _iters_[_k_] from _openIters_.
                                            self.open[k] = false;
                                            // ii. Return ? IteratorCloseAll(_openIters_, _open_).
                                            return iterator_close_all(
                                                agent,
                                                underlying,
                                                &self.open,
                                                Err(e),
                                            );
                                        }
                                    };
                                    // d. Set _open_ to ! _open_.
                                    // e. If _open_ is ~done~, then
                                    if open_result.is_none() {
                                        // i. Remove _iters_[_k_] from _openIters_.
                                        self.open[k] = false;
                                    } else {
                                        // f. Else,
                                        // i. Return ? IteratorCloseAll(_openIters_, ThrowCompletion(a newly created *TypeError* object)).
                                        let error = agent.throw_type_error(
                                            "Iterator.zipKeyed: iterables have different lengths",
                                        );
                                        return iterator_close_all(
                                            agent,
                                            underlying,
                                            &self.open,
                                            Err(error),
                                        );
                                    }
                                }
                                // 3. Return ReturnCompletion(*undefined*).
                                return Ok(None);
                            }
                            // iv. Else,
                            Mode::Longest => {
                                // 1. Assert: _mode_ is *"longest"*.
                                // 2. If _openIters_ is empty, return ReturnCompletion(*undefined*).
                                if !self.open.iter().any(|open| *open) {
                                    return Ok(None);
                                }
                                // 3. Set _iters_[_i_] to *null*. (Its `open` flag is already false.)
                                // 4. Set _result_ to _padding_[_i_].
                                self.padding[i]
                            }
                        }
                    }
                }
            };
            // 4. Append _result_ to _results_.
            results.push(result);
        }
        // iv. Set _results_ to _finishResults_(_results_).
        let results = self.finish_results.finish(agent, &results)?;
        // v. Let _completion_ be Completion(Yield(_results_)).
        // vi. IfAbruptCloseIterators(_completion_, _openIters_): HelperClosure::on_return.
        Ok(Some(results))
    }

    // 3.b.vi. IfAbruptCloseIterators(_completion_, _openIters_) for a return completion.
    fn on_return(&mut self, agent: &mut Agent, underlying: &mut [IteratorRecord]) -> JsResult<()> {
        iterator_close_all(agent, underlying, &self.open, Ok(()))
    }
}

/// IteratorCloseAll(_iters_, _completion_) (§7.4.12), over the entries of
/// `iters` whose `open` flag is set (the entries of _openIters_).
fn iterator_close_all<T>(
    agent: &mut Agent,
    iters: &[IteratorRecord],
    open: &[bool],
    completion: JsResult<T>,
) -> JsResult<T> {
    let mut completion = completion;
    for (record, _) in iters.iter().zip(open).rev().filter(|(_, open)| **open) {
        completion = ops::iterator_close(agent, record, completion);
    }
    completion
}

/// IfAbruptCloseIterators(_value_, _iters_) (§7.4.13) for a list of iterators that are all open.
fn if_abrupt_close_iterators<T>(
    agent: &mut Agent,
    value: JsResult<T>,
    iters: &[IteratorRecord],
) -> JsResult<T> {
    match value {
        Ok(v) => Ok(v),
        // If value is an abrupt completion, return ? IteratorCloseAll(iters, value).
        Err(e) => {
            let open = vec![true; iters.len()];
            iterator_close_all(agent, iters, &open, Err(e))
        }
    }
}

/// GetOptionsObject ( _options_ ) (§7.3.22)
fn get_options_object(agent: &mut Agent, options: Value) -> JsResult<Object> {
    match options {
        // 1. If _options_ is *undefined*, then
        // a. Return OrdinaryObjectCreate(*null*).
        Value::Undefined => Ok(agent.ordinary_object_create(None)),
        // 2. If _options_ is an Object, then
        // a. Return _options_.
        Value::Object(o) => Ok(o),
        // 3. Throw a *TypeError* exception.
        _ => Err(agent.throw_type_error("Iterator.zipKeyed: options must be an object")),
    }
}

/// GetIteratorFlattenable ( _obj_, ~reject-primitives~ ) (§7.4.17)
fn get_iterator_flattenable_reject_primitives(
    agent: &mut Agent,
    obj: Value,
) -> JsResult<IteratorRecord> {
    // 1. If _obj_ is not an Object, then
    let Value::Object(obj) = obj else {
        // a. If _primitiveHandling_ is ~reject-primitives~, throw a *TypeError* exception.
        return Err(agent.throw_type_error("Iterator.zipKeyed: iterable is not an object"));
    };
    // 2. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
    let method = ops::get_method(
        agent,
        Value::Object(obj),
        PropertyKey::Symbol(agent.symbols_wk.iterator),
    )?;
    // 3. If _method_ is *undefined*, then
    let iterator = if matches!(method, Value::Undefined) {
        // a. Let _iterator_ be _obj_.
        obj
    } else {
        // 4. Else,
        // a. Let _iterator_ be ? Call(_method_, _obj_).
        let iterator = ops::call(agent, method, Value::Object(obj), &[])?;
        // 5. If _iterator_ is not an Object, throw a *TypeError* exception.
        let Value::Object(iterator) = iterator else {
            return Err(agent.throw_type_error("Iterator.zipKeyed: iterator is not an object"));
        };
        iterator
    };
    // 6. Return ? GetIteratorDirect(_iterator_).
    ops::get_iterator_direct(agent, iterator)
}

/// Whether `v` is the String whose code units are `expected`.
fn is_string_value(agent: &Agent, v: Value, expected: &str) -> bool {
    matches!(v, Value::String(s) if agent.str(s).iter().copied().eq(expected.encode_utf16()))
}
