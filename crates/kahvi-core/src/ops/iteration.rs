//! Operations on iterator objects (ECMA-262 §7.4).

use crate::agent::Agent;
use crate::error::{JsError, JsResult};
use crate::object::PropertyKey;
use crate::ops::{call, call_rooted, create_data_property_or_throw, get, get_method, to_boolean};
use crate::value::{Object, Value};

/// An Iterator Record (§7.4.1).
#[derive(Debug, Clone, Copy)]
pub struct IteratorRecord {
    pub iterator: Object,
    pub next_method: Value,
    pub done: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IteratorKind {
    Sync,
    Async,
}

/// GetIteratorDirect(obj) (§7.4.2)
pub fn get_iterator_direct(agent: &mut Agent, obj: Object) -> JsResult<IteratorRecord> {
    // 1. Let nextMethod be ? Get(obj, "next").
    let next_method = get(agent, obj, PropertyKey::String(agent.names.next))?;
    // 2-3. Return the Iterator Record { [[Iterator]]: obj, [[NextMethod]]: nextMethod, [[Done]]: false }.
    Ok(IteratorRecord {
        iterator: obj,
        next_method,
        done: false,
    })
}

/// GetIteratorFromMethod(obj, method) (§7.4.3)
pub fn get_iterator_from_method(
    agent: &mut Agent,
    obj: Value,
    method: Value,
) -> JsResult<IteratorRecord> {
    // 1. Let iterator be ? Call(method, obj).
    let iterator = call(agent, method, obj, &[])?;
    // 2. If iterator is not an Object, throw a TypeError exception.
    let Value::Object(iterator) = iterator else {
        return Err(agent.throw_type_error("Result of the Symbol.iterator method is not an object"));
    };
    // 3. Return ? GetIteratorDirect(iterator).
    get_iterator_direct(agent, iterator)
}

/// GetIterator(obj, kind) (§7.4.4)
pub fn get_iterator(agent: &mut Agent, obj: Value, kind: IteratorKind) -> JsResult<IteratorRecord> {
    let method = if kind == IteratorKind::Async {
        // 1.a Let method be ? GetMethod(obj, %Symbol.asyncIterator%).
        let m = get_method(
            agent,
            obj,
            PropertyKey::Symbol(agent.symbols_wk.async_iterator),
        )?;
        // 1.b If method is undefined, then use the sync iterator and CreateAsyncFromSyncIterator.
        if m.is_undefined() {
            let sync_method =
                get_method(agent, obj, PropertyKey::Symbol(agent.symbols_wk.iterator))?;
            if sync_method.is_undefined() {
                return Err(agent.throw_type_error("object is not async iterable"));
            }
            let sync = get_iterator_from_method(agent, obj, sync_method)?;
            return crate::vm::create_async_from_sync_iterator(agent, sync);
        }
        m
    } else {
        // 2. Else, let method be ? GetMethod(obj, %Symbol.iterator%).
        get_method(agent, obj, PropertyKey::Symbol(agent.symbols_wk.iterator))?
    };
    // 3. If method is undefined, throw a TypeError exception.
    if method.is_undefined() {
        return Err(agent.throw_type_error("object is not iterable"));
    }
    // 4. Return ? GetIteratorFromMethod(obj, method).
    get_iterator_from_method(agent, obj, method)
}

/// IteratorNext(iteratorRecord [, value]) (§7.4.7)
pub fn iterator_next(
    agent: &mut Agent,
    record: &mut IteratorRecord,
    value: Option<Value>,
) -> JsResult<Object> {
    iterator_next_impl(agent, record, value, false)
}

/// `rooted`: the caller keeps the record (and `value`) rooted, so `next()`
/// runs with `call_rooted` and may collect garbage (docs/GC.md).
fn iterator_next_impl(
    agent: &mut Agent,
    record: &mut IteratorRecord,
    value: Option<Value>,
    rooted: bool,
) -> JsResult<Object> {
    // 1-2. Let result be Completion(Call(nextMethod, iterator [, « value »])).
    let args: Vec<Value> = value.into_iter().collect();
    let call = if rooted { call_rooted } else { call };
    let result = match call(
        agent,
        record.next_method,
        Value::Object(record.iterator),
        &args,
    ) {
        Ok(r) => r,
        Err(e) => {
            // 3. If result is a throw completion, set iteratorRecord.[[Done]] to true.
            record.done = true;
            return Err(e);
        }
    };
    // 5. If result is not an Object, then set [[Done]] to true and throw a TypeError exception.
    match result {
        Value::Object(o) => Ok(o),
        _ => {
            record.done = true;
            Err(agent.throw_type_error("iterator result is not an object"))
        }
    }
}

/// IteratorComplete(iteratorResult) (§7.4.8)
pub fn iterator_complete(agent: &mut Agent, result: Object) -> JsResult<bool> {
    // 1. Return ToBoolean(? Get(iteratorResult, "done")).
    let done = get(agent, result, PropertyKey::String(agent.names.done))?;
    Ok(to_boolean(agent, done))
}

/// IteratorValue(iteratorResult) (§7.4.9)
pub fn iterator_value(agent: &mut Agent, result: Object) -> JsResult<Value> {
    // 1. Return ? Get(iteratorResult, "value").
    get(agent, result, PropertyKey::String(agent.names.value))
}

/// IteratorStep(iteratorRecord) (§7.4.10): the result object, or None when done.
pub fn iterator_step(agent: &mut Agent, record: &mut IteratorRecord) -> JsResult<Option<Object>> {
    iterator_step_impl(agent, record, false)
}

fn iterator_step_impl(
    agent: &mut Agent,
    record: &mut IteratorRecord,
    rooted: bool,
) -> JsResult<Option<Object>> {
    // 1. Let result be ? IteratorNext(iteratorRecord).
    let result = iterator_next_impl(agent, record, None, rooted)?;
    // 2. Let done be Completion(IteratorComplete(result)).
    let done = match iterator_complete(agent, result) {
        Ok(d) => d,
        Err(e) => {
            // 3. If done is a throw completion, set iteratorRecord.[[Done]] to true.
            record.done = true;
            return Err(e);
        }
    };
    // 5. If done is true, then set [[Done]] to true and return done.
    if done {
        record.done = true;
        return Ok(None);
    }
    // 6. Return result.
    Ok(Some(result))
}

/// IteratorStepValue(iteratorRecord) (§7.4.11): the next value, or None when done.
pub fn iterator_step_value(
    agent: &mut Agent,
    record: &mut IteratorRecord,
) -> JsResult<Option<Value>> {
    iterator_step_value_impl(agent, record, false)
}

/// IteratorStepValue for a caller that keeps the iterator record rooted (the
/// interpreter's iterator stack, a built-in's explicit roots): `next()` may
/// collect garbage, so a generator driven by `for-of` collects as it runs.
pub fn iterator_step_value_rooted(
    agent: &mut Agent,
    record: &mut IteratorRecord,
) -> JsResult<Option<Value>> {
    iterator_step_value_impl(agent, record, true)
}

fn iterator_step_value_impl(
    agent: &mut Agent,
    record: &mut IteratorRecord,
    rooted: bool,
) -> JsResult<Option<Value>> {
    // 1. Let result be ? IteratorStep(iteratorRecord).
    let Some(result) = iterator_step_impl(agent, record, rooted)? else {
        return Ok(None);
    };
    // 3. Let value be Completion(IteratorValue(result)).
    match iterator_value(agent, result) {
        Ok(v) => Ok(Some(v)),
        Err(e) => {
            // 4. If value is a throw completion, set iteratorRecord.[[Done]] to true.
            record.done = true;
            Err(e)
        }
    }
}

/// IteratorClose(iteratorRecord, completion) (§7.4.12). `completion` is the
/// result of the work that was interrupted; it is returned unless closing
/// itself throws (and the completion was not already a throw).
pub fn iterator_close<T>(
    agent: &mut Agent,
    record: &IteratorRecord,
    completion: JsResult<T>,
) -> JsResult<T> {
    // 3. Let innerResult be Completion(GetMethod(iterator, "return")).
    let iterator = record.iterator;
    let inner = get_method(
        agent,
        Value::Object(iterator),
        PropertyKey::String(agent.names.r#return),
    );
    // 4. If innerResult is a normal completion, then
    let inner = match inner {
        Ok(ret) if ret.is_undefined() => {
            // b. If return is undefined, return ? completion.
            return completion;
        }
        // c. Set innerResult to Completion(Call(return, iterator)).
        Ok(ret) => call(agent, ret, Value::Object(iterator), &[]),
        Err(e) => Err(e),
    };
    // 5. If completion is a throw completion, return ? completion.
    if matches!(
        completion,
        Err(JsError::Throw(_)) | Err(JsError::NotImplemented(_))
    ) {
        return completion;
    }
    // 6. If innerResult is a throw completion, return ? innerResult.
    let inner = inner?;
    // 7. If innerResult.[[Value]] is not an Object, throw a TypeError exception.
    if !inner.is_object() {
        return Err(agent.throw_type_error("iterator.return() did not return an object"));
    }
    // 8. Return ? completion.
    completion
}

/// CreateIteratorResultObject(value, done) (§7.4.14)
pub fn create_iter_result_object(agent: &mut Agent, value: Value, done: bool) -> Object {
    // 1. Let obj be OrdinaryObjectCreate(%Object.prototype%).
    let proto = agent.intrinsic("%Object.prototype%");
    let obj = agent.ordinary_object_create(Some(proto));
    // 2. Perform ! CreateDataPropertyOrThrow(obj, "value", value).
    let _ =
        create_data_property_or_throw(agent, obj, PropertyKey::String(agent.names.value), value);
    // 3. Perform ! CreateDataPropertyOrThrow(obj, "done", done).
    let _ = create_data_property_or_throw(
        agent,
        obj,
        PropertyKey::String(agent.names.done),
        Value::Boolean(done),
    );
    // 4. Return obj.
    obj
}

/// IteratorToList(iteratorRecord) (§7.4.16)
pub fn iterator_to_list(agent: &mut Agent, record: &mut IteratorRecord) -> JsResult<Vec<Value>> {
    let mut values = Vec::new();
    while let Some(v) = iterator_step_value(agent, record)? {
        values.push(v);
    }
    Ok(values)
}

impl crate::gc::Trace for IteratorRecord {
    fn trace(&self, t: &mut crate::gc::Tracer) {
        t.object(self.iterator);
        t.value(self.next_method);
    }
}
