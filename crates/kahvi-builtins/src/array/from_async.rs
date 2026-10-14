//! Array.fromAsync ( _items_ [ , _mapper_ [ , _thisArg_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-array.fromasync
//!
//! The spec body is an async function. Native code cannot suspend, so the
//! function's state lives in a native object, and each Await registers a
//! promise reaction closure that resumes the state machine (`settle`).

use kahvi_core::function::{builtin_slots, create_builtin_function};
use kahvi_core::object::array::array_create;
use kahvi_core::ops::IteratorRecord;
use kahvi_core::ops::promise::{
    PromiseCapability, new_promise_capability, perform_promise_then, promise_resolve,
};
use kahvi_core::prelude::*;
use kahvi_core::vm::create_async_from_sync_iterator;

/// 2^53 - 1
const MAX_SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;

/// The suspended state of one Array.fromAsync call.
#[derive(Debug, Clone, Copy)]
struct FromAsyncState {
    capability: PromiseCapability,
    mapper: Value,
    this_arg: Value,
    mapping: bool,
    pending: Pending,
}

/// The Await the function is suspended at.
#[derive(Debug, Clone, Copy)]
enum Pending {
    /// No Await is outstanding.
    Idle,
    /// Await(nextResult), step 8.d.iv.
    Next {
        iterator: IteratorRecord,
        array: Object,
        k: f64,
    },
    /// Await(mappedValue), step 8.d.ix.3.
    Mapped {
        iterator: IteratorRecord,
        array: Object,
        k: f64,
    },
    /// The Await of the return() result inside AsyncIteratorClose; the
    /// completion being closed for is `error`, which is always rethrown.
    Close { error: Value },
    /// Await(kValue), step 9.g.iii.
    ArrayLikeValue {
        array_like: Object,
        array: Object,
        len: f64,
        k: f64,
    },
    /// Await(mappedValue), step 9.g.iv.2.
    ArrayLikeMapped {
        array_like: Object,
        array: Object,
        len: f64,
        k: f64,
    },
}

/// Array.fromAsync
pub(crate) fn from_async(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    // This async function performs the following steps when called:
    //
    // 1. Let _ctor_ be the *this* value.
    let ctor = this;
    // (The promise for the async function is created before its body runs.)
    let promise_ctor = agent.intrinsic("%Promise%");
    let capability = new_promise_capability(agent, Value::Object(promise_ctor))?;
    match start(
        agent,
        ctor,
        args.get_or_undefined(0),
        args.get_or_undefined(1),
        args.get_or_undefined(2),
        capability,
    ) {
        Ok(()) => {}
        // An abrupt completion before the first Await rejects the promise.
        Err(JsError::Throw(e)) => reject(agent, capability, e)?,
        Err(e) => return Err(e),
    }
    Ok(capability.promise)
}

/// The body of Array.fromAsync up to its first Await (or to completion).
fn start(
    agent: &mut Agent,
    ctor: Value,
    items: Value,
    mapper: Value,
    this_arg: Value,
    capability: PromiseCapability,
) -> JsResult<()> {
    // 2. Let _mapping_ be *false*.
    // 3. If _mapper_ is not *undefined*, then
    let mapping = !mapper.is_undefined();
    // a. If IsCallable(_mapper_) is *false*, throw a *TypeError* exception.
    if mapping && !ops::is_callable(agent, mapper) {
        return Err(agent.throw_type_error("Array.fromAsync: mapper is not a function"));
    }
    // b. Set _mapping_ to *true*. (Folded into the initialisation above.)
    let state = agent.object_with_kind(
        None,
        ObjectKind::Native(Box::new(FromAsyncState {
            capability,
            mapper,
            this_arg,
            mapping,
            pending: Pending::Idle,
        })),
    );
    // 4. Let _iteratorRecord_ be *undefined*.
    // 5. Let _usingAsyncIterator_ be ? GetMethod(_items_, %Symbol.asyncIterator%).
    let async_key = PropertyKey::Symbol(agent.symbols_wk.async_iterator);
    let using_async_iterator = ops::get_method(agent, items, async_key)?;
    // 6. If _usingAsyncIterator_ is *undefined*, then
    let iterator_record = if using_async_iterator.is_undefined() {
        // a. Let _usingSyncIterator_ be ? GetMethod(_items_, %Symbol.iterator%).
        let sync_key = PropertyKey::Symbol(agent.symbols_wk.iterator);
        let using_sync_iterator = ops::get_method(agent, items, sync_key)?;
        // b. If _usingSyncIterator_ is not *undefined*, then
        if using_sync_iterator.is_undefined() {
            None
        } else {
            // i. Set _iteratorRecord_ to CreateAsyncFromSyncIterator(? GetIteratorFromMethod(_items_, _usingSyncIterator_)).
            let sync_record = ops::get_iterator_from_method(agent, items, using_sync_iterator)?;
            Some(create_async_from_sync_iterator(agent, sync_record)?)
        }
    } else {
        // 7. Else,
        // a. Set _iteratorRecord_ to ? GetIteratorFromMethod(_items_, _usingAsyncIterator_).
        Some(ops::get_iterator_from_method(
            agent,
            items,
            using_async_iterator,
        )?)
    };
    match iterator_record {
        // 8. If _iteratorRecord_ is not *undefined*, then
        Some(iterator) => {
            // a. If IsConstructor(_ctor_) is *true*, then
            let array = match ctor {
                // i. Let _array_ be ? Construct(_ctor_).
                Value::Object(c) if c.is_constructor(agent) => ops::construct(agent, c, &[], None)?,
                // b. Else, i. Let _array_ be ! ArrayCreate(0).
                _ => array_create(agent, 0.0, None)?,
            };
            // c. Let _k_ be 0.
            // d. Repeat, ...
            iter_step(agent, state, iterator, array, 0.0)
        }
        // 9. Else,
        None => {
            // a. NOTE: _items_ is neither async iterable nor iterable so assume it is an array-like object.
            // b. Let _arrayLike_ be ! ToObject(_items_).
            let array_like = ops::to_object(agent, items)?;
            // c. Let _length_ be ? LengthOfArrayLike(_arrayLike_).
            let len = ops::length_of_array_like(agent, array_like)?;
            // d. If IsConstructor(_ctor_) is *true*, then
            let array = match ctor {
                // i. Let _array_ be ? Construct(_ctor_, « 𝔽(_length_) »).
                Value::Object(c) if c.is_constructor(agent) => {
                    ops::construct(agent, c, &[Value::Number(len)], None)?
                }
                // e. Else, i. Let _array_ be ? ArrayCreate(_length_).
                _ => array_create(agent, len, None)?,
            };
            // f. Let _k_ be 0.
            // g. Repeat, while _k_ < _length_, ...
            array_like_step(agent, state, array_like, array, len, 0.0)
        }
    }
}

/// Steps 8.d.i to 8.d.iv: one turn of the iterator loop, up to its Await.
fn iter_step(
    agent: &mut Agent,
    state: Object,
    iterator: IteratorRecord,
    array: Object,
    k: f64,
) -> JsResult<()> {
    // d. Repeat,
    // i. If _k_ ≥ 2^53 - 1, then
    if k >= MAX_SAFE_INTEGER {
        // 1. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
        let error = type_error_value(agent, "Array.fromAsync: too many elements")?;
        // 2. Return ? AsyncIteratorClose(_iteratorRecord_, _error_).
        return async_iterator_close(agent, state, iterator, error);
    }
    // iii. Let _nextResult_ be ? Call(_iteratorRecord_.[[NextMethod]], _iteratorRecord_.[[Iterator]]).
    let next_result = ops::call(
        agent,
        iterator.next_method,
        Value::Object(iterator.iterator),
        &[],
    )?;
    // iv. Set _nextResult_ to ? Await(_nextResult_).
    set_pending(agent, state, Pending::Next { iterator, array, k })?;
    await_value(agent, state, next_result)
}

/// Steps 9.g.i to 9.g.ii: one turn of the array-like loop, up to its Await.
fn array_like_step(
    agent: &mut Agent,
    state: Object,
    array_like: Object,
    array: Object,
    len: f64,
    k: f64,
) -> JsResult<()> {
    // g. Repeat, while _k_ < _length_,
    if k < len {
        // i. Let _propertyKey_ be ! ToString(𝔽(_k_)).
        let key = agent.key_from_index(k);
        // ii. Let _kValue_ be ? Get(_arrayLike_, _propertyKey_).
        let k_value = ops::get(agent, array_like, key)?;
        // iii. Set _kValue_ to ? Await(_kValue_).
        set_pending(
            agent,
            state,
            Pending::ArrayLikeValue {
                array_like,
                array,
                len,
                k,
            },
        )?;
        return await_value(agent, state, k_value);
    }
    // h. Perform ? Set(_array_, *"length"*, 𝔽(_length_), *true*).
    set_length(agent, array, len)?;
    // i. Return _array_.
    resolve(agent, state, array)
}

/// Resumes the state machine with the outcome of the outstanding Await. An
/// abrupt completion that escapes the body rejects the promise.
fn step(agent: &mut Agent, state: Object, settled: Result<Value, Value>) -> JsResult<()> {
    let s = state_of(agent, state)?;
    set_pending(agent, state, Pending::Idle)?;
    match s.pending {
        Pending::Idle => Err(agent.throw_type_error("Array.fromAsync: resumed while not awaiting")),
        Pending::Next { iterator, array, k } => {
            // iv. Set _nextResult_ to ? Await(_nextResult_).
            let next_result = settled.map_err(JsError::Throw)?;
            next_fulfilled(agent, state, s, iterator, array, k, next_result)
        }
        Pending::Mapped { iterator, array, k } => match settled {
            // 3-4. IfAbruptCloseAsyncIterator(mappedValue, iteratorRecord).
            Err(e) => close_on_error(agent, state, iterator, JsError::Throw(e)),
            Ok(mapped) => {
                // xi-xiii. Define the value, then continue the loop.
                if !define_or_close(agent, state, iterator, array, k, mapped)? {
                    // The iterator is being closed; its return() result is awaited.
                    return Ok(());
                }
                iter_step(agent, state, iterator, array, k + 1.0)
            }
        },
        // Whatever the return() result was, the original completion wins.
        Pending::Close { error } => Err(JsError::Throw(error)),
        Pending::ArrayLikeValue {
            array_like,
            array,
            len,
            k,
        } => {
            // iii. Set _kValue_ to ? Await(_kValue_).
            let k_value = settled.map_err(JsError::Throw)?;
            if s.mapping {
                // iv. 1. Let _mappedValue_ be ? Call(_mapper_, _thisArg_, « _kValue_, 𝔽(_k_) »).
                let mapped = ops::call(agent, s.mapper, s.this_arg, &[k_value, Value::Number(k)])?;
                // iv. 2. Set _mappedValue_ to ? Await(_mappedValue_).
                set_pending(
                    agent,
                    state,
                    Pending::ArrayLikeMapped {
                        array_like,
                        array,
                        len,
                        k,
                    },
                )?;
                await_value(agent, state, mapped)
            } else {
                // v. Else, let _mappedValue_ be _kValue_.
                // vi. Perform ? CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_).
                define_array_like(agent, array, k, k_value)?;
                // vii. Set _k_ to _k_ + 1.
                array_like_step(agent, state, array_like, array, len, k + 1.0)
            }
        }
        Pending::ArrayLikeMapped {
            array_like,
            array,
            len,
            k,
        } => {
            // iv. 2. Set _mappedValue_ to ? Await(_mappedValue_).
            let mapped = settled.map_err(JsError::Throw)?;
            // vi. Perform ? CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_).
            define_array_like(agent, array, k, mapped)?;
            // vii. Set _k_ to _k_ + 1.
            array_like_step(agent, state, array_like, array, len, k + 1.0)
        }
    }
}

/// Steps 8.d.v to 8.d.xiii, for a fulfilled Await(_nextResult_).
fn next_fulfilled(
    agent: &mut Agent,
    state: Object,
    s: FromAsyncState,
    iterator: IteratorRecord,
    array: Object,
    k: f64,
    next_result: Value,
) -> JsResult<()> {
    // v. If _nextResult_ is not an Object, throw a *TypeError* exception.
    let Value::Object(next_result) = next_result else {
        return Err(agent.throw_type_error("Array.fromAsync: iterator result is not an object"));
    };
    // vi. Let _done_ be ? IteratorComplete(_nextResult_).
    let done = ops::iterator_complete(agent, next_result)?;
    // vii. If _done_ is *true*, then
    if done {
        // 1. Perform ? Set(_array_, *"length"*, 𝔽(_k_), *true*).
        set_length(agent, array, k)?;
        // 2. Return _array_.
        return resolve(agent, state, array);
    }
    // viii. Let _nextValue_ be ? IteratorValue(_nextResult_).
    let next_value = ops::iterator_value(agent, next_result)?;
    // ix. If _mapping_ is *true*, then
    if s.mapping {
        // 1. Let _mappedValue_ be Completion(Call(_mapper_, _thisArg_, « _nextValue_, 𝔽(_k_) »)).
        // 2. IfAbruptCloseAsyncIterator(_mappedValue_, _iteratorRecord_).
        let mapped = match ops::call(agent, s.mapper, s.this_arg, &[next_value, Value::Number(k)]) {
            Ok(m) => m,
            Err(e) => return close_on_error(agent, state, iterator, e),
        };
        // 3. Set _mappedValue_ to Completion(Await(_mappedValue_)).
        // 4. IfAbruptCloseAsyncIterator(_mappedValue_, _iteratorRecord_).
        set_pending(agent, state, Pending::Mapped { iterator, array, k })?;
        if let Err(e) = await_value(agent, state, mapped) {
            return close_on_error(agent, state, iterator, e);
        }
        return Ok(());
    }
    // x. Else, 1. Let _mappedValue_ be _nextValue_.
    // xi. Let _defineStatus_ be Completion(CreateDataPropertyOrThrow(_array_, _propertyKey_, _mappedValue_)).
    // xii. IfAbruptCloseAsyncIterator(_defineStatus_, _iteratorRecord_).
    if !define_or_close(agent, state, iterator, array, k, next_value)? {
        // The iterator is being closed; its return() result is awaited.
        return Ok(());
    }
    // xiii. Set _k_ to _k_ + 1.
    iter_step(agent, state, iterator, array, k + 1.0)
}

/// Step 8.d.xi and 8.d.xii: CreateDataPropertyOrThrow, closing the iterator on failure.
/// Returns `Ok(true)` when the loop may continue, and `Ok(false)` when the
/// iterator is being closed: the return() result is awaited and the caller
/// must stop without touching the iterator again.
fn define_or_close(
    agent: &mut Agent,
    state: Object,
    iterator: IteratorRecord,
    array: Object,
    k: f64,
    value: Value,
) -> JsResult<bool> {
    // i. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    let key = agent.key_from_index(k);
    match ops::create_data_property_or_throw(agent, array, key, value) {
        Ok(()) => Ok(true),
        Err(e) => {
            close_on_error(agent, state, iterator, e)?;
            Ok(false)
        }
    }
}

/// Step 9.g.vi: CreateDataPropertyOrThrow for an array-like element.
fn define_array_like(agent: &mut Agent, array: Object, k: f64, value: Value) -> JsResult<()> {
    // i. Let _propertyKey_ be ! ToString(𝔽(_k_)).
    let key = agent.key_from_index(k);
    ops::create_data_property_or_throw(agent, array, key, value)
}

/// IfAbruptCloseAsyncIterator(_completion_, _iteratorRecord_): a throw
/// completion closes the iterator, a non-throw failure is passed on.
fn close_on_error(
    agent: &mut Agent,
    state: Object,
    iterator: IteratorRecord,
    error: JsError,
) -> JsResult<()> {
    match error {
        JsError::Throw(e) => async_iterator_close(agent, state, iterator, e),
        other => Err(other),
    }
}

/// AsyncIteratorClose(_iteratorRecord_, _completion_) for a throw completion
/// `error`. Returns `Ok` while the return() result is awaited; the call then
/// rejects with `error`. Returns `Err` when the rejection is immediate.
fn async_iterator_close(
    agent: &mut Agent,
    state: Object,
    iterator: IteratorRecord,
    error: Value,
) -> JsResult<()> {
    // 2. Let _iterator_ be _iteratorRecord_.[[Iterator]].
    let iterator_value = Value::Object(iterator.iterator);
    // 3. Let _innerResult_ be Completion(GetMethod(_iterator_, *"return"*)).
    let key = agent.key("return");
    let return_method = match ops::get_method(agent, iterator_value, key) {
        Ok(m) => m,
        // 5. If _completion_ is a throw completion, return ? _completion_.
        // (The GetMethod failure is discarded, as the spec does.)
        Err(JsError::Throw(_)) => return Err(JsError::Throw(error)),
        Err(e) => return Err(e),
    };
    // 4. b. If _return_ is *undefined*, return ? _completion_.
    if return_method.is_undefined() {
        return Err(JsError::Throw(error));
    }
    // 4. c. Set _innerResult_ to Completion(Call(_return_, _iterator_)).
    match ops::call(agent, return_method, iterator_value, &[]) {
        // 4. d. Set _innerResult_ to Completion(Await(_innerResult_.[[Value]])).
        Ok(inner) => {
            set_pending(agent, state, Pending::Close { error })?;
            match await_value(agent, state, inner) {
                Ok(()) => Ok(()),
                Err(JsError::Throw(_)) => Err(JsError::Throw(error)),
                Err(e) => Err(e),
            }
        }
        // 5. If _completion_ is a throw completion, return ? _completion_.
        Err(JsError::Throw(_)) => Err(JsError::Throw(error)),
        Err(e) => Err(e),
    }
}

/// Await(_value_): PromiseResolve, then PerformPromiseThen with continuations
/// that resume `state`.
fn await_value(agent: &mut Agent, state: Object, value: Value) -> JsResult<()> {
    // 1. Let _asyncContext_ be the running execution context.
    // 2. Let _promise_ be ? PromiseResolve(%Promise%, _value_).
    let promise_ctor = agent.intrinsic("%Promise%");
    let promise = promise_resolve(agent, promise_ctor, value)?;
    // 3-4. Let _onFulfilled_ and _onRejected_ resume the async context with the outcome.
    let empty = PropertyKey::String(agent.names.empty);
    let on_fulfilled = create_builtin_function(
        agent,
        on_fulfilled,
        1,
        empty,
        vec![Value::Object(state)],
        None,
        None,
        None,
        false,
    );
    let on_rejected = create_builtin_function(
        agent,
        on_rejected,
        1,
        empty,
        vec![Value::Object(state)],
        None,
        None,
        None,
        false,
    );
    // 5. Perform PerformPromiseThen(_promise_, _onFulfilled_, _onRejected_).
    perform_promise_then(
        agent,
        promise,
        Value::Object(on_fulfilled),
        Value::Object(on_rejected),
        None,
    );
    Ok(())
}

/// The fulfilled reaction of an Await: resumes the function with the value.
fn on_fulfilled(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let state = active_state(agent)?;
    settle(agent, state, Ok(args.get_or_undefined(0)))
}

/// The rejected reaction of an Await: resumes the function with the reason.
fn on_rejected(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let state = active_state(agent)?;
    settle(agent, state, Err(args.get_or_undefined(0)))
}

/// Runs one step of the function after an Await and rejects the promise if
/// the body completes abruptly.
fn settle(agent: &mut Agent, state: Object, settled: Result<Value, Value>) -> JsResult<Value> {
    match step(agent, state, settled) {
        Ok(()) => Ok(Value::Undefined),
        Err(JsError::Throw(e)) => {
            let capability = state_of(agent, state)?.capability;
            reject(agent, capability, e)?;
            Ok(Value::Undefined)
        }
        Err(e) => Err(e),
    }
}

/// The state object of the reaction currently running.
fn active_state(agent: &mut Agent) -> JsResult<Object> {
    let slot = agent
        .active_function()
        .and_then(|f| builtin_slots(agent, f).first().copied());
    match slot {
        Some(Value::Object(o)) => Ok(o),
        _ => Err(agent.throw_type_error("Array.fromAsync: missing state")),
    }
}

fn state_of(agent: &mut Agent, state: Object) -> JsResult<FromAsyncState> {
    match agent.native::<FromAsyncState>(state) {
        Some(s) => Ok(*s),
        None => Err(agent.throw_type_error("Array.fromAsync: missing state")),
    }
}

fn set_pending(agent: &mut Agent, state: Object, pending: Pending) -> JsResult<()> {
    match agent.native_mut::<FromAsyncState>(state) {
        Some(s) => {
            s.pending = pending;
            Ok(())
        }
        None => Err(agent.throw_type_error("Array.fromAsync: missing state")),
    }
}

/// Performs ? Set(_array_, *"length"*, 𝔽(_len_), *true*).
fn set_length(agent: &mut Agent, array: Object, len: f64) -> JsResult<()> {
    let key = PropertyKey::String(agent.names.length);
    ops::set(agent, array, key, Value::Number(len), true)
}

/// Calls the promise's resolve function with `array`.
fn resolve(agent: &mut Agent, state: Object, array: Object) -> JsResult<()> {
    let capability = state_of(agent, state)?.capability;
    ops::call(
        agent,
        capability.resolve,
        Value::Undefined,
        &[Value::Object(array)],
    )?;
    Ok(())
}

fn reject(agent: &mut Agent, capability: PromiseCapability, reason: Value) -> JsResult<()> {
    ops::call(agent, capability.reject, Value::Undefined, &[reason])?;
    Ok(())
}

/// A newly created TypeError object with the given message.
fn type_error_value(agent: &mut Agent, message: &str) -> JsResult<Value> {
    match agent.throw_type_error(message) {
        JsError::Throw(e) => Ok(e),
        other => Err(other),
    }
}
