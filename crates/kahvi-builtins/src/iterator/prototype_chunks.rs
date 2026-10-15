//! Iterator.prototype.chunks ( _chunkSize_ )
//!
//! Spec: https://tc39.es/proposal-iterator-chunking/#sec-iterator.prototype.chunks

use kahvi_core::ops::IteratorRecord;
use kahvi_core::prelude::*;

use crate::support::iterator_helper::{HelperClosure, create_iterator_helper};

/// Iterator.prototype.chunks
pub(crate) fn prototype_chunks(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let chunk_size = args.get_or_undefined(0);
    // 1. Let O be the this value.
    // 2. If O is not an Object, throw a TypeError exception.
    let Value::Object(o) = this else {
        return Err(agent.throw_type_error("Iterator.prototype.chunks called on a non-object"));
    };
    // 3. Let iterated be the Iterator Record { [[Iterator]]: O, [[NextMethod]]: undefined, [[Done]]: false }.
    let iterated = IteratorRecord {
        iterator: o,
        next_method: Value::Undefined,
        done: false,
    };
    // 4. If chunkSize is not an integral Number, then
    let size = match chunk_size {
        Value::Number(n) if n.is_finite() && n.trunc() == n => n,
        _ => {
            // a. Let error be ThrowCompletion(a newly created TypeError object).
            let error = agent.throw_type_error(
                "Iterator.prototype.chunks: chunkSize must be an integral Number",
            );
            // b. Return ? IteratorClose(iterated, error).
            return ops::iterator_close(agent, &iterated, Err(error));
        }
    };
    // 5. If chunkSize is not in the inclusive interval from 1𝔽 to 𝔽(2^32 - 1), then
    if !(1.0..=4294967295.0).contains(&size) {
        // a. Let error be ThrowCompletion(a newly created RangeError object).
        let error = agent.throw_range_error("Iterator.prototype.chunks: chunkSize is out of range");
        // b. Return ? IteratorClose(iterated, error).
        return ops::iterator_close(agent, &iterated, Err(error));
    }
    // 6. Set iterated to ? GetIteratorDirect(O).
    let iterated = ops::get_iterator_direct(agent, o)?;
    // 7. Let closure be a new Abstract Closure with no parameters that captures iterated and chunkSize
    //    and performs the following steps when called:
    let closure = ChunksClosure {
        chunk_size: size as usize,
        buffer: Vec::new(),
        finished: false,
    };
    // 8. Let result be CreateIteratorFromClosure(closure, "Iterator Helper", %IteratorHelperPrototype%,
    //    « [[UnderlyingIterators]] »).
    // 9. Set result.[[UnderlyingIterators]] to « iterated ».
    // 10. Return result.
    Ok(Value::Object(create_iterator_helper(
        agent,
        Box::new(closure),
        vec![iterated],
    )))
}

/// Step 7's closure.
#[derive(Debug)]
struct ChunksClosure {
    chunk_size: usize,
    /// a. Let buffer be a new empty List.
    buffer: Vec<Value>,
    /// The last partial chunk has been yielded (step b.ii.1.a); the next resumption returns.
    finished: bool,
}

impl HelperClosure for ChunksClosure {
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
                //     1. If buffer is not empty, then
                if !self.buffer.is_empty() {
                    // a. Perform Completion(Yield(CreateArrayFromList(buffer))).
                    self.finished = true;
                    let chunk = std::mem::take(&mut self.buffer);
                    return Ok(Some(Value::Object(ops::create_array_from_list(
                        agent, &chunk,
                    ))));
                }
                // 2. Return ReturnCompletion(undefined).
                return Ok(None);
            };
            // iii. Append value to buffer.
            self.buffer.push(value);
            // iv. If the number of elements in buffer is ℝ(chunkSize), then
            if self.buffer.len() == self.chunk_size {
                // 1. Let completion be Completion(Yield(CreateArrayFromList(buffer))).
                // 2. IfAbruptCloseIterator(completion, iterated): HelperClosure::on_return.
                // 3. Set buffer to a new empty List.
                let chunk = std::mem::take(&mut self.buffer);
                return Ok(Some(Value::Object(ops::create_array_from_list(
                    agent, &chunk,
                ))));
            }
        }
    }
}

impl kahvi_core::gc::Trace for ChunksClosure {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        for v in &self.buffer {
            v.trace(t);
        }
    }
}
