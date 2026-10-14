# Ticket B-reg-exp-prototype-get-flags: get RegExp.prototype.flags

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

- `crates/kahvi-builtins/src/reg_exp/prototype_get_flags.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`
- `crates/kahvi-builtins/src/reg_exp/prototype_test.rs`

## Spec

get RegExp.prototype.flags
  id: sec-get-regexp.prototype.flags   type: built-in function

`RegExp.prototype.flags` is an accessor property whose set accessor function is *undefined*. Its get accessor function performs the following steps when called:

1. Let _regexp_ be the *this* value.
2. If _regexp_ is not an Object, throw a *TypeError* exception.
3. Let _codeUnits_ be a new empty List.
4. Let _hasIndices_ be ToBoolean(? Get(_regexp_, *"hasIndices"*)).
5. If _hasIndices_ is *true*, append the code unit 0x0064 (LATIN SMALL LETTER D) to _codeUnits_.
6. Let _global_ be ToBoolean(? Get(_regexp_, *"global"*)).
7. If _global_ is *true*, append the code unit 0x0067 (LATIN SMALL LETTER G) to _codeUnits_.
8. Let _ignoreCase_ be ToBoolean(? Get(_regexp_, *"ignoreCase"*)).
9. If _ignoreCase_ is *true*, append the code unit 0x0069 (LATIN SMALL LETTER I) to _codeUnits_.
10. Let _multiline_ be ToBoolean(? Get(_regexp_, *"multiline"*)).
11. If _multiline_ is *true*, append the code unit 0x006D (LATIN SMALL LETTER M) to _codeUnits_.
12. Let _dotAll_ be ToBoolean(? Get(_regexp_, *"dotAll"*)).
13. If _dotAll_ is *true*, append the code unit 0x0073 (LATIN SMALL LETTER S) to _codeUnits_.
14. Let _unicode_ be ToBoolean(? Get(_regexp_, *"unicode"*)).
15. If _unicode_ is *true*, append the code unit 0x0075 (LATIN SMALL LETTER U) to _codeUnits_.
16. Let _unicodeSets_ be ToBoolean(? Get(_regexp_, *"unicodeSets"*)).
17. If _unicodeSets_ is *true*, append the code unit 0x0076 (LATIN SMALL LETTER V) to _codeUnits_.
18. Let _sticky_ be ToBoolean(? Get(_regexp_, *"sticky"*)).
19. If _sticky_ is *true*, append the code unit 0x0079 (LATIN SMALL LETTER Y) to _codeUnits_.
20. Return the String value whose code units are the elements of the List _codeUnits_. If _codeUnits_ has no elements, the empty String is returned.

[subclause sec-regexphasflag: RegExpHasFlag (
_regexp_: an ECMAScript language value,
_codeUnit_: a code unit,
): either a normal completion containing either a Boolean or *undefined*, or a throw completion]

--- referenced: ToBoolean ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-toboolean

description: It converts _arg_ to a value of type Boolean.

1. If _arg_ is a Boolean, return _arg_.
2. If _arg_ is one of *undefined*, *null*, *+0*_𝔽, *-0*_𝔽, *NaN*, *0*_ℤ, or the empty String, return *false*.
3. If the host is a web browser or otherwise supports [sec-IsHTMLDDA-internal-slot], then
   a. If _arg_ is an Object and _arg_ has an [[IsHTMLDDA]] internal slot, return *false*.
4. Return *true*.

--- referenced: Get ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-get-o-p

description: It is used to retrieve the value of a specific property of an object.

1. Return ? _obj_.[[Get]](_propertyKey_, _obj_).


## Done when

- Every test262 run under built-ins/RegExp/prototype/flags/ passes (check with `./t262 built-ins/RegExp/prototype/flags/`). 6 pass on the main branch right now.

