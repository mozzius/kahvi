//! The bytecode interpreter.

use crate::agent::Agent;
use crate::error::JsResult;
use crate::ops::IteratorRecord;
use crate::value::{Object, Value};

/// A loaded script or module.
#[derive(Debug)]
pub struct ScriptRecord {}

/// The private names visible to a class body (§9.2 PrivateEnvironment Record).
#[derive(Debug)]
pub struct PrivateEnv {}

pub fn call_ecma_function(
    agent: &mut Agent,
    _f: Object,
    _this: Value,
    _args: &[Value],
) -> JsResult<Value> {
    Err(agent.not_implemented("calling ECMAScript functions"))
}

pub fn construct_ecma_function(
    agent: &mut Agent,
    _f: Object,
    _args: &[Value],
    _new_target: Object,
) -> JsResult<Object> {
    Err(agent.not_implemented("constructing ECMAScript functions"))
}

pub fn create_async_from_sync_iterator(
    agent: &mut Agent,
    _sync: IteratorRecord,
) -> JsResult<IteratorRecord> {
    Err(agent.not_implemented("CreateAsyncFromSyncIterator"))
}
