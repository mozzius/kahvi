# Ticket B-iterator-prototype-take: Iterator.prototype.take

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

- `crates/kahvi-builtins/src/iterator/prototype_take.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

Iterator.prototype.take ( _limit_ )
  id: sec-iterator.prototype.take   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. If _obj_ is not an Object, throw a *TypeError* exception.
3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
4. Let _numberLimit_ be Completion(ToNumber(_limit_)).
5. IfAbruptCloseIterator(_numberLimit_, _iterated_).
6. If _numberLimit_ is *NaN*, then
   a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
   b. Return ? IteratorClose(_iterated_, _error_).
7. If _numberLimit_ is finite and _numberLimit_ > 𝔽(2^53 - 1), then
   a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
   b. Return ? IteratorClose(_iterated_, _error_).
8. Let _intLimit_ be ! ToIntegerOrInfinity(_numberLimit_).
9. If _intLimit_ < 0, then
   a. Let _error_ be ThrowCompletion(a newly created *RangeError* object).
   b. Return ? IteratorClose(_iterated_, _error_).
10. Set _iterated_ to ? GetIteratorDirect(_obj_).
11. Let _closure_ be a new Abstract Closure with no parameters that captures _iterated_ and _intLimit_ and performs the following steps when called:
   a. Let _remaining_ be 𝔽(_intLimit_).
   b. Repeat,
      i. If _remaining_ is *+0*_𝔽, then
         1. Return ? IteratorClose(_iterated_, ReturnCompletion(*undefined*)).
      ii. Set _remaining_ to _remaining_ - *1*_𝔽.
      iii. Let _value_ be ? IteratorStepValue(_iterated_).
      iv. If _value_ is ~done~, return ReturnCompletion(*undefined*).
      v. Let _completion_ be Completion(Yield(_value_)).
      vi. IfAbruptCloseIterator(_completion_, _iterated_).
12. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
13. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
14. Return _result_.

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

--- referenced: ToNumber ( _arg_: an ECMAScript language value, ): either a normal completion containing a Number or a throw completion
  id: sec-tonumber

description: It converts _arg_ to a value of type Number.

1. If _arg_ is a Number, return _arg_.
2. If _arg_ is either a Symbol or a BigInt, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *NaN*.
4. If _arg_ is either *null* or *false*, return *+0*_𝔽.
5. If _arg_ is *true*, return *1*_𝔽.
6. If _arg_ is a String, return StringToNumber(_arg_).
7. Assert: _arg_ is an Object.
8. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~number~).
9. Assert: _primitiveValue_ is not an Object.
10. Return ? ToNumber(_primitiveValue_).

[subclause sec-tonumber-applied-to-the-string-type: ToNumber Applied to the String Type]

--- referenced: ThrowCompletion ( _value_: an ECMAScript language value, ): a throw completion
  id: sec-throwcompletion

1. Return Completion Record { [[Type]]: ~throw~, [[Value]]: _value_, [[Target]]: ~empty~ }.

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

--- referenced: GetIteratorDirect ( _obj_: an Object, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratordirect

1. Let _nextMethod_ be ? Get(_obj_, *"next"*).
2. Let _iteratorRecord_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: _nextMethod_, [[Done]]: *false* }.
3. Return _iteratorRecord_.

--- referenced: ReturnCompletion ( _value_: an ECMAScript language value, ): a return completion
  id: sec-returncompletion

1. Return Completion Record { [[Type]]: ~return~, [[Value]]: _value_, [[Target]]: ~empty~ }.

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

--- referenced: Yield ( _arg_: an ECMAScript language value, ): either a normal completion containing an ECMAScript language value or an abrupt completion
  id: sec-yield

1. Let _genKind_ be GetGeneratorKind().
2. If _genKind_ is ~async~, return ? AsyncGeneratorYield(? Await(_arg_)).
3. Return ? GeneratorYield(CreateIteratorResultObject(_arg_, *false*)).

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

- Every test262 run under built-ins/Iterator/prototype/take/ passes (check with `./t262 built-ins/Iterator/prototype/take/`). 12 pass on the main branch right now.

