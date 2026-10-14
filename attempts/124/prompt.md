# Ticket B-iterator-prototype-flat-map: Iterator.prototype.flatMap

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

- `crates/kahvi-builtins/src/iterator/prototype_flat_map.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/iterator_helper.rs`
- `crates/kahvi-builtins/src/iterator/prototype_map.rs`

## Spec

Iterator.prototype.flatMap ( _mapper_ )
  id: sec-iterator.prototype.flatmap   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be the *this* value.
2. If _obj_ is not an Object, throw a *TypeError* exception.
3. Let _iterated_ be the Iterator Record { [[Iterator]]: _obj_, [[NextMethod]]: *undefined*, [[Done]]: *false* }.
4. If IsCallable(_mapper_) is *false*, then
   a. Let _error_ be ThrowCompletion(a newly created *TypeError* object).
   b. Return ? IteratorClose(_iterated_, _error_).
5. Set _iterated_ to ? GetIteratorDirect(_obj_).
6. Let _closure_ be a new Abstract Closure with no parameters that captures _iterated_ and _mapper_ and performs the following steps when called:
   a. Let _counter_ be *+0*_𝔽.
   b. Repeat,
      i. Let _value_ be ? IteratorStepValue(_iterated_).
      ii. If _value_ is ~done~, return ReturnCompletion(*undefined*).
      iii. Let _mapped_ be Completion(Call(_mapper_, *undefined*, « _value_, _counter_ »)).
      iv. IfAbruptCloseIterator(_mapped_, _iterated_).
      v. Let _innerIterator_ be Completion(GetIteratorFlattenable(_mapped_, ~reject-primitives~)).
      vi. IfAbruptCloseIterator(_innerIterator_, _iterated_).
      vii. Let _innerAlive_ be *true*.
      viii. Repeat, while _innerAlive_ is *true*,
         1. Let _innerValue_ be Completion(IteratorStepValue(_innerIterator_)).
         2. IfAbruptCloseIterator(_innerValue_, _iterated_).
         3. If _innerValue_ is ~done~, then
            a. Set _innerAlive_ to *false*.
         4. Else,
            a. Let _completion_ be Completion(Yield(_innerValue_)).
            b. If _completion_ is an abrupt completion, then
               i. Let _backupCompletion_ be Completion(IteratorClose(_innerIterator_, _completion_)).
               ii. IfAbruptCloseIterator(_backupCompletion_, _iterated_).
               iii. Return ? IteratorClose(_iterated_, _completion_).
      ix. NOTE: The following step will not change _counter_ once it reaches *2^53*_𝔽.
      x. Set _counter_ to _counter_ + *1*_𝔽.
7. Let _result_ be CreateIteratorFromClosure(_closure_, *"Iterator Helper"*, %IteratorHelperPrototype%, « [[UnderlyingIterators]] »).
8. Set _result_.[[UnderlyingIterators]] to « _iterated_ ».
9. Return _result_.

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

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

--- referenced: ReturnCompletion ( _value_: an ECMAScript language value, ): a return completion
  id: sec-returncompletion

1. Return Completion Record { [[Type]]: ~return~, [[Value]]: _value_, [[Target]]: ~empty~ }.

--- referenced: Completion ( _completionRecord_: a Completion Record, ): a Completion Record
  id: sec-completion-ao

description: It is used to emphasize that a Completion Record is being returned.
skip return checks: true

1. Assert: _completionRecord_ is a Completion Record.
2. Return _completionRecord_.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).

--- referenced: GetIteratorFlattenable ( _obj_: an ECMAScript language value, _primitiveHandling_: ~iterate-string-primitives~ or ~reject-primitives~, ): either a normal completion containing an Iterator Record or a throw completion
  id: sec-getiteratorflattenable

1. If _obj_ is not an Object, then
   a. If _primitiveHandling_ is ~reject-primitives~, throw a *TypeError* exception.
   b. Assert: _primitiveHandling_ is ~iterate-string-primitives~.
   c. If _obj_ is not a String, throw a *TypeError* exception.
2. Let _method_ be ? GetMethod(_obj_, %Symbol.iterator%).
3. If _method_ is *undefined*, then
   a. Let _iterator_ be _obj_.
4. Else,
   a. Let _iterator_ be ? Call(_method_, _obj_).
5. If _iterator_ is not an Object, throw a *TypeError* exception.
6. Return ? GetIteratorDirect(_iterator_).

--- referenced: Yield ( _arg_: an ECMAScript language value, ): either a normal completion containing an ECMAScript language value or an abrupt completion
  id: sec-yield

1. Let _genKind_ be GetGeneratorKind().
2. If _genKind_ is ~async~, return ? AsyncGeneratorYield(? Await(_arg_)).
3. Return ? GeneratorYield(CreateIteratorResultObject(_arg_, *false*)).


## Done when

- Every test262 run under built-ins/Iterator/prototype/flatMap/ passes (check with `./t262 built-ins/Iterator/prototype/flatMap/`). 12 pass on the main branch right now.

