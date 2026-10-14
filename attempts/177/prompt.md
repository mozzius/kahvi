# Ticket B-string-prototype-match-all: String.prototype.matchAll

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

- `crates/kahvi-builtins/src/string/prototype_match_all.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.matchAll ( _regexpOrPattern_ )
  id: sec-string.prototype.matchall   type: built-in function

This method performs a regular expression match of the String representing the *this* value against _regexpOrPattern_ and returns an iterator that yields match results. Each match result is an Array containing the matched portion of the String as the first element, followed by the portions matched by any capturing groups. If the regular expression never matches, the returned iterator does not yield any match results.
It performs the following steps when called:

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. If _regexpOrPattern_ is an Object, then
   a. Let _isRegexp_ be ? IsRegExp(_regexpOrPattern_).
   b. If _isRegexp_ is *true*, then
      i. Let _flags_ be ? Get(_regexpOrPattern_, *"flags"*).
      ii. Perform ? RequireObjectCoercible(_flags_).
      iii. If ? ToString(_flags_) does not contain *"g"*, throw a *TypeError* exception.
   c. Let _matcher_ be ? GetMethod(_regexpOrPattern_, %Symbol.matchAll%).
   d. If _matcher_ is not *undefined*, then
      i. Return ? Call(_matcher_, _regexpOrPattern_, « _thisValue_ »).
4. Let _string_ be ? ToString(_thisValue_).
5. Let _regexp_ be ? RegExpCreate(_regexpOrPattern_, *"g"*).
6. Return ? Invoke(_regexp_, %Symbol.matchAll%, « _string_ »).

NOTE: This method is intentionally generic, it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
NOTE: Similarly to `String.prototype.split`, `String.prototype.matchAll` is designed to typically act without mutating its inputs.

--- referenced: RequireObjectCoercible ( _arg_: an ECMAScript language value, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireobjectcoercible

description: It throws an error if _arg_ is a value that cannot be converted to an Object using ToObject.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. Return ~unused~.

--- referenced: IsRegExp ( _arg_: an ECMAScript language value, ): either a normal completion containing a Boolean or a throw completion
  id: sec-isregexp

1. If _arg_ is not an Object, return *false*.
2. Let _matcher_ be ? Get(_arg_, %Symbol.match%).
3. If _matcher_ is not *undefined*, return ToBoolean(_matcher_).
4. If _arg_ has a [[RegExpMatcher]] internal slot, return *true*.
5. Return *false*.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).

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

--- referenced: RegExpCreate ( _pattern_: an ECMAScript language value, _flags_: a String or *undefined*, ): either a normal completion containing an Object or a throw completion
  id: sec-regexpcreate

1. Let _obj_ be ! RegExpAlloc(%RegExp%).
2. Return ? RegExpInitialize(_obj_, _pattern_, _flags_).

--- referenced: Invoke ( _value_: an ECMAScript language value, _propertyKey_: a property key, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-invoke

description: It is used to call a method property of an ECMAScript language value. _value_ serves as both the lookup point for the property and the *this* value of the call. _argList_ is the list of arguments values passed to the method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. Let _func_ be ? GetV(_value_, _propertyKey_).
3. Return ? Call(_func_, _value_, _argList_).


## Done when

- Every test262 run under built-ins/String/prototype/matchAll/ passes (check with `./t262 built-ins/String/prototype/matchAll/`). 8 pass on the main branch right now.

