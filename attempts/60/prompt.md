# Ticket B-string-prototype-symbol-iterator: String.prototype[Symbol.iterator]

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

- `crates/kahvi-builtins/src/string/prototype_symbol_iterator.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype [ %Symbol.iterator% ] ( )
  id: sec-string.prototype-%symbol.iterator%   type: built-in function

This method returns an iterator object that iterates over the code points of a String value, returning each code point as a String value.
It performs the following steps when called:

1. Let _string_ be the *this* value.
2. Perform ? RequireObjectCoercible(_string_).
3. Set _string_ to ? ToString(_string_).
4. Let _closure_ be a new Abstract Closure with no parameters that captures _string_ and performs the following steps when called:
   a. Let _length_ be the length of _string_.
   b. Let _position_ be 0.
   c. Repeat, while _position_ < _length_,
      i. Let _codePoint_ be CodePointAt(_string_, _position_).
      ii. Let _nextIndex_ be _position_ + _codePoint_.[[CodeUnitCount]].
      iii. Let _resultString_ be the substring of _string_ from _position_ to _nextIndex_.
      iv. Set _position_ to _nextIndex_.
      v. Perform ? GeneratorYield(CreateIteratorResultObject(_resultString_, *false*)).
   d. Return NormalCompletion(~unused~).
5. Return CreateIteratorFromClosure(_closure_, *"%StringIteratorPrototype%"*, %StringIteratorPrototype%).

The value of the *"name"* property of this method is *"[Symbol.iterator]"*.

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

--- referenced: ToString ( _arg_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-tostring

description: It converts _arg_ to a value of type String.

1. If _arg_ is a String, return _arg_.
2. If _arg_ is a Symbol, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *"undefined"*.
4. If _arg_ is *null*, return *"null"*.
5. If _arg_ is *true*, return *"true"*.
6. If _arg_ is *false*, return *"false"*.
7. If _arg_ is a Number, return Number::toString(_arg_, 10).
8. If _arg_ is a BigInt, return BigInt::toString(_arg_, 10).
9. Assert: _arg_ is an Object.
10. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~string~).
11. Assert: _primitiveValue_ is not an Object.
12. Return ? ToString(_primitiveValue_).

--- referenced: GeneratorYield ( _iteratorResult_: an Object that conforms to the IteratorResult interface, ): either a normal completion containing an ECMAScript language value or an abrupt completion
  id: sec-generatoryield

1. Let _genContext_ be the running execution context.
2. Assert: _genContext_ is the execution context of a generator.
3. Let _gen_ be the value of the Generator component of _genContext_.
4. Assert: GetGeneratorKind() is ~sync~.
5. Set _gen_.[[GeneratorState]] to ~suspended-yield~.
6. Return ? RunCallerContext(_iteratorResult_).

--- referenced: CreateIteratorResultObject ( _value_: an ECMAScript language value, _done_: a Boolean, ): an Object that conforms to the IteratorResult interface
  id: sec-createiterresultobject

description: It creates an object that conforms to the IteratorResult interface.

1. Let _obj_ be OrdinaryObjectCreate(%Object.prototype%).
2. Perform ! CreateDataPropertyOrThrow(_obj_, *"value"*, _value_).
3. Perform ! CreateDataPropertyOrThrow(_obj_, *"done"*, _done_).
4. Return _obj_.

--- referenced: NormalCompletion ( _value_: any value except a Completion Record, ): a normal completion
  id: sec-normalcompletion

skip return checks: true

1. Return Completion Record { [[Type]]: ~normal~, [[Value]]: _value_, [[Target]]: ~empty~ }.

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

- Every test262 run under built-ins/String/prototype/Symbol.iterator/ passes (check with `./t262 built-ins/String/prototype/Symbol.iterator/`). 8 pass on the main branch right now.

