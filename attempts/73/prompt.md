# Ticket B-reg-exp-prototype-symbol-match-all: RegExp.prototype[Symbol.matchAll]

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

- `crates/kahvi-builtins/src/reg_exp/prototype_symbol_match_all.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

RegExp.prototype [ %Symbol.matchAll% ] ( _string_ )
  id: sec-regexp-prototype-%symbol.matchall%   type: built-in function

This method performs the following steps when called:

1. Let _regexp_ be the *this* value.
2. If _regexp_ is not an Object, throw a *TypeError* exception.
3. Set _string_ to ? ToString(_string_).
4. Let _speciesCtor_ be ? SpeciesConstructor(_regexp_, %RegExp%).
5. Let _flags_ be ? ToString(? Get(_regexp_, *"flags"*)).
6. Let _matcher_ be ? Construct(_speciesCtor_, « _regexp_, _flags_ »).
7. Let _lastIndex_ be ? ToLength(? Get(_regexp_, *"lastIndex"*)).
8. Perform ? Set(_matcher_, *"lastIndex"*, _lastIndex_, *true*).
9. If _flags_ contains *"g"*, let _global_ be *true*.
10. Else, let _global_ be *false*.
11. If _flags_ contains *"u"* or _flags_ contains *"v"*, let _fullUnicode_ be *true*.
12. Else, let _fullUnicode_ be *false*.
13. Return CreateRegExpStringIterator(_matcher_, _string_, _global_, _fullUnicode_).

The value of the *"name"* property of this method is *"[Symbol.matchAll]"*.

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

--- referenced: SpeciesConstructor ( _obj_: an Object, _defaultCtor_: a constructor, ): either a normal completion containing a constructor or a throw completion
  id: sec-speciesconstructor

description: It is used to retrieve the constructor that should be used to create new objects that are derived from _obj_. _defaultCtor_ is the constructor to use if a constructor %Symbol.species% property cannot be found starting from _obj_.

1. Let _ctor_ be ? Get(_obj_, *"constructor"*).
2. If _ctor_ is *undefined*, return _defaultCtor_.
3. If _ctor_ is not an Object, throw a *TypeError* exception.
4. Let _species_ be ? Get(_ctor_, %Symbol.species%).
5. If _species_ is either *undefined* or *null*, return _defaultCtor_.
6. If IsConstructor(_species_) is *true*, return _species_.
7. Throw a *TypeError* exception.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

--- referenced: Construct ( _ctor_: a constructor, optional _argList_: a List of ECMAScript language values, optional _newTarget_: a constructor, ): either a normal completion containing an Object or a throw completion
  id: sec-construct

description: It is used to call the [[Construct]] internal method of a function object. _argList_ and _newTarget_ are the values to be passed as the corresponding arguments of the internal method. If _argList_ is not present, a new empty List is used as its value. If _newTarget_ is not present, _ctor_ is used as its value.

1. If _newTarget_ is not present, set _newTarget_ to _ctor_.
2. If _argList_ is not present, set _argList_ to a new empty List.
3. Return ? _ctor_.[[Construct]](_argList_, _newTarget_).

NOTE:
If _newTarget_ is not present, this operation is equivalent to: `new F(...argumentsList)`

--- referenced: ToLength ( _arg_: an ECMAScript language value, ): either a normal completion containing a non-negative integral Number or a throw completion
  id: sec-tolength

description: It clamps and truncates _arg_ to a non-negative integral Number suitable for use as the length of an array-like object.

1. Let _length_ be ? ToIntegerOrInfinity(_arg_).
2. Let _clampedLength_ be the result of clamping _length_ between 0 and 2^53 - 1.
3. Return 𝔽(_clampedLength_).

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: CreateRegExpStringIterator ( _regexp_: an Object, _string_: a String, _global_: a Boolean, _fullUnicode_: a Boolean, ): an Object
  id: sec-createregexpstringiterator

1. Let _iterator_ be OrdinaryObjectCreate(%RegExpStringIteratorPrototype%, « [[IteratingRegExp]], [[IteratedString]], [[Global]], [[Unicode]], [[Done]] »).
2. Set _iterator_.[[IteratingRegExp]] to _regexp_.
3. Set _iterator_.[[IteratedString]] to _string_.
4. Set _iterator_.[[Global]] to _global_.
5. Set _iterator_.[[Unicode]] to _fullUnicode_.
6. Set _iterator_.[[Done]] to *false*.
7. Return _iterator_.


## Done when

- Every test262 run under built-ins/RegExp/prototype/Symbol.matchAll/ passes (check with `./t262 built-ins/RegExp/prototype/Symbol.matchAll/`). 8 pass on the main branch right now.

