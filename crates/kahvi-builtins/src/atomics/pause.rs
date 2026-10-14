//! Atomics.pause ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-atomics.pause

use kahvi_core::prelude::*;

/// Atomics.pause
pub(crate) fn pause(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function provides a hint to the CPU that the program is spin looping while waiting on a value.
    // It performs the following steps when called:
    //
    // 1. If the execution environment of the ECMAScript implementation supports signaling to the operating system or CPU that the current executing code is in a spin-wait loop, send that signal.
    // 2. Return *undefined*.
    //
    // NOTE:
    // This method is designed for programs implementing spin-wait loops, such as spinlock fast paths inside of mutexes, to provide a hint to the CPU that it is spinning while waiting on a value. It has no observable behaviour other than timing.
    // Implementations are expected to implement a pause or yield instruction if the best practices of the underlying architecture recommends such instructions in spin loops. For example, the Intel Optimization Manual recommends the pause instruction.
    // Implementations are encouraged to have an internal upper bound on the maximum amount of time paused on the order of tens to hundreds of nanoseconds.
    // NOTE:
    // Due to the overhead of function calls, it is reasonable that an inlined call to this method in an optimizing compiler waits a different amount of time than a non-inlined call.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Atomics.pause"))
}
