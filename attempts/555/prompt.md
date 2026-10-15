# Ticket I-string-locale-case: String.prototype.toLocaleUpperCase / toLocaleLowerCase (ECMA-402)

This built-in already implements ECMA-262. ECMA-402 supersedes it (the
spec text below): when `agent.intl` is true, follow the ECMA-402 steps;
when it is false, keep today's ECMA-262 behaviour exactly as it is. Keep
each spec step as a comment directly above the code for it. Read
docs/INTL.md first.

- Construct the Intl service through its intrinsic (`ops::construct` on
  `agent.intrinsic("%Intl.X%")`), as the spec does, unless a support
  helper does the same thing without the observable lookups.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))`. No new Rust tests.
- Case mapping is ICU4X's `icu::casemap::CaseMapper` with the resolved
  locale's language (TransformCase).

## Files you may change

- `crates/kahvi-builtins/src/string/prototype_to_locale_upper_case.rs`
- `crates/kahvi-builtins/src/string/prototype_to_locale_lower_case.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`

## Spec

String.prototype.toLocaleUpperCase ( [ _locales_ ] )
  id: sup-string.prototype.tolocaleuppercase   type: built-in function

This definition supersedes the definition provided in ECMA-262, [sec-string.prototype.tolocaleuppercase].
This function interprets a String value as a sequence of code points, as described in ECMA-262, [sec-ecmascript-language-types-string-type]. The following steps are taken:

1. Let _thisValue_ be ? RequireObjectCoercible(*this* value).
2. Let _string_ be ? ToString(_thisValue_).
3. Return ? TransformCase(_string_, _locales_, ~upper~).

NOTE:
The `toLocaleUpperCase` function is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.

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


String.prototype.toLocaleLowerCase ( [ _locales_ ] )
  id: sup-string.prototype.tolocalelowercase   type: built-in function

This definition supersedes the definition provided in ECMA-262, [sec-string.prototype.tolocalelowercase].
This function interprets a String value as a sequence of code points, as described in ECMA-262, [sec-ecmascript-language-types-string-type]. The following steps are taken:

1. Let _thisValue_ be ? RequireObjectCoercible(*this* value).
2. Let _string_ be ? ToString(_thisValue_).
3. Return ? TransformCase(_string_, _locales_, ~lower~).

NOTE:
The `toLocaleLowerCase` function is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
[subclause sec-transform-case: TransformCase (
_string_: a String,
_locales_: an ECMAScript language value,
_targetCase_: ~lower~ or ~upper~,
)]

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


## Done when

- Every test262 run under intl402/String/prototype/toLocaleUpperCase/, intl402/String/prototype/toLocaleLowerCase/ passes (check with `./t262 intl402/String/prototype/toLocaleUpperCase/ intl402/String/prototype/toLocaleLowerCase/`). 0 pass on the main branch right now.

