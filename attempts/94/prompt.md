# Ticket B-iterator-concat: Iterator.concat

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

## Files you may change

- `crates/kahvi-builtins/src/iterator/concat.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

Iterator.concat ( ..._items_ )
  id: sec-iterator.concat   type: built-in function

1. Let _iterables_ be a new empty List.
2. For each element _item_ of _items_, do
   a. If _item_ is not an Object, throw a *TypeError* exception.
   b. Let _method_ be ? GetMethod(_item_, %Symbol.iterator%).
   c. If _method_ is *undefined*, throw a *TypeError* exception.
   d. Append the Record { [[OpenMethod]]: _method_, [[Iterable]]: _item_ } to _iterables_.
3. Let _closure_ be a new Abstract Closure with no parameters that captures _iterables_ and performs the following steps when called:
   a. For each Record _iterable_ of _iterables_, do
      i. Let _iterator_ be ? Call(_iterable_.[[OpenMethod]], _iterable_.[[Iterable]]).
      ii. If _iterator_ is not an Object, throw a *TypeError* exception.
      iii. Let _iteratorRecord_ be ? GetIteratorDirect(_iterator_).
      iv. Let _innerAlive_ be *true*.
      v. Repeat, while _innerAlive_ is *true*,
         1. Let _innerValue_ be ? IteratorStepValue(_iteratorRecord_).
         2. If _innerValue_ is ~done~, then
            a. Set _innerAlive_ to *false*.
         3. Else,
            a. Let _completion_ be Completion(Yield(_innerValue_)).
            b. If _completion_ is an abrupt completion, then
               i. Return ? IteratorClose(_iteratorRecord_, _completion_).
   b. Return ReturnCompletion(*undefined*).
4. Let _gen_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
5. Set _gen_.[[UnderlyingIterators]] to a new empty List.
6. Return _gen_.

--- referenced: GetMethod ( _value_: an ECMAScript language value, _propertyKey_: a property key, ): either a normal completion containing either a function object or *undefined*, or a throw completion
  id: sec-getmethod

description: It is used to get the value of a specific property of an ECMAScript language value when the value of the property is expected to be a function.

1. Let _func_ be ? GetV(_value_, _propertyKey_).
2. If _func_ is either *undefined* or *null*, return *undefined*.
3. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
4. Return _func_.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).

--- referenced: GetIteratorDirect ( _obj_: an Object, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratordirect

1. Let _nextMethod_ be ? Get(_obj_, *"next"*).
2. Let _iteratorRecord_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: _nextMethod_, [[Done]]: *false* }.
3. Return _iteratorRecord_.

--- referenced: IteratorStepValue ( _iteratorRecord_: an Iterator Record, ): either a normal completion containing either an ECMAScript language value or ~done~, or a throw completion
  id: sec-iteratorstepvalue

description: It requests the next value from _iteratorRecord_.[[Iterator]] by calling _iteratorRecord_.[[NextMethod]] and returns either ~done~ indicating that the iterator has reached its end or the value from the IteratorResult object if a next value is available.

1. Let _result_ be ? IteratorStep(_iteratorRecord_).
2. If _result_ is ~done~, then
   a. Return ~done~.
3. Let _value_ be Completion(IteratorValue(_result_)).
4. If _value_ is a throw completion, then
   a. Set _iteratorRecord_.[[Done]] to *true*.
5. Return ? _value_.

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

--- referenced: Yield ( _arg_: an ECMAScript language value, ): either a normal completion containing an ECMAScript language value or an abrupt completion
  id: sec-yield

1. Let _genKind_ be GetGeneratorKind().
2. If _genKind_ is ~async~, return ? AsyncGeneratorYield(? Await(_arg_)).
3. Return ? GeneratorYield(CreateIteratorResultObject(_arg_, *false*)).

--- referenced: IteratorClose ( _iteratorRecord_: an Iterator Record, _completion_: a Completion Record, ): a Completion Record
  id: sec-iteratorclose

description: It is used to notify an iterator that it should perform any actions it would normally perform when it has reached its completed state.

1. Assert: _iteratorRecord_.[[Iterator]] is an Object.
2. Let _iterator_ be _iteratorRecord_.[[Iterator]].
3. Let _innerResult_ be Completion(GetMethod(_iterator_, *"return"*)).
4. If _innerResult_ is a normal completion, then
   a. Let _return_ be _innerResult_.[[Value]].
   b. If _return_ is *undefined*, return ? _completion_.
   c. Set _innerResult_ to Completion(Call(_return_, _iterator_)).
5. If _completion_ is a throw completion, return ? _completion_.
6. If _innerResult_ is a throw completion, return ? _innerResult_.
7. If _innerResult_.[[Value]] is not an Object, throw a *TypeError* exception.
8. Return ? _completion_.

--- referenced: ReturnCompletion ( _value_: an ECMAScript language value, ): a return completion
  id: sec-returncompletion

1. Return Completion Record { [[Type]]: ~return~, [[Value]]: _value_, [[Target]]: ~empty~ }.

--- referenced: CreateIteratorFromClosure ( _closure_: an Abstract Closure with no parameters, _genBrand_: a String or ~empty~, _genProto_: an Object, optional _extraSlots_: a List of names of internal slots, ): a Generator
  id: sec-createiteratorfromclosure

1. NOTE: _closure_ can contain uses of the Yield operation to yield an IteratorResult object.
2. If _extraSlots_ is not present, set _extraSlots_ to a new empty List.
3. Let _internalSlotsList_ be the list-concatenation of _extraSlots_ and « [[GeneratorState]], [[GeneratorContext]], [[GeneratorBrand]] ».
4. Let _gen_ be OrdinaryObjectCreate(_genProto_, _internalSlotsList_).
5. Set _gen_.[[GeneratorBrand]] to _genBrand_.
6. Set _gen_.[[GeneratorState]] to ~suspended-start~.
7. Let _callerContext_ be the running execution context.
8. Let _calleeContext_ be a new execution context.
9. Set the Function of _calleeContext_ to *null*.
10. Set the Realm of _calleeContext_ to the current Realm Record.
11. Set the ScriptOrModule of _calleeContext_ to _callerContext_'s ScriptOrModule.
12. If _callerContext_ is not already suspended, suspend _callerContext_.
13. Push _calleeContext_ onto the execution context stack; _calleeContext_ is now the running execution context.
14. Perform GeneratorStart(_gen_, _closure_).
15. Remove _calleeContext_ from the execution context stack and restore _callerContext_ as the running execution context.
16. Return _gen_.


## Done when

- Every test262 run under built-ins/Iterator/concat/ passes (check with `./t262 built-ins/Iterator/concat/`). 12 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 91: worker reported BLOCKED but changed files: StringIteratorPrototype.next (needed by built-ins/Iterator/concat/iterable-primitive-wrapper-objects.js)
- attempt 91: not merged (skips_observable_steps); continue from this diff. 1) Verify how the helper trait (crates/kahvi-builtins/src/support/iterator_helper.rs, not in the diff) routes generator.throw() into a suspended closure. If it has no throw hook, add the throw-completion path: on a throw resumption with self.current = Some(rec), take it, call ops::iterator_close with a throw completion (return() errors ignored, original error rethrown), and set current to None. Keep on_return for return completions. Check this against test262 tests that call throw() on a suspended concat helper. 2) Spec-faithful structure is otherwise correct: step 2.a-d validation and GetMethod order, lazy Call/GetIteratorDirect per iterable, IteratorStepValue ? propagation without closing, and ReturnCompletion(undefined) at the end. Keep these. 3) Lint nit at concat.rs:96: the unreachable! after self.current.as_mut() can be removed by restructuring, for example by binding the record directly from the Option after assignment, so no panic path is left in the code.

