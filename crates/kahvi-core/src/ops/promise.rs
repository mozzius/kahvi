//! Promise abstract operations (ECMA-262 §27.2.1-27.2.2, §27.2.5.4.1).
//!
//! They live in the core because `await` needs them; the Promise built-ins
//! (constructor, then, combinators) are in kahvi-builtins and call these.

use crate::agent::{Agent, ExecutionContext, Job};
use crate::error::{JsError, JsResult};
use crate::function::{builtin_slots, create_builtin_function, get_function_realm};
use crate::object::{ObjectKind, PropertyKey};
use crate::ops::{call, construct, get, is_callable};
use crate::value::{Args, Object, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromiseState {
    Pending,
    Fulfilled,
    Rejected,
}

/// A PromiseCapability Record (§27.2.1.1).
#[derive(Debug, Clone, Copy)]
pub struct PromiseCapability {
    /// [[Promise]] (undefined for await's internal reactions).
    pub promise: Value,
    pub resolve: Value,
    pub reject: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactionType {
    Fulfill,
    Reject,
}

/// A PromiseReaction Record (§27.2.1.2).
#[derive(Debug, Clone, Copy)]
pub struct PromiseReaction {
    pub capability: Option<PromiseCapability>,
    pub kind: ReactionType,
    /// [[Handler]]: a callable, or None for empty.
    pub handler: Option<Value>,
}

/// The internal slots of a Promise instance (Table 89).
#[derive(Debug)]
pub struct PromiseData {
    pub state: PromiseState,
    pub result: Value,
    pub fulfill_reactions: Vec<PromiseReaction>,
    pub reject_reactions: Vec<PromiseReaction>,
    pub is_handled: bool,
}

impl PromiseData {
    pub fn new() -> PromiseData {
        PromiseData {
            state: PromiseState::Pending,
            result: Value::Undefined,
            fulfill_reactions: Vec::new(),
            reject_reactions: Vec::new(),
            is_handled: false,
        }
    }
}

impl Default for PromiseData {
    fn default() -> Self {
        PromiseData::new()
    }
}

/// The shared [[AlreadyResolved]] record of a pair of resolving functions.
#[derive(Debug)]
struct AlreadyResolved(bool);

/// The record GetCapabilitiesExecutor closures fill in.
#[derive(Debug, Default)]
struct CapabilityCells {
    resolve: Option<Value>,
    reject: Option<Value>,
}

/// IsPromise(x) (§27.2.1.6)
/// A settled promise's result: Ok for fulfilled, Err for rejected; None while pending.
pub fn promise_result(agent: &Agent, p: Object) -> Option<Result<Value, Value>> {
    let d = agent.native::<PromiseData>(p)?;
    match d.state {
        PromiseState::Pending => None,
        PromiseState::Fulfilled => Some(Ok(d.result)),
        PromiseState::Rejected => Some(Err(d.result)),
    }
}

pub fn is_promise(agent: &Agent, x: Value) -> bool {
    matches!(x, Value::Object(o) if agent.native::<PromiseData>(o).is_some())
}

fn data_mut(agent: &mut Agent, promise: Object) -> &mut PromiseData {
    agent
        .native_mut::<PromiseData>(promise)
        .expect("promise object")
}

/// A new pending promise object with the given prototype (the
/// OrdinaryCreateFromConstructor step of the Promise constructor).
pub fn promise_alloc(agent: &mut Agent, proto: Object) -> Object {
    agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(PromiseData::new())),
    )
}

fn active_slots(agent: &Agent) -> Vec<Value> {
    let f = agent
        .active_function()
        .expect("a built-in runs with an active function");
    builtin_slots(agent, f).to_vec()
}

// ---- resolving functions (§27.2.1.3) ----------------------------------------------------

/// CreateResolvingFunctions(promise) (§27.2.1.3): (resolve, reject).
pub fn create_resolving_functions(agent: &mut Agent, promise: Object) -> (Object, Object) {
    // 1. Let alreadyResolved be the Record { [[Value]]: false }.
    let proto = agent.intrinsic("%Object.prototype%");
    let already = agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(AlreadyResolved(false))),
    );
    let slots = vec![Value::Object(promise), Value::Object(already)];
    let empty = PropertyKey::String(agent.names.empty);
    // 2-6. resolve = CreateBuiltinFunction(resolveSteps, 1, "", « [[Promise]], [[AlreadyResolved]] »).
    let resolve = create_builtin_function(
        agent,
        promise_resolve_function,
        1,
        empty,
        slots.clone(),
        None,
        None,
        None,
        false,
    );
    // 7-11. reject = CreateBuiltinFunction(rejectSteps, 1, "", « [[Promise]], [[AlreadyResolved]] »).
    let reject = create_builtin_function(
        agent,
        promise_reject_function,
        1,
        empty,
        slots,
        None,
        None,
        None,
        false,
    );
    (resolve, reject)
}

/// Checks and sets [[AlreadyResolved]]; returns the promise if this call may proceed.
fn take_resolution(agent: &mut Agent) -> Option<Object> {
    let slots = active_slots(agent);
    let (Some(&Value::Object(promise)), Some(&Value::Object(already))) =
        (slots.first(), slots.get(1))
    else {
        return None;
    };
    let record = agent.native_mut::<AlreadyResolved>(already)?;
    // If alreadyResolved.[[Value]] is true, return undefined.
    if record.0 {
        return None;
    }
    // Set alreadyResolved.[[Value]] to true.
    record.0 = true;
    Some(promise)
}

/// Promise Resolve Functions (§27.2.1.3.2)
fn promise_resolve_function(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let resolution = args.get_or_undefined(0);
    // 1-6. The [[Promise]] slot, unless already resolved.
    let Some(promise) = take_resolution(agent) else {
        return Ok(Value::Undefined);
    };
    // 7. If SameValue(resolution, promise) is true, then
    if matches!(resolution, Value::Object(r) if r == promise) {
        // a. Let selfResolutionError be a newly created TypeError object.
        let err = agent.throw_type_error("Chaining cycle detected for promise");
        let JsError::Throw(e) = err else {
            return Err(err);
        };
        // b. Perform RejectPromise(promise, selfResolutionError).
        reject_promise(agent, promise, e);
        return Ok(Value::Undefined);
    }
    // 8. If resolution is not an Object, then perform FulfillPromise(promise, resolution).
    let Value::Object(res) = resolution else {
        fulfill_promise(agent, promise, resolution);
        return Ok(Value::Undefined);
    };
    // 9. Let then be Completion(Get(resolution, "then")).
    let then_key = PropertyKey::String(agent.names.then);
    let then = match get(agent, res, then_key) {
        Ok(t) => t,
        // 10. If then is an abrupt completion, then perform RejectPromise(promise, then.[[Value]]).
        Err(JsError::Throw(e)) => {
            reject_promise(agent, promise, e);
            return Ok(Value::Undefined);
        }
        Err(e) => return Err(e),
    };
    // 12. If IsCallable(thenAction) is false, then perform FulfillPromise(promise, resolution).
    if !is_callable(agent, then) {
        fulfill_promise(agent, promise, resolution);
        return Ok(Value::Undefined);
    }
    // 13-15. Enqueue NewPromiseResolveThenableJob(promise, resolution, thenJobCallback).
    let realm = match then {
        Value::Object(t) => get_function_realm(agent, t).unwrap_or_else(|_| agent.current_realm()),
        _ => agent.current_realm(),
    };
    agent.enqueue_job(Job::PromiseResolveThenable {
        realm,
        promise,
        thenable: resolution,
        then,
    });
    Ok(Value::Undefined)
}

/// Promise Reject Functions (§27.2.1.3.1)
fn promise_reject_function(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let reason = args.get_or_undefined(0);
    let Some(promise) = take_resolution(agent) else {
        return Ok(Value::Undefined);
    };
    // 7. Perform RejectPromise(promise, reason).
    reject_promise(agent, promise, reason);
    Ok(Value::Undefined)
}

/// FulfillPromise(promise, value) (§27.2.1.4)
pub fn fulfill_promise(agent: &mut Agent, promise: Object, value: Value) {
    let data = data_mut(agent, promise);
    // 2. Let reactions be promise.[[PromiseFulfillReactions]].
    let reactions = std::mem::take(&mut data.fulfill_reactions);
    // 3-6. Set [[PromiseResult]], clear the reaction lists, set [[PromiseState]] to fulfilled.
    data.result = value;
    data.reject_reactions.clear();
    data.state = PromiseState::Fulfilled;
    // 7. Perform TriggerPromiseReactions(reactions, value).
    trigger_promise_reactions(agent, reactions, value);
}

/// RejectPromise(promise, reason) (§27.2.1.7)
pub fn reject_promise(agent: &mut Agent, promise: Object, reason: Value) {
    let data = data_mut(agent, promise);
    let reactions = std::mem::take(&mut data.reject_reactions);
    data.result = reason;
    data.fulfill_reactions.clear();
    data.state = PromiseState::Rejected;
    let handled = data.is_handled;
    // 7. If promise.[[PromiseIsHandled]] is false, perform HostPromiseRejectionTracker(promise, "reject").
    if !handled {
        host_promise_rejection_tracker(agent, promise, false);
    }
    // 8. Perform TriggerPromiseReactions(reactions, reason).
    trigger_promise_reactions(agent, reactions, reason);
}

/// TriggerPromiseReactions(reactions, argument) (§27.2.1.8)
fn trigger_promise_reactions(agent: &mut Agent, reactions: Vec<PromiseReaction>, argument: Value) {
    for reaction in reactions {
        new_promise_reaction_job(agent, reaction, argument);
    }
}

/// HostPromiseRejectionTracker(promise, operation) (§27.2.1.9): `handled`
/// is true for "handle". The host reads `Agent::rejection_tracker`.
fn host_promise_rejection_tracker(agent: &mut Agent, promise: Object, handled: bool) {
    agent.rejection_tracker.push((promise, handled));
}

/// NewPromiseReactionJob(reaction, argument) (§27.2.2.1), enqueued with
/// HostEnqueuePromiseJob.
fn new_promise_reaction_job(agent: &mut Agent, reaction: PromiseReaction, argument: Value) {
    // 3-5. The job's realm is the handler's function realm if it can be found.
    let realm = match reaction.handler {
        Some(Value::Object(h)) => {
            get_function_realm(agent, h).unwrap_or_else(|_| agent.current_realm())
        }
        _ => agent.current_realm(),
    };
    agent.enqueue_job(Job::PromiseReaction {
        realm,
        reaction,
        argument,
    });
}

/// Runs a job (HostEnqueuePromiseJob: in an execution context for its realm).
pub(crate) fn run_job(agent: &mut Agent, job: Job) -> JsResult<()> {
    let realm = match job {
        Job::PromiseReaction { realm, .. } | Job::PromiseResolveThenable { realm, .. } => realm,
    };
    agent.contexts.push(ExecutionContext {
        function: None,
        realm,
        script_or_module: None,
    });
    let r = match job {
        Job::PromiseReaction {
            reaction, argument, ..
        } => promise_reaction_job(agent, reaction, argument),
        Job::PromiseResolveThenable {
            promise,
            thenable,
            then,
            ..
        } => promise_resolve_thenable_job(agent, promise, thenable, then),
    };
    agent.contexts.pop();
    r
}

/// The Job Abstract Closure of NewPromiseReactionJob (§27.2.2.1 step 1).
fn promise_reaction_job(
    agent: &mut Agent,
    reaction: PromiseReaction,
    argument: Value,
) -> JsResult<()> {
    // 1.a-c Let handlerResult be the handler's completion (or argument when the handler is empty).
    let handler_result = match reaction.handler {
        None => match reaction.kind {
            ReactionType::Fulfill => Ok(argument),
            ReactionType::Reject => Err(JsError::Throw(argument)),
        },
        Some(h) => call(agent, h, Value::Undefined, &[argument]),
    };
    // 1.f If promiseCapability is undefined, return empty.
    let Some(capability) = reaction.capability else {
        return match handler_result {
            Err(JsError::NotImplemented(m)) => Err(JsError::NotImplemented(m)),
            _ => Ok(()),
        };
    };
    match handler_result {
        // 1.h Else, return ? Call(promiseCapability.[[Resolve]], undefined, « handlerResult.[[Value]] »).
        Ok(v) => call(agent, capability.resolve, Value::Undefined, &[v]).map(|_| ()),
        // 1.g If handlerResult is an abrupt completion, return ? Call(promiseCapability.[[Reject]], ...).
        Err(JsError::Throw(e)) => {
            call(agent, capability.reject, Value::Undefined, &[e]).map(|_| ())
        }
        Err(e) => Err(e),
    }
}

/// The Job Abstract Closure of NewPromiseResolveThenableJob (§27.2.2.2 step 1).
fn promise_resolve_thenable_job(
    agent: &mut Agent,
    promise: Object,
    thenable: Value,
    then: Value,
) -> JsResult<()> {
    // a. Let resolvingFunctions be CreateResolvingFunctions(promiseToResolve).
    let (resolve, reject) = create_resolving_functions(agent, promise);
    // b. Let thenCallResult be Completion(HostCallJobCallback(then, thenable, « resolve, reject »)).
    match call(
        agent,
        then,
        thenable,
        &[Value::Object(resolve), Value::Object(reject)],
    ) {
        Ok(_) => Ok(()),
        // c. If thenCallResult is an abrupt completion, then return ? Call(reject, undefined, « thenCallResult.[[Value]] »).
        Err(JsError::Throw(e)) => {
            call(agent, Value::Object(reject), Value::Undefined, &[e]).map(|_| ())
        }
        Err(e) => Err(e),
    }
}

// ---- capabilities and then (§27.2.1.5, §27.2.5.4.1) ----------------------------------------

/// GetCapabilitiesExecutor Functions (§27.2.1.5 steps 4-5)
fn capabilities_executor(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let slots = active_slots(agent);
    let Some(&Value::Object(cells)) = slots.first() else {
        return Ok(Value::Undefined);
    };
    let (resolve, reject) = (args.get_or_undefined(0), args.get_or_undefined(1));
    let record = agent
        .native_mut::<CapabilityCells>(cells)
        .expect("capability record");
    // a. If promiseCapability.[[Resolve]] is not undefined, throw a TypeError exception.
    // b. If promiseCapability.[[Reject]] is not undefined, throw a TypeError exception.
    if record.resolve.is_some_and(|r| !r.is_undefined())
        || record.reject.is_some_and(|r| !r.is_undefined())
    {
        return Err(agent.throw_type_error(
            "Promise executor has already been invoked with non-undefined arguments",
        ));
    }
    // c-d. Set promiseCapability.[[Resolve]] to resolve and [[Reject]] to reject.
    record.resolve = Some(resolve);
    record.reject = Some(reject);
    Ok(Value::Undefined)
}

/// NewPromiseCapability(C) (§27.2.1.5)
pub fn new_promise_capability(agent: &mut Agent, c: Value) -> JsResult<PromiseCapability> {
    // 1. If IsConstructor(C) is false, throw a TypeError exception.
    let c = match c {
        Value::Object(o) if o.is_constructor(agent) => o,
        _ => {
            return Err(
                agent.throw_type_error("Promise capability constructor is not a constructor")
            );
        }
    };
    // 3. Let resolvingFunctions be the Record { [[Resolve]]: undefined, [[Reject]]: undefined }.
    let proto = agent.intrinsic("%Object.prototype%");
    let cells = agent.object_with_kind(
        Some(proto),
        ObjectKind::Native(Box::new(CapabilityCells::default())),
    );
    // 4-5. executor = CreateBuiltinFunction(executorClosure, 2, "", « »).
    let empty = PropertyKey::String(agent.names.empty);
    let executor = create_builtin_function(
        agent,
        capabilities_executor,
        2,
        empty,
        vec![Value::Object(cells)],
        None,
        None,
        None,
        false,
    );
    // 6. Let promise be ? Construct(C, « executor »).
    let promise = construct(agent, c, &[Value::Object(executor)], None)?;
    let record = agent
        .native::<CapabilityCells>(cells)
        .expect("capability record");
    let (resolve, reject) = (
        record.resolve.unwrap_or_default(),
        record.reject.unwrap_or_default(),
    );
    // 7. If IsCallable(resolvingFunctions.[[Resolve]]) is false, throw a TypeError exception.
    // 8. If IsCallable(resolvingFunctions.[[Reject]]) is false, throw a TypeError exception.
    if !is_callable(agent, resolve) || !is_callable(agent, reject) {
        return Err(agent.throw_type_error("Promise resolve or reject function is not callable"));
    }
    // 9. Return the PromiseCapability Record { [[Promise]]: promise, [[Resolve]], [[Reject]] }.
    Ok(PromiseCapability {
        promise: Value::Object(promise),
        resolve,
        reject,
    })
}

/// PerformPromiseThen(promise, onFulfilled, onRejected [, resultCapability]) (§27.2.5.4.1)
pub fn perform_promise_then(
    agent: &mut Agent,
    promise: Object,
    on_fulfilled: Value,
    on_rejected: Value,
    result_capability: Option<PromiseCapability>,
) -> Value {
    // 3-6. Non-callable handlers become empty.
    let on_fulfilled = is_callable(agent, on_fulfilled).then_some(on_fulfilled);
    let on_rejected = is_callable(agent, on_rejected).then_some(on_rejected);
    // 7-8. The two PromiseReaction Records.
    let fulfill = PromiseReaction {
        capability: result_capability,
        kind: ReactionType::Fulfill,
        handler: on_fulfilled,
    };
    let reject = PromiseReaction {
        capability: result_capability,
        kind: ReactionType::Reject,
        handler: on_rejected,
    };
    let data = data_mut(agent, promise);
    match data.state {
        // 9. If promise.[[PromiseState]] is pending, append the reactions.
        PromiseState::Pending => {
            data.fulfill_reactions.push(fulfill);
            data.reject_reactions.push(reject);
            data.is_handled = true;
        }
        // 10. Else if fulfilled, enqueue NewPromiseReactionJob(fulfillReaction, value).
        PromiseState::Fulfilled => {
            let value = data.result;
            data.is_handled = true;
            new_promise_reaction_job(agent, fulfill, value);
        }
        // 11. Else (rejected): track the newly handled rejection, then enqueue.
        PromiseState::Rejected => {
            let reason = data.result;
            let was_handled = data.is_handled;
            data.is_handled = true;
            if !was_handled {
                host_promise_rejection_tracker(agent, promise, true);
            }
            new_promise_reaction_job(agent, reject, reason);
        }
    }
    // 13-14. Return resultCapability.[[Promise]], or undefined.
    result_capability
        .map(|c| c.promise)
        .unwrap_or(Value::Undefined)
}

/// PromiseResolve(C, x) (§27.2.4.7.1)
pub fn promise_resolve(agent: &mut Agent, c: Object, x: Value) -> JsResult<Object> {
    // 1. If IsPromise(x) is true, then
    if let Value::Object(xo) = x
        && is_promise(agent, x)
    {
        // a. Let xConstructor be ? Get(x, "constructor").
        let key = PropertyKey::String(agent.names.constructor);
        let x_constructor = get(agent, xo, key)?;
        // b. If SameValue(xConstructor, C) is true, return x.
        if matches!(x_constructor, Value::Object(xc) if xc == c) {
            return Ok(xo);
        }
    }
    // 2. Let promiseCapability be ? NewPromiseCapability(C).
    let capability = new_promise_capability(agent, Value::Object(c))?;
    // 3. Perform ? Call(promiseCapability.[[Resolve]], undefined, « x »).
    call(agent, capability.resolve, Value::Undefined, &[x])?;
    // 4. Return promiseCapability.[[Promise]].
    match capability.promise {
        Value::Object(p) => Ok(p),
        _ => Err(agent.throw_type_error("Promise capability has no promise")),
    }
}

/// The state and result of a promise (for hosts and tests).
pub fn promise_state(agent: &Agent, promise: Object) -> Option<(PromiseState, Value)> {
    agent
        .native::<PromiseData>(promise)
        .map(|d| (d.state, d.result))
}

// ---- garbage collection ----------------------------------------------------

use crate::gc::{Trace, Tracer};

impl Trace for PromiseCapability {
    fn trace(&self, t: &mut Tracer) {
        t.value(self.promise);
        t.value(self.resolve);
        t.value(self.reject);
    }
}

impl Trace for PromiseReaction {
    fn trace(&self, t: &mut Tracer) {
        self.capability.trace(t);
        self.handler.trace(t);
    }
}

impl Trace for PromiseData {
    fn trace(&self, t: &mut Tracer) {
        t.value(self.result);
        self.fulfill_reactions.trace(t);
        self.reject_reactions.trace(t);
    }
}

impl Trace for AlreadyResolved {
    fn trace(&self, _t: &mut Tracer) {}
}

impl Trace for CapabilityCells {
    fn trace(&self, t: &mut Tracer) {
        self.resolve.trace(t);
        self.reject.trace(t);
    }
}
