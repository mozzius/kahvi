//! Iterator.prototype.windows ( _windowSize_ [ , _undersized_ ] )
//!
//! Spec: https://tc39.es/proposal-iterator-chunking/#sec-iterator.prototype.windows

use std::collections::VecDeque;

use kahvi_core::ops::IteratorRecord;
use kahvi_core::prelude::*;

use crate::support::iterator_helper::{HelperClosure, create_iterator_helper};

/// Iterator.prototype.windows
pub(crate) fn prototype_windows(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let window_size = args.get_or_undefined(0);
    let undersized = args.get_or_undefined(1);
    // 1. Let O be the this value.
    // 2. If O is not an Object, throw a TypeError exception.
    let Value::Object(o) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.windows called on a non-object"));
    };
    // 3. Let iterated be the Iterator Record { [[Iterator]]: O, [[NextMethod]]: undefined, [[Done]]: false }.
    let iterated = IteratorRecord {
        iterator: o,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. If windowSize is not an integral Number, then
    let size = match window_size {
        Value::Number(n) if n.is_finite() && n.trunc() == n => n,
        _ => {
            // a. Let error be ThrowCompletion(a newly created TypeError object).
            let error = agent.throw_type_error(
                "Iterator.prototype.windows: windowSize must be an integral Number",
            );
            // b. Return ? IteratorClose(iterated, error).
            return ops::iterator_close(agent, &iterated, Err(error));
        }
    };
    // 5. If windowSize is not in the inclusive interval from 1𝔽 to 𝔽(2^32 - 1), then
    if !(1.0..=4294967295.0).contains(&size) {
        // a. Let error be ThrowCompletion(a newly created RangeError object).
        let error =
            agent.throw_range_error("Iterator.prototype.windows: windowSize is out of range");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 6. If undersized is undefined, set undersized to "only-full".
    // 7. If undersized is neither "only-full" nor "allow-partial", then
    let allow_partial = match undersized {
        Value::Undefined => false,
        Value::String(s) if agent.str_lossy(s) == "only-full" => false,
        Value::String(s) if agent.str_lossy(s) == "allow-partial" => true,
        _ => {
            // a. Let error be ThrowCompletion(a newly created TypeError object).
            let error = agent.throw_type_error(
                "Iterator.prototype.windows: undersized must be \"only-full\" or \"allow-partial\"",
            );
            // b. Return ? IteratorClose(iterated, error).
            return ops::iterator_close(agent, &iterated, Err(error));
        }
    };
    // 8. Set iterated to ? GetIteratorDirect(O).
    let iterated = ops::get_iterator_direct(agent, o)?;
    // 9. Let closure be a new Abstract Closure with no parameters that captures iterated, windowSize,
    //    and undersized and performs the following steps when called:
    let closure = WindowsClosure {
        window_size: size as usize,
        allow_partial,
        buffer: VecDeque::new(),
        finished: false,
    };
    // 10. Let result be CreateIteratorFromClosure(closure, "Iterator Helper", %IteratorHelperPrototype%,
    //     « [[UnderlyingIterators]] »).
    // 11. Set result.[[UnderlyingIterators]] to « iterated ».
    // 12. Return result.
    Ok(Value::Object(create_iterator_helper(
        agent,
        Box::new(closure),
        vec![iterated],
    )))
}

/// Step 9's closure.
#[derive(Debug)]
struct WindowsClosure {
    window_size: usize,
    allow_partial: bool,
    /// a. Let buffer be a new empty List.
    buffer: VecDeque<Value>,
    /// The partial window has been yielded (step b.ii.1.a); the next resumption returns.
    finished: bool,
}

impl HelperClosure for WindowsClosure {
    // b. Repeat,
    fn resume(
        &mut self,
        agent: &mut Agent,
        underlying: &mut [IteratorRecord],
    ) -> JsResult<Option<Value>> {
        if self.finished {
            // b.ii.2. Return ReturnCompletion(undefined).
            return Ok(None);
        }
        loop {
            // i. Let value be ? IteratorStepValue(iterated).
            let Some(value) = ops::iterator_step_value(agent, &mut underlying[0])? else {
                // ii. If value is done, then
                //     1. If undersized is "allow-partial", buffer is not empty, and the number of
                //        elements in buffer < ℝ(windowSize), then
                if self.allow_partial
                    && !self.buffer.is_empty()
                    && self.buffer.len() < self.window_size
                {
                    // a. Perform Completion(Yield(CreateArrayFromList(buffer))).
                    self.finished = true;
                    let window: Vec<Value> = self.buffer.iter().copied().collect();
                    return Ok(Some(Value::Object(ops::create_array_from_list(
                        agent, &window,
                    ))));
                }
                // 2. Return ReturnCompletion(undefined).
                return Ok(None);
            };
            // iii. If the number of elements in buffer is ℝ(windowSize), then
            if self.buffer.len() == self.window_size {
                // 1. Remove the first element from buffer.
                self.buffer.pop_front();
            }
            // iv. Append value to buffer.
            self.buffer.push_back(value);
            // v. If the number of elements in buffer is ℝ(windowSize), then
            if self.buffer.len() == self.window_size {
                // 1. Let completion be Completion(Yield(CreateArrayFromList(buffer))).
                // 2. IfAbruptCloseIterator(completion, iterated): HelperClosure::on_return.
                let window: Vec<Value> = self.buffer.iter().copied().collect();
                return Ok(Some(Value::Object(ops::create_array_from_list(
                    agent, &window,
                ))));
            }
        }
    }
}

impl kahvi_core::gc::Trace for WindowsClosure {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        for v in &self.buffer {
            v.trace(t);
        }
    }
}
