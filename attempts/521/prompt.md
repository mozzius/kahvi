# Ticket B-atomics-notify: Atomics.notify

Implement this built-in in its file, following the spec steps already in
the stub: keep each step as a comment directly above the code for it.

- Abstract operations are in `kahvi_core::ops` (see docs/API.md); use them
  rather than re-implementing. If a helper you need is missing and other
  built-ins would need it too, write it in your file and say so in your
  final report; do not edit other files.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing. Never return
  a made-up value.
- No new Rust tests: the test262 target is the measure.

Kahvi has one agent, and it can never be suspended in a WaiterList (see
Atomics.wait: AgentCanSuspend() is false), so every WaiterList is empty and
the number of waiters notified is 0, after the validation and coercion
steps in spec order.

## Files you may change

- `crates/kahvi-builtins/src/atomics/notify.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/atomics.rs`
- `crates/kahvi-builtins/src/atomics/add.rs`
- `crates/kahvi-builtins/src/atomics/and.rs`
- `crates/kahvi-core/src/object/typed_array.rs`

## Spec

Atomics.notify ( _ta_, _index_, _count_ )
  id: sec-atomics.notify   type: built-in function

This function notifies some agents that are sleeping in the wait queue.
It performs the following steps when called:

1. Let _taRecord_ be ? ValidateIntegerTypedArray(_ta_, *true*).
2. Let _byteIndexInBuffer_ be ? ValidateAtomicAccess(_taRecord_, _index_).
3. If _count_ is *undefined*, then
   a. Set _count_ to +∞.
4. Else,
   a. Let _intCount_ be ? ToIntegerOrInfinity(_count_).
   b. Set _count_ to max(_intCount_, 0).
5. Let _buffer_ be _ta_.[[ViewedArrayBuffer]].
6. If IsSharedArrayBuffer(_buffer_) is *false*, return *+0*_𝔽.
7. Let _block_ be _buffer_.[[ArrayBufferData]].
8. Let _waiterList_ be GetWaiterList(_block_, _byteIndexInBuffer_).
9. Perform EnterCriticalSection(_waiterList_).
10. Let _waiters_ be RemoveWaiters(_waiterList_, _count_).
11. For each element _waiterRecord_ of _waiters_, do
   a. Perform NotifyWaiter(_waiterList_, _waiterRecord_).
12. Perform LeaveCriticalSection(_waiterList_).
13. Let _waitersCount_ be the number of elements in _waiters_.
14. Return 𝔽(_waitersCount_).

--- referenced: ValidateIntegerTypedArray ( _ta_: an ECMAScript language value, _waitable_: a Boolean, ): either a normal completion containing a TypedArray With Buffer Witness Record, or a throw completion
  id: sec-validateintegertypedarray

description: It is used to constrain the TypedArrays that can be used with %Atomics% functions, particularly those that can guarantee atomic interactions with shared memory.

1. NOTE: Bounds checking is not a synchronizing operation when _ta_'s backing buffer is a growable SharedArrayBuffer.
2. Let _taRecord_ be ? ValidateTypedArray(_ta_, ~unordered~).
3. Let _elementType_ be TypedArrayElementType(_ta_).
4. If IsNoTearConfiguration(_elementType_, ~seq-cst~) is *false*, throw a *TypeError* exception.
5. If _waitable_ is *true* and _elementType_ is neither ~int32~ nor ~bigint64~, throw a *TypeError* exception.
6. Return _taRecord_.

--- referenced: ValidateAtomicAccess ( _taRecord_: a TypedArray With Buffer Witness Record, _requestIndex_: an ECMAScript language value, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-validateatomicaccess

1. Let _length_ be TypedArrayLength(_taRecord_).
2. Let _accessIndex_ be ? ToIndex(_requestIndex_).
3. Assert: _accessIndex_ ≥ 0.
4. If _accessIndex_ ≥ _length_, throw a *RangeError* exception.
5. Let _ta_ be _taRecord_.[[Object]].
6. Let _elementSize_ be TypedArrayElementSize(_ta_).
7. Let _offset_ be _ta_.[[ByteOffset]].
8. Return (_accessIndex_ × _elementSize_) + _offset_.

--- referenced: ToIntegerOrInfinity ( _arg_: an ECMAScript language value, ): either a normal completion containing either an integer, +∞, or -∞, or a throw completion
  id: sec-tointegerorinfinity

description: It converts _arg_ to an integer representing its Number value with fractional part truncated, or to +∞ or -∞ when that Number value is infinite.

1. Let _number_ be ? ToNumber(_arg_).
2. If _number_ is one of *NaN*, *+0*_𝔽, or *-0*_𝔽, return 0.
3. If _number_ is *+∞*_𝔽, return +∞.
4. If _number_ is *-∞*_𝔽, return -∞.
5. Return truncate(ℝ(_number_)).

NOTE:
𝔽(ToIntegerOrInfinity(_x_)) never returns *-0*_𝔽 for any value of _x_. The truncation of the fractional part is performed after converting _x_ to a mathematical value.

--- referenced: IsSharedArrayBuffer ( _obj_: an ArrayBuffer or a SharedArrayBuffer, ): a Boolean
  id: sec-issharedarraybuffer

description: It tests whether an object is a SharedArrayBuffer.

1. If _obj_.[[ArrayBufferData]] is a Shared Data Block, return *true*.
2. Return *false*.

--- referenced: GetWaiterList ( _block_: a Shared Data Block, _i_: a non-negative integer that is evenly divisible by 4, ): a WaiterList Record
  id: sec-getwaiterlist

1. Assert: _i_ and _i_ + 3 are valid byte offsets within the memory of _block_.
2. Return the WaiterList Record that is referenced by the pair (_block_, _i_).

--- referenced: EnterCriticalSection ( _waiterList_: a WaiterList Record, ): ~unused~
  id: sec-entercriticalsection

1. Assert: The surrounding agent is not in the critical section for any WaiterList Record.
2. Wait until no agent is in the critical section for _waiterList_, then enter the critical section for _waiterList_ (without allowing any other agent to enter).
3. If _waiterList_.[[MostRecentLeaveEvent]] is not ~empty~, then
   a. NOTE: A _waiterList_ whose critical section has been entered at least once has a Synchronize event set by LeaveCriticalSection.
   b. Let _agentRecord_ be the Agent Record of the surrounding agent.
   c. Let _execution_ be _agentRecord_.[[CandidateExecution]].
   d. Let _eventsRecord_ be the Agent Events Record of _execution_.[[EventsRecords]] whose [[AgentSignifier]] is AgentSignifier().
   e. Let _enterEvent_ be a new Synchronize event.
   f. Append _enterEvent_ to _eventsRecord_.[[EventList]].
   g. Append (_waiterList_.[[MostRecentLeaveEvent]], _enterEvent_) to _eventsRecord_.[[AgentSynchronizesWith]].
4. Return ~unused~.

EnterCriticalSection has contention when an agent attempting to enter the critical section must wait for another agent to leave it. When there is no contention, FIFO order of EnterCriticalSection calls is observable. When there is contention, an implementation may choose an arbitrary order but may not cause an agent to wait indefinitely.

--- referenced: RemoveWaiters ( _waiterList_: a WaiterList Record, _count_: a non-negative integer or +∞, ): a List of Waiter Records
  id: sec-removewaiters

1. Assert: The surrounding agent is in the critical section for _waiterList_.
2. Let _length_ be the number of elements in _waiterList_.[[Waiters]].
3. Set _count_ to min(_count_, _length_).
4. Let _waiters_ be a List whose elements are the first _count_ elements of _waiterList_.[[Waiters]].
5. Remove the first _count_ elements of _waiterList_.[[Waiters]].
6. Return _waiters_.

--- referenced: NotifyWaiter ( _waiterList_: a WaiterList Record, _waiterRecord_: a Waiter Record, ): ~unused~
  id: sec-notifywaiter

1. Assert: The surrounding agent is in the critical section for _waiterList_.
2. If _waiterRecord_.[[PromiseCapability]] is ~blocking~, then
   a. Wake the agent whose signifier is _waiterRecord_.[[AgentSignifier]] from suspension.
   b. NOTE: This causes the agent to resume execution in SuspendThisAgent.
3. Else if AgentSignifier() is _waiterRecord_.[[AgentSignifier]], then
   a. Let _promiseCapability_ be _waiterRecord_.[[PromiseCapability]].
   b. Perform ! Call(_promiseCapability_.[[Resolve]], *undefined*, « _waiterRecord_.[[Result]] »).
4. Else,
   a. Perform EnqueueResolveInAgentJob(_waiterRecord_.[[AgentSignifier]], _waiterRecord_.[[PromiseCapability]], _waiterRecord_.[[Result]]).
5. Return ~unused~.

NOTE:
An agent must not access another agent's promise capability in any capacity beyond passing it to the host.

--- referenced: LeaveCriticalSection ( _waiterList_: a WaiterList Record, ): ~unused~
  id: sec-leavecriticalsection

1. Assert: The surrounding agent is in the critical section for _waiterList_.
2. Let _agentRecord_ be the Agent Record of the surrounding agent.
3. Let _execution_ be _agentRecord_.[[CandidateExecution]].
4. Let _eventsRecord_ be the Agent Events Record of _execution_.[[EventsRecords]] whose [[AgentSignifier]] is AgentSignifier().
5. Let _leaveEvent_ be a new Synchronize event.
6. Append _leaveEvent_ to _eventsRecord_.[[EventList]].
7. Set _waiterList_.[[MostRecentLeaveEvent]] to _leaveEvent_.
8. Leave the critical section for _waiterList_.
9. Return ~unused~.


## Done when

- Every test262 run under built-ins/Atomics/notify/ passes (check with `./t262 built-ins/Atomics/notify/`). 0 pass on the main branch right now.

