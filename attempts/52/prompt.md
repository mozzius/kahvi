# Ticket B-global-decode-uri-component: decodeURIComponent

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

- `crates/kahvi-builtins/src/global/decode_uri_component.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

decodeURIComponent ( _encodedURIComponent_ )
  id: sec-decodeuricomponent-encodeduricomponent   type: built-in function

This function computes a new version of a URI in which each escape sequence and UTF-8 encoding of the sort that might be introduced by the `encodeURIComponent` function is replaced with the UTF-16 encoding of the code point that it represents.
It is the %decodeURIComponent% intrinsic object.
It performs the following steps when called:

1. Let _componentString_ be ? ToString(_encodedURIComponent_).
2. Let _preserveEscapeSet_ be the empty String.
3. Return ? Decode(_componentString_, _preserveEscapeSet_).

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

- Every test262 run under built-ins/decodeURIComponent/ passes (check with `./t262 built-ins/decodeURIComponent/`). 20 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 50: not merged (contradicts_spec); continue from this diff. Change line ~73 of crates/kahvi-builtins/src/global/decode_uri_component.rs from `units[start..=k + 1].to_vec()` to `units[start..k + 1].to_vec()`. The spec's 'substring from start to k + 1' is end-exclusive, and after `k += 2` the escape occupies start..=k. The inclusive form duplicates the following code unit and can panic when k + 1 == len. Everything else matches the spec step by step and can be kept as is. Re-run built-ins/decodeURIComponent/ after the fix. The timeouts in S15.1.3.2_A2.5_T1 are likely interpreter speed, not this code, but check them separately.
- attempt 51: worker reported BLOCKED but changed files: S15.1.3.2_A2.5_T1 times out at 10s (about 1.3M loop iterations); needs interpreter speed-up or a longer test timeout to pass.

