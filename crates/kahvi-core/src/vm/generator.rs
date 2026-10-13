//! Generator objects (§27.5): a generator holds its suspended frame.
//!
//! The %GeneratorPrototype% methods (next/return/throw) live in the
//! builtins crate and call [`generator_resume`] / [`generator_resume_abrupt`].

use super::{Frame, FrameResult, run_frame};
use crate::agent::{Agent, ExecutionContext};
use crate::error::{JsError, JsResult};
use crate::object::ObjectKind;
use crate::ops;
use crate::value::{Object, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratorState {
    SuspendedStart,
    SuspendedYield,
    Executing,
    Completed,
}

/// The internal slots of a generator object.
#[derive(Debug)]
pub struct GeneratorData {
    /// [[GeneratorState]]
    pub state: GeneratorState,
    /// The suspended [[GeneratorContext]]; `None` while executing or completed.
    pub frame: Option<Box<Frame>>,
    /// The generator function (the context's Function).
    pub function: Object,
}

/// EvaluateGeneratorBody steps 2-4: G = OrdinaryCreateFromConstructor(F,
/// "%GeneratorPrototype%"), then GeneratorStart.
pub(super) fn create_generator(agent: &mut Agent, f: Object, frame: Frame) -> JsResult<Object> {
    let data = GeneratorData {
        state: GeneratorState::SuspendedStart,
        frame: Some(Box::new(frame)),
        function: f,
    };
    ops::ordinary_create_from_constructor(
        agent,
        f,
        "%GeneratorPrototype%",
        ObjectKind::Native(Box::new(data)),
    )
}

/// GeneratorValidate(generator, brand) (§27.5.3.2): the state, after
/// checking the receiver is a generator that is not running.
fn generator_validate(agent: &mut Agent, generator: Value) -> JsResult<(Object, GeneratorState)> {
    let Value::Object(g) = generator else {
        return Err(agent.throw_type_error("not a generator object"));
    };
    let Some(data) = agent.native::<GeneratorData>(g) else {
        return Err(agent.throw_type_error("not a generator object"));
    };
    let state = data.state;
    if state == GeneratorState::Executing {
        return Err(agent.throw_type_error("generator is already running"));
    }
    Ok((g, state))
}

fn data_mut(agent: &mut Agent, g: Object) -> &mut GeneratorData {
    agent
        .native_mut::<GeneratorData>(g)
        .expect("generator object")
}

/// How a suspended generator is resumed.
enum Resumption {
    Next(Value),
    Return(Value),
    Throw(Value),
}

/// Runs the generator's frame from its suspension point.
fn resume(agent: &mut Agent, g: Object, how: Resumption) -> JsResult<Value> {
    let (mut frame, function) = {
        let data = data_mut(agent, g);
        data.state = GeneratorState::Executing;
        (
            data.frame.take().expect("suspended generator frame"),
            data.function,
        )
    };
    let started = frame.pc > 0;
    if frame.delegating {
        // Suspended in yield*: the step re-runs and forwards the completion.
        frame.delegating = false;
        let (v, kind) = match how {
            Resumption::Next(v) => (v, 0),
            Resumption::Throw(v) => (v, 1),
            Resumption::Return(v) => (v, 2),
        };
        frame.stack.push(v);
        frame.delegate_kind = kind;
    } else {
        match how {
            Resumption::Next(v) => {
                if started && frame.code.ops[frame.pc - 1] != crate::compiler::Op::GeneratorStart {
                    frame.stack.push(v);
                }
            }
            Resumption::Return(v) => {
                frame.stack.push(v);
                frame.pc = frame.yield_return_pc;
            }
            Resumption::Throw(v) => {
                if !frame.handle_throw(v) {
                    let data = data_mut(agent, g);
                    data.state = GeneratorState::Completed;
                    return Err(JsError::Throw(v));
                }
            }
        }
    }
    // Push genContext onto the execution context stack.
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
    let result = if agent.call_depth > crate::agent::MAX_CALL_DEPTH {
        Err(agent.throw_range_error("Maximum call stack size exceeded"))
    } else {
        run_frame(agent, &mut frame)
    };
    agent.call_depth -= 1;
    agent.contexts.pop();
    match result {
        // yield* hands the inner result object through unchanged.
        Ok(FrameResult::YieldDelegate(r)) => {
            let data = data_mut(agent, g);
            data.state = GeneratorState::SuspendedYield;
            data.frame = Some(frame);
            Ok(r)
        }
        Ok(FrameResult::Yield(v)) => {
            let data = data_mut(agent, g);
            data.state = GeneratorState::SuspendedYield;
            data.frame = Some(frame);
            Ok(Value::Object(ops::create_iter_result_object(
                agent, v, false,
            )))
        }
        Ok(FrameResult::Return(v)) => {
            data_mut(agent, g).state = GeneratorState::Completed;
            Ok(Value::Object(ops::create_iter_result_object(
                agent, v, true,
            )))
        }
        Ok(FrameResult::Await(_)) => {
            data_mut(agent, g).state = GeneratorState::Completed;
            Err(agent.not_implemented("await in a generator"))
        }
        Err(e) => {
            data_mut(agent, g).state = GeneratorState::Completed;
            Err(e)
        }
    }
}

/// GeneratorResume(generator, value, brand) (§27.5.3.3)
pub fn generator_resume(agent: &mut Agent, generator: Value, value: Value) -> JsResult<Value> {
    let (g, state) = generator_validate(agent, generator)?;
    // 2. If state is completed, return CreateIteratorResultObject(undefined, true).
    if state == GeneratorState::Completed {
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    }
    resume(agent, g, Resumption::Next(value))
}

/// GeneratorResumeAbrupt(generator, abruptCompletion, brand) (§27.5.3.4).
/// `is_return` selects a return completion, otherwise a throw completion.
pub fn generator_resume_abrupt(
    agent: &mut Agent,
    generator: Value,
    value: Value,
    is_return: bool,
) -> JsResult<Value> {
    let (g, mut state) = generator_validate(agent, generator)?;
    // 2. If state is suspended-start, then set state to completed.
    if state == GeneratorState::SuspendedStart {
        let data = data_mut(agent, g);
        data.state = GeneratorState::Completed;
        data.frame = None;
        state = GeneratorState::Completed;
    }
    // 3. If state is completed, then
    if state == GeneratorState::Completed {
        // a. If abruptCompletion is a return completion, return CreateIteratorResultObject(value, true).
        if is_return {
            return Ok(Value::Object(ops::create_iter_result_object(
                agent, value, true,
            )));
        }
        // b. Return ? abruptCompletion.
        return Err(JsError::Throw(value));
    }
    resume(
        agent,
        g,
        if is_return {
            Resumption::Return(value)
        } else {
            Resumption::Throw(value)
        },
    )
}
