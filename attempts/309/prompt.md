# Ticket B-object-prototype-to-locale-string: Object.prototype.toLocaleString

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

- `crates/kahvi-builtins/src/object/prototype_to_locale_string.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.prototype.toLocaleString ( [ _reserved1_ [ , _reserved2_ ] ] )
  id: sec-object.prototype.tolocalestring   type: built-in function

This method performs the following steps when called:

1. Let _thisValue_ be the *this* value.
2. Return ? Invoke(_thisValue_, *"toString"*).

The optional parameters to this method are not used but are intended to correspond to the parameter pattern used by ECMA-402 `toLocaleString` methods. Implementations that do not include ECMA-402 support must not use those parameter positions for other purposes.
NOTE:
This method provides a generic `toLocaleString` implementation for objects that have no locale-sensitive `toString` behaviour. `Array`, `Number`, `Date`, and %TypedArray% provide their own locale-sensitive `toLocaleString` methods.
NOTE:
ECMA-402 intentionally does not provide an alternative to this default implementation.

--- referenced: Invoke ( _value_: an ECMAScript language value, _propertyKey_: a property key, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-invoke

description: It is used to call a method property of an ECMAScript language value. _value_ serves as both the lookup point for the property and the *this* value of the call. _argList_ is the list of arguments values passed to the method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. Let _func_ be ? GetV(_value_, _propertyKey_).
3. Return ? Call(_func_, _value_, _argList_).


## Done when

- Every test262 run under built-ins/Object/prototype/toLocaleString/ passes (check with `./t262 built-ins/Object/prototype/toLocaleString/`). 14 pass on the main branch right now.

