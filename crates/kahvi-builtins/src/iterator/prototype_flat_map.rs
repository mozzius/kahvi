//! Iterator.prototype.flatMap ( _mapper_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-iterator.prototype.flatmap

use kahvi_core::prelude::*;

use kahvi_core::ops::IteratorRecord;

use crate::support::iterator_helper::{
    HelperClosure, create_iterator_helper, if_abrupt_close_iterator, iterator_close_all,
};

/// Iterator.prototype.flatMap
pub(crate) fn prototype_flat_map(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let mapper = args.get_or_undefined(0);
    // This method performs the following steps when called:
    //
    // 1. Let _obj_ be the *this* value.
    // 2. If _obj_ is not an Object, throw a *TypeError* exception.
    let Value::Object(obj) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.flatMap called on a non-object"));
    };
    // 3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
    let iterated = IteratorRecord {
        iterator: obj,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. If IsCallable(_mapper_) is *false*, then
    if !ops::is_callable(agent, mapper) {
        // a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
        let error = agent.throw_type_error("Iterator.prototype.flatMap: mapper is not callable");
        // b. Return ? IteratorClose(_iterated_, _error_).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 5. Set _iterated_ to ? GetIteratorDirect(_obj_).
    let iterated = ops::get_iterator_direct(agent, obj)?;
    // 6. Let _closure_ be a new Abstract Closure with no parameters that captures _iterated_ and _mapper_ and performs the following steps when called:
    let closure = FlatMapClosure {
        mapper,
        // a. Let _counter_ be *+0*_𝔽.
        counter: 0.0,
        inner: None,
    };
    // 7. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
    // 8. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
    // 9. Return _result_.
    Ok(Value::Object(create_iterator_helper(
        agent,
        Box::new(closure),
        vec![iterated],
    )))
}

/// Step 6's closure: captures iterated and mapper, plus the inner iterator
/// that is alive between Yields (_innerIterator_ in step 6.b.v).
#[derive(Debug)]
struct FlatMapClosure {
    mapper: Value,
    /// a. Let _counter_ be *+0*_𝔽.
    counter: f64,
    /// _innerIterator_ while _innerAlive_ is *true* (step 6.b.viii).
    inner: Option<IteratorRecord>,
}

impl HelperClosure for FlatMapClosure {
    // b. Repeat,
    fn resume(
        &mut self,
        agent: &mut Agent,
        underlying: &mut [IteratorRecord],
    ) -> JsResult<Option<Value>> {
        loop {
            // viii. Repeat, while _innerAlive_ is *true*,
            if let Some(mut inner) = self.inner {
                // 1. Let _innerValue_ be Completion(IteratorStepValue(_innerIterator_)).
                let inner_value = ops::iterator_step_value(agent, &mut inner);
                // 2. IfAbruptCloseIterator(_innerValue_, _iterated_).
                let inner_value = if_abrupt_close_iterator(agent, inner_value, &underlying[0]);
                // The record's [[Done]] may have changed; keep it for the next resume.
                self.inner = Some(inner);
                match inner_value? {
                    // 3. If _innerValue_ is ~done~, then
                    None => {
                        // a. Set _innerAlive_ to *false*.
                        self.inner = None;
                        // ix. NOTE: The following step will not change _counter_ once it reaches *2^53*_𝔽.
                        // x. Set _counter_ to _counter_ + *1*_𝔽.
                        if self.counter < 9007199254740992.0 {
                            self.counter += 1.0;
                        }
                    }
                    // 4. Else,
                    Some(inner_value) => {
                        // a. Let _completion_ be Completion(Yield(_innerValue_)).
                        // b. If _completion_ is an abrupt completion, then: see on_return.
                        return Ok(Some(inner_value));
                    }
                }
                continue;
            }
            // i. Let _value_ be ? IteratorStepValue(_iterated_).
            // ii. If _value_ is ~done~, return ReturnCompletion(*undefined*).
            let Some(value) = ops::iterator_step_value(agent, &mut underlying[0])? else {
                return Ok(None);
            };
            // iii. Let _mapped_ be Completion(Call(_mapper_, *undefined*, « _value_, _counter_ »)).
            let mapped = ops::call(
                agent,
                self.mapper,
                Value::Undefined,
                &[value, Value::Number(self.counter)],
            );
            // iv. IfAbruptCloseIterator(_mapped_, _iterated_).
            let mapped = if_abrupt_close_iterator(agent, mapped, &underlying[0])?;
            // v. Let _innerIterator_ be Completion(GetIteratorFlattenable(_mapped_, ~reject-primitives~)).
            let inner_iterator = get_iterator_flattenable_reject_primitives(agent, mapped);
            // vi. IfAbruptCloseIterator(_innerIterator_, _iterated_).
            let inner_iterator = if_abrupt_close_iterator(agent, inner_iterator, &underlying[0])?;
            // vii. Let _innerAlive_ be *true*.
            self.inner = Some(inner_iterator);
        }
    }

    // The closure was resumed at a Yield (step 6.b.viii.4.b) with a return completion.
    fn on_return(&mut self, agent: &mut Agent, underlying: &mut [IteratorRecord]) -> JsResult<()> {
        if let Some(inner) = self.inner.take() {
            // i. Let _backupCompletion_ be Completion(IteratorClose(_innerIterator_, _completion_)).
            let backup_completion = ops::iterator_close(agent, &inner, Ok(()));
            // ii. IfAbruptCloseIterator(_backupCompletion_, _iterated_).
            if_abrupt_close_iterator(agent, backup_completion, &underlying[0])?;
        }
        // iii. Return ? IteratorClose(_iterated_, _completion_).
        iterator_close_all(agent, underlying, Ok(()))
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
        return Err(
            agent.throw_type_error("Iterator.prototype.flatMap: mapper result is not an object")
        );
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
            return Err(
                agent.throw_type_error("Iterator.prototype.flatMap: iterator is not an object")
            );
        };
        iterator
    };
    // 6. Return ? GetIteratorDirect(_iterator_).
    ops::get_iterator_direct(agent, iterator)
}

impl kahvi_core::gc::Trace for FlatMapClosure {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        t.value(self.mapper);
        self.inner.trace(t);
    }
}
