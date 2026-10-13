//! Async generator objects (§27.6): a request queue in front of a frame
//! that awaits like an async function and yields like a generator.
//!
//! The %AsyncGeneratorPrototype% methods in kahvi-builtins call
//! [`async_generator_next`], [`async_generator_return`] and
//! [`async_generator_throw`].

use std::collections::VecDeque;

use super::{Frame, FrameResult, run_frame};
use crate::agent::{Agent, ExecutionContext};
use crate::error::{JsError, JsResult};
use crate::function::{builtin_slots, create_builtin_function};
use crate::object::{ObjectKind, PropertyKey};
use crate::ops::promise::{
    PromiseCapability, new_promise_capability, perform_promise_then, promise_resolve,
};
use crate::ops::{call, create_iter_result_object};
use crate::value::{Args, Object, Value};

/// [[AsyncGeneratorState]]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsyncGeneratorState {
    SuspendedStart,
    SuspendedYield,
    Executing,
    DrainingQueue,
    Completed,
}

/// A completion record carried by a request or a resumption.
#[derive(Debug, Clone, Copy)]
pub enum Completion {
    Normal(Value),
    Return(Value),
    Throw(Value),
}

/// An AsyncGeneratorRequest Record (§27.6.3.1).
#[derive(Debug, Clone, Copy)]
struct Request {
    completion: Completion,
    capability: PromiseCapability,
}

/// The internal slots of an async generator object.
#[derive(Debug)]
pub struct AsyncGeneratorData {
    pub state: AsyncGeneratorState,
    queue: VecDeque<Request>,
    /// [[AsyncGeneratorContext]] while suspended.
    frame: Option<Box<Frame>>,
    function: Object,
}

fn data(agent: &mut Agent, g: Object) -> &mut AsyncGeneratorData {
    agent
        .native_mut::<AsyncGeneratorData>(g)
        .expect("async generator object")
}

/// EvaluateAsyncGeneratorBody (§15.6.2) steps 2-4: the object, suspended at its start.
pub(super) fn create_async_generator(
    agent: &mut Agent,
    f: Object,
    frame: Frame,
) -> JsResult<Object> {
    let d = AsyncGeneratorData {
        state: AsyncGeneratorState::SuspendedStart,
        queue: VecDeque::new(),
        frame: Some(Box::new(frame)),
        function: f,
    };
    crate::ops::ordinary_create_from_constructor(
        agent,
        f,
        "%AsyncGeneratorPrototype%",
        ObjectKind::Native(Box::new(d)),
    )
}

/// The method's promise capability plus AsyncGeneratorValidate: `Err` holds
/// the promise to return when the receiver is not an async generator.
fn begin(
    agent: &mut Agent,
    generator: Value,
) -> JsResult<Result<(Object, PromiseCapability), Value>> {
    // 2. Let promiseCapability be ! NewPromiseCapability(%Promise%).
    let promise_ctor = agent.intrinsic("%Promise%");
    let capability = new_promise_capability(agent, Value::Object(promise_ctor))?;
    // 3-4. Let result be Completion(AsyncGeneratorValidate(generator, empty)); IfAbruptRejectPromise(result, promiseCapability).
    match generator {
        Value::Object(g) if agent.native::<AsyncGeneratorData>(g).is_some() => {
            Ok(Ok((g, capability)))
        }
        _ => {
            let err = agent.throw_type_error("not an async generator");
            let JsError::Throw(e) = err else {
                return Err(err);
            };
            call(agent, capability.reject, Value::Undefined, &[e])?;
            Ok(Err(capability.promise))
        }
    }
}

/// %AsyncGeneratorPrototype%.next ( value ) (§27.6.1.2)
pub fn async_generator_next(agent: &mut Agent, generator: Value, value: Value) -> JsResult<Value> {
    let (g, capability) = match begin(agent, generator)? {
        Ok(v) => v,
        Err(promise) => return Ok(promise),
    };
    // 5. Let state be generator.[[AsyncGeneratorState]].
    let state = data(agent, g).state;
    // 6. If state is completed, then resolve with CreateIteratorResultObject(undefined, true).
    if state == AsyncGeneratorState::Completed {
        let result = create_iter_result_object(agent, Value::Undefined, true);
        call(
            agent,
            capability.resolve,
            Value::Undefined,
            &[Value::Object(result)],
        )?;
        return Ok(capability.promise);
    }
    // 7-8. Let completion be NormalCompletion(value); perform AsyncGeneratorEnqueue(generator, completion, promiseCapability).
    let completion = Completion::Normal(value);
    data(agent, g).queue.push_back(Request {
        completion,
        capability,
    });
    // 9. If state is either suspended-start or suspended-yield, perform AsyncGeneratorResume(generator, completion).
    if matches!(
        state,
        AsyncGeneratorState::SuspendedStart | AsyncGeneratorState::SuspendedYield
    ) {
        resume(agent, g, completion)?;
    }
    // 11. Return promiseCapability.[[Promise]].
    Ok(capability.promise)
}

/// %AsyncGeneratorPrototype%.return ( value ) (§27.6.1.3)
pub fn async_generator_return(
    agent: &mut Agent,
    generator: Value,
    value: Value,
) -> JsResult<Value> {
    let (g, capability) = match begin(agent, generator)? {
        Ok(v) => v,
        Err(promise) => return Ok(promise),
    };
    // 5-6. Let completion be ReturnCompletion(value); perform AsyncGeneratorEnqueue.
    let completion = Completion::Return(value);
    data(agent, g).queue.push_back(Request {
        completion,
        capability,
    });
    let state = data(agent, g).state;
    match state {
        // 8. If state is either suspended-start or completed, then
        AsyncGeneratorState::SuspendedStart | AsyncGeneratorState::Completed => {
            // a. Set generator.[[AsyncGeneratorState]] to draining-queue.
            let d = data(agent, g);
            d.state = AsyncGeneratorState::DrainingQueue;
            d.frame = None;
            // b. Perform AsyncGeneratorAwaitReturn(generator).
            await_return(agent, g)?;
        }
        // 9. Else if state is suspended-yield, perform AsyncGeneratorResume(generator, completion).
        AsyncGeneratorState::SuspendedYield => resume(agent, g, completion)?,
        _ => {}
    }
    Ok(capability.promise)
}

/// %AsyncGeneratorPrototype%.throw ( exception ) (§27.6.1.4)
pub fn async_generator_throw(
    agent: &mut Agent,
    generator: Value,
    exception: Value,
) -> JsResult<Value> {
    let (g, capability) = match begin(agent, generator)? {
        Ok(v) => v,
        Err(promise) => return Ok(promise),
    };
    // 5. Let state be generator.[[AsyncGeneratorState]].
    let mut state = data(agent, g).state;
    // 6. If state is suspended-start, then set state to completed.
    if state == AsyncGeneratorState::SuspendedStart {
        let d = data(agent, g);
        d.state = AsyncGeneratorState::Completed;
        d.frame = None;
        state = AsyncGeneratorState::Completed;
    }
    // 7. If state is completed, then reject with exception.
    if state == AsyncGeneratorState::Completed {
        call(agent, capability.reject, Value::Undefined, &[exception])?;
        return Ok(capability.promise);
    }
    // 8-9. Enqueue ThrowCompletion(exception).
    let completion = Completion::Throw(exception);
    data(agent, g).queue.push_back(Request {
        completion,
        capability,
    });
    // 10. If state is suspended-yield, perform AsyncGeneratorResume(generator, completion).
    if state == AsyncGeneratorState::SuspendedYield {
        resume(agent, g, completion)?;
    }
    Ok(capability.promise)
}

/// AsyncGeneratorCompleteStep(generator, completion, done) (§27.6.3.4)
fn complete_step(agent: &mut Agent, g: Object, completion: Completion, done: bool) -> JsResult<()> {
    // 2-3. Remove the first element from the queue.
    let Some(next) = data(agent, g).queue.pop_front() else {
        return Ok(());
    };
    match completion {
        // 6. If completion is a throw completion, then perform ! Call(promiseCapability.[[Reject]], undefined, « value »).
        Completion::Throw(e) => {
            call(agent, next.capability.reject, Value::Undefined, &[e])?;
        }
        // 7. Else, resolve with CreateIteratorResultObject(value, done).
        Completion::Normal(v) | Completion::Return(v) => {
            let result = create_iter_result_object(agent, v, done);
            call(
                agent,
                next.capability.resolve,
                Value::Undefined,
                &[Value::Object(result)],
            )?;
        }
    }
    Ok(())
}

/// AsyncGeneratorDrainQueue(generator) (§27.6.3.10)
fn drain_queue(agent: &mut Agent, g: Object) -> JsResult<()> {
    loop {
        // 3. If queue is empty, set state to completed and return.
        let Some(next) = data(agent, g).queue.front().copied() else {
            data(agent, g).state = AsyncGeneratorState::Completed;
            return Ok(());
        };
        match next.completion {
            // b. If completion is a return completion, then perform AsyncGeneratorAwaitReturn and stop.
            Completion::Return(_) => return await_return(agent, g),
            // c. Else, a normal completion completes with undefined.
            Completion::Normal(_) => {
                complete_step(agent, g, Completion::Normal(Value::Undefined), true)?
            }
            Completion::Throw(e) => complete_step(agent, g, Completion::Throw(e), true)?,
        }
    }
}

/// AsyncGeneratorAwaitReturn(generator) (§27.6.3.9)
fn await_return(agent: &mut Agent, g: Object) -> JsResult<()> {
    // 4-5. The first request's completion is a return completion.
    let Some(Request {
        completion: Completion::Return(value),
        ..
    }) = data(agent, g).queue.front().copied()
    else {
        return Ok(());
    };
    // 6. Let promiseCompletion be Completion(PromiseResolve(%Promise%, completion.[[Value]])).
    let promise_ctor = agent.intrinsic("%Promise%");
    let promise = match promise_resolve(agent, promise_ctor, value) {
        Ok(p) => p,
        // 7. If promiseCompletion is an abrupt completion, then complete the step with it and drain.
        Err(JsError::Throw(e)) => {
            complete_step(agent, g, Completion::Throw(e), true)?;
            return drain_queue(agent, g);
        }
        Err(e) => return Err(e),
    };
    // 9-14. Settle the request when the promise settles, then keep draining.
    let empty = PropertyKey::String(agent.names.empty);
    let slots = vec![Value::Object(g)];
    let on_fulfilled = create_builtin_function(
        agent,
        await_return_fulfilled,
        1,
        empty,
        slots.clone(),
        None,
        None,
        None,
        false,
    );
    let on_rejected = create_builtin_function(
        agent,
        await_return_rejected,
        1,
        empty,
        slots,
        None,
        None,
        None,
        false,
    );
    perform_promise_then(
        agent,
        promise,
        Value::Object(on_fulfilled),
        Value::Object(on_rejected),
        None,
    );
    Ok(())
}

fn generator_of_active(agent: &Agent) -> Option<Object> {
    let f = agent.active_function()?;
    builtin_slots(agent, f).first().and_then(|v| v.as_object())
}

fn await_return_fulfilled(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let Some(g) = generator_of_active(agent) else {
        return Ok(Value::Undefined);
    };
    complete_step(agent, g, Completion::Normal(args.get_or_undefined(0)), true)?;
    drain_queue(agent, g)?;
    Ok(Value::Undefined)
}

fn await_return_rejected(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let Some(g) = generator_of_active(agent) else {
        return Ok(Value::Undefined);
    };
    complete_step(agent, g, Completion::Throw(args.get_or_undefined(0)), true)?;
    drain_queue(agent, g)?;
    Ok(Value::Undefined)
}

/// AsyncGeneratorResume(generator, completion) (§27.6.3.6), and resumption
/// after an await inside the body: runs the frame until it awaits, yields,
/// returns or throws, handling each as AsyncGeneratorStart and
/// AsyncGeneratorYield specify.
pub(super) fn resume(agent: &mut Agent, g: Object, mut how: Completion) -> JsResult<()> {
    loop {
        let (mut frame, function, at_start) = {
            let d = data(agent, g);
            let at_start = d.state == AsyncGeneratorState::SuspendedStart;
            d.state = AsyncGeneratorState::Executing;
            (
                d.frame.take().expect("suspended async generator frame"),
                d.function,
                at_start,
            )
        };
        // Deliver the completion at the suspension point.
        let early = match how {
            Completion::Normal(v) => {
                if !at_start {
                    frame.stack.push(v);
                }
                None
            }
            Completion::Return(v) => {
                frame.stack.push(v);
                frame.pc = frame.yield_return_pc;
                None
            }
            Completion::Throw(e) => (!frame.handle_throw(e)).then_some(Err(JsError::Throw(e))),
        };
        let result = match early {
            Some(r) => r,
            None => {
                let (realm, script_or_module) = match &agent.obj(function).kind {
                    ObjectKind::Function(e) => (e.realm, e.script_or_module),
                    _ => (agent.current_realm(), None),
                };
                agent.contexts.push(ExecutionContext {
                    function: Some(function),
                    realm,
                    script_or_module,
                });
                agent.call_depth += 1;
                let r = if agent.call_depth > crate::agent::MAX_CALL_DEPTH {
                    Err(agent.throw_range_error("Maximum call stack size exceeded"))
                } else {
                    run_frame(agent, &mut frame)
                };
                agent.call_depth -= 1;
                agent.contexts.pop();
                r
            }
        };
        match result {
            // An await in the body: keep the frame and continue when the promise settles.
            Ok(FrameResult::Await(v)) => {
                data(agent, g).frame = Some(frame);
                match super::async_function::await_value(agent, g, v) {
                    Ok(()) => return Ok(()),
                    Err(JsError::Throw(e)) => {
                        how = Completion::Throw(e);
                        continue;
                    }
                    Err(e) => return Err(e),
                }
            }
            // AsyncGeneratorYield(value) (§27.6.3.8)
            Ok(FrameResult::Yield(v)) => {
                // 9. Perform AsyncGeneratorCompleteStep(generator, NormalCompletion(value), false).
                data(agent, g).frame = Some(frame);
                complete_step(agent, g, Completion::Normal(v), false)?;
                // 11-12. If the queue is not empty, continue at once with its first request.
                match data(agent, g).queue.front().copied() {
                    Some(next) => {
                        how = next.completion;
                        continue;
                    }
                    // 13. Else, set state to suspended-yield and suspend.
                    None => {
                        data(agent, g).state = AsyncGeneratorState::SuspendedYield;
                        return Ok(());
                    }
                }
            }
            Ok(FrameResult::YieldDelegate(_)) => {
                data(agent, g).state = AsyncGeneratorState::Completed;
                return Err(agent.not_implemented("yield* in async generators"));
            }
            // AsyncGeneratorStart step 4.f-k: the body finished.
            Ok(FrameResult::Return(v)) => {
                data(agent, g).state = AsyncGeneratorState::DrainingQueue;
                complete_step(agent, g, Completion::Return(v), true)?;
                return drain_queue(agent, g);
            }
            Err(JsError::Throw(e)) => {
                data(agent, g).state = AsyncGeneratorState::DrainingQueue;
                complete_step(agent, g, Completion::Throw(e), true)?;
                return drain_queue(agent, g);
            }
            Err(e) => {
                data(agent, g).state = AsyncGeneratorState::Completed;
                return Err(e);
            }
        }
    }
}

/// Whether an await-resumption target is an async generator.
pub(super) fn is_async_generator(agent: &Agent, o: Object) -> bool {
    agent.native::<AsyncGeneratorData>(o).is_some()
}
