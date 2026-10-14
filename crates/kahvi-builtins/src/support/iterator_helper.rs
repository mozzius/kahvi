//! Iterator helper objects (§27.1.2.1): CreateIteratorFromClosure for the
//! Iterator.prototype methods and Iterator.concat/zip, whose spec steps are
//! Abstract Closures that Yield.
//!
//! Each such method implements [`HelperClosure`]: `resume` runs the
//! closure's steps from the start or the last Yield up to the next Yield
//! (returning the yielded value) or to the end (returning `None`), keeping
//! whatever local state the steps need in the struct between calls.
#![allow(dead_code)] // helpers for the Iterator tickets

use kahvi_core::ops::IteratorRecord;
use kahvi_core::prelude::*;

/// The steps of an Abstract Closure given to CreateIteratorFromClosure.
pub(crate) trait HelperClosure: std::fmt::Debug + Send + kahvi_core::gc::Trace {
    /// Runs to the next Yield (`Some(value)`) or to a return (`None`).
    /// `underlying` is [[UnderlyingIterators]].
    fn resume(
        &mut self,
        agent: &mut Agent,
        underlying: &mut [IteratorRecord],
    ) -> JsResult<Option<Value>>;

    /// The closure was suspended at a Yield and resumed with a return
    /// completion: the steps that follow that Yield. Most closures do
    /// IfAbruptCloseIterator(completion, iterated), which this default does
    /// for every underlying iterator (IteratorCloseAll).
    fn on_return(&mut self, agent: &mut Agent, underlying: &mut [IteratorRecord]) -> JsResult<()> {
        iterator_close_all(agent, underlying, Ok(()))
    }
}

/// [[GeneratorState]]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    SuspendedStart,
    SuspendedYield,
    Executing,
    Completed,
}

/// An Iterator Helper object's slots: [[GeneratorState]], [[UnderlyingIterators]] and the closure.
#[derive(Debug)]
pub(crate) struct IteratorHelper {
    state: State,
    underlying: Vec<IteratorRecord>,
    closure: Option<Box<dyn HelperClosure>>,
}

/// CreateIteratorFromClosure(closure, "Iterator Helper", %IteratorHelperPrototype%, « [[UnderlyingIterators]] »),
/// with [[UnderlyingIterators]] set to `underlying`.
pub(crate) fn create_iterator_helper(
    agent: &mut Agent,
    closure: Box<dyn HelperClosure>,
    underlying: Vec<IteratorRecord>,
) -> Object {
    let proto = agent.intrinsic("%IteratorHelperPrototype%");
    agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(IteratorHelper {
            state: State::SuspendedStart,
            underlying,
            closure: Some(closure),
        })),
    )
}

/// IfAbruptCloseIterator(value, iteratorRecord) (§7.4.13)
pub(crate) fn if_abrupt_close_iterator<T>(
    agent: &mut Agent,
    value: JsResult<T>,
    record: &IteratorRecord,
) -> JsResult<T> {
    match value {
        Ok(v) => Ok(v),
        // If value is an abrupt completion, return ? IteratorClose(iteratorRecord, value).
        Err(e) => ops::iterator_close(agent, record, Err(e)),
    }
}

/// IteratorCloseAll(iters, completion) (§7.4.12): closes in reverse order.
pub(crate) fn iterator_close_all(
    agent: &mut Agent,
    iters: &[IteratorRecord],
    completion: JsResult<()>,
) -> JsResult<()> {
    let mut completion = completion;
    for record in iters.iter().rev() {
        completion = ops::iterator_close(agent, record, completion);
    }
    completion
}

/// GeneratorValidate(generator, "Iterator Helper") (§27.5.3.2): the helper's state.
fn validate(agent: &mut Agent, this: Value) -> JsResult<(Object, State)> {
    let o = match this {
        Value::Object(o) => o,
        _ => return Err(agent.throw_type_error("not an Iterator Helper")),
    };
    let Some(helper) = agent.native::<IteratorHelper>(o) else {
        return Err(agent.throw_type_error("not an Iterator Helper"));
    };
    let state = helper.state;
    // If state is executing, throw a TypeError exception.
    if state == State::Executing {
        return Err(agent.throw_type_error("Iterator Helper is already running"));
    }
    Ok((o, state))
}

fn helper_mut(agent: &mut Agent, o: Object) -> &mut IteratorHelper {
    agent
        .native_mut::<IteratorHelper>(o)
        .expect("an Iterator Helper")
}

/// GeneratorResume(generator, undefined, "Iterator Helper") (§27.5.3.3)
pub(crate) fn helper_resume(agent: &mut Agent, this: Value) -> JsResult<Value> {
    // 1. Let state be ? GeneratorValidate(generator, generatorBrand).
    let (o, state) = validate(agent, this)?;
    // 2. If state is completed, return CreateIteratorResultObject(undefined, true).
    if state == State::Completed {
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    }
    // 4-9. Run the closure (state executing) until it yields, returns or throws.
    let h = helper_mut(agent, o);
    h.state = State::Executing;
    let mut closure = h
        .closure
        .take()
        .expect("a suspended helper has its closure");
    let mut underlying = std::mem::take(&mut h.underlying);
    let result = closure.resume(agent, &mut underlying);
    let h = helper_mut(agent, o);
    h.underlying = underlying;
    h.closure = Some(closure);
    match result {
        // Yield(value): suspended-yield, and the iterator result is { value, done: false }.
        Ok(Some(v)) => {
            h.state = State::SuspendedYield;
            Ok(Value::Object(ops::create_iter_result_object(
                agent, v, false,
            )))
        }
        // The closure returned: completed, { undefined, done: true }.
        Ok(None) => {
            h.state = State::Completed;
            Ok(Value::Object(ops::create_iter_result_object(
                agent,
                Value::Undefined,
                true,
            )))
        }
        // The closure threw: completed, and the throw propagates.
        Err(e) => {
            h.state = State::Completed;
            Err(e)
        }
    }
}

/// %IteratorHelperPrototype%.return steps 2-6: the helper is closed with a return completion.
pub(crate) fn helper_return(agent: &mut Agent, this: Value) -> JsResult<Value> {
    // 2. Perform ? RequireInternalSlot(O, [[UnderlyingIterators]]).
    let (o, state) = validate(agent, this)?;
    // 4. If O.[[GeneratorState]] is suspended-start, then
    if state == State::SuspendedStart {
        // a. Set O.[[GeneratorState]] to completed.
        let h = helper_mut(agent, o);
        h.state = State::Completed;
        // c. Perform ? IteratorCloseAll(O.[[UnderlyingIterators]], NormalCompletion(unused)).
        let underlying = h.underlying.clone();
        iterator_close_all(agent, &underlying, Ok(()))?;
        // d. Return CreateIteratorResultObject(undefined, true).
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    }
    // 5-6. Return ? GeneratorResumeAbrupt(O, ReturnCompletion(undefined), "Iterator Helper").
    if state == State::Completed {
        // GeneratorResumeAbrupt step 3: a completed generator returns { value, done: true }.
        return Ok(Value::Object(ops::create_iter_result_object(
            agent,
            Value::Undefined,
            true,
        )));
    }
    // Resume the closure at its Yield with the return completion.
    let h = helper_mut(agent, o);
    h.state = State::Executing;
    let mut closure = h
        .closure
        .take()
        .expect("a suspended helper has its closure");
    let mut underlying = std::mem::take(&mut h.underlying);
    let result = closure.on_return(agent, &mut underlying);
    let h = helper_mut(agent, o);
    h.underlying = underlying;
    h.closure = Some(closure);
    h.state = State::Completed;
    // The return completion ends the closure: { undefined, done: true }, unless closing threw.
    result?;
    Ok(Value::Object(ops::create_iter_result_object(
        agent,
        Value::Undefined,
        true,
    )))
}

/// A %WrapForValidIteratorPrototype% object's [[Iterated]] slot (Iterator.from).
#[derive(Debug)]
pub(crate) struct WrapForValidIterator {
    pub iterated: IteratorRecord,
}

/// OrdinaryObjectCreate(%WrapForValidIteratorPrototype%, « [[Iterated]] ») with [[Iterated]] set.
pub(crate) fn create_wrap_for_valid_iterator(
    agent: &mut Agent,
    iterated: IteratorRecord,
) -> Object {
    let proto = agent.intrinsic("%WrapForValidIteratorPrototype%");
    agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(WrapForValidIterator { iterated })),
    )
}

/// RequireInternalSlot(O, [[Iterated]]): O.[[Iterated]].
pub(crate) fn wrapped_iterated(agent: &mut Agent, o: Value) -> JsResult<IteratorRecord> {
    match o {
        Value::Object(o) => match agent.native::<WrapForValidIterator>(o) {
            Some(w) => Ok(w.iterated),
            None => Err(agent.throw_type_error("not a wrapped iterator")),
        },
        _ => Err(agent.throw_type_error("not a wrapped iterator")),
    }
}

// ---- garbage collection ----------------------------------------------------

impl kahvi_core::gc::Trace for IteratorHelper {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        self.underlying.trace(t);
        self.closure.trace(t);
    }
}

impl kahvi_core::gc::Trace for WrapForValidIterator {
    fn trace(&self, t: &mut kahvi_core::gc::Tracer) {
        self.iterated.trace(t);
    }
}
