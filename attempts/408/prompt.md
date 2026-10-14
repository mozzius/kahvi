# Ticket B-string-prototype-locale-compare: String.prototype.localeCompare

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

- `crates/kahvi-builtins/src/string/prototype_locale_compare.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/string_iterator.rs`
- `crates/kahvi-builtins/src/support/regexp.rs`

## Spec

String.prototype.localeCompare ( _that_ [ , _reserved1_ [ , _reserved2_ ] ] )
  id: sec-string.prototype.localecompare   type: built-in function

An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used:
This method returns a Number other than *NaN* representing the result of an implementation-defined locale-sensitive String comparison of the *this* value (converted to a String _string_) with _that_ (converted to a String _thatValue_). The result is intended to correspond with a sort order of String values according to conventions of the host environment's current locale, and will be negative when _string_ is ordered before _thatValue_, positive when _string_ is ordered after _thatValue_, and zero in all other cases (representing no relative ordering between _string_ and _thatValue_).
Before performing the comparisons, this method performs the following steps to prepare the Strings:

1. Let _thisValue_ be the *this* value.
2. Perform ? RequireObjectCoercible(_thisValue_).
3. Let _string_ be ? ToString(_thisValue_).
4. Let _thatValue_ be ? ToString(_that_).

The meaning of the optional second and third parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not assign any other interpretation to those parameter positions.
The actual return values are implementation-defined to permit encoding additional information in them, but this method, when considered as a method of two arguments, is required to be a consistent comparator defining a total ordering on the set of all Strings. This method is also required to recognize and honour canonical equivalence according to the Unicode Standard, including returning *+0*_𝔽 when comparing distinguishable Strings that are canonically equivalent.
NOTE:
This method itself is not directly suitable as an argument to `Array.prototype.sort` because the latter requires a function of two arguments.
NOTE:
This method may rely on whatever language- and/or locale-sensitive comparison functionality is available to the ECMAScript environment from the host environment, and is intended to compare according to the conventions of the host environment's current locale. However, regardless of comparison capabilities, this method must recognize and honour canonical equivalence according to the Unicode Standard—for example, the following comparisons must all return *+0*_𝔽:
// Å ANGSTROM SIGN vs.
// Å LATIN CAPITAL LETTER A + COMBINING RING ABOVE
"\u212B".localeCompare("A\u030A")
// Ω OHM SIGN vs.
// Ω GREEK CAPITAL LETTER OMEGA
"\u2126".localeCompare("\u03A9")
// ṩ LATIN SMALL LETTER S WITH DOT BELOW AND DOT ABOVE vs.
// ṩ LATIN SMALL LETTER S + COMBINING DOT ABOVE + COMBINING DOT BELOW
"\u1E69".localeCompare("s\u0307\u0323")
// ḍ̇ LATIN SMALL LETTER D WITH DOT ABOVE + COMBINING DOT BELOW vs.
// ḍ̇ LATIN SMALL LETTER D WITH DOT BELOW + COMBINING DOT ABOVE
"\u1E0B\u0323".localeCompare("\u1E0D\u0307")
// 가 HANGUL CHOSEONG KIYEOK + HANGUL JUNGSEONG A vs.
// 가 HANGUL SYLLABLE GA
"\u1100\u1161".localeCompare("\uAC00")
For a definition and discussion of canonical equivalence see the Unicode Standard, chapters 2 and 3, as well as Unicode Standard Annex #15, Unicode Normalization Forms and Unicode Technical Note #5, Canonical Equivalence in Applications. Also see Unicode Technical Standard #10, Unicode Collation Algorithm.
It is recommended that this method should not honour Unicode compatibility equivalents or compatibility decompositions as defined in the Unicode Standard, chapter 3, section 3.7.
NOTE:
This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.

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

- Every test262 run under built-ins/String/prototype/localeCompare/ passes (check with `./t262 built-ins/String/prototype/localeCompare/`). 16 pass on the main branch right now.

## Previous attempts

Your worktree already contains a previous attempt's changes (not merged). Start from them and fix what the reviewer pointed out.

- attempt 392: not merged (unrelated_changes); continue from this diff. Keep the implementation as it is: ToString/RequireObjectCoercible steps in order, then NFC plus code-unit comparison via kahvi_unicode::normalize_code_points. Restore the original NOTE example comments (the \u escape sequences in the stub) exactly as they were in the stub, and do not edit the spec comment text. Optionally, check whether kahvi_core::ops already has UTF-16/code-point helpers before keeping the local to_code_points/from_code_points copies. The ticket allows local helpers only if they are missing from ops.
- attempt 396: not merged (unrelated_changes); continue from this diff. The implementation is sound and can be kept. Steps 1-4 run in spec order with `?` on RequireObjectCoercible and both ToString calls. The result is computed by comparing NFC code units, which honours canonical equivalence and gives a total preorder. Only cleanup is needed: restore the original bytes of the spec NOTE quote comments that the diff rewrote (the Å, Ω, ṩ, ḍ̇ and 가 example lines), so the diff contains only the code and comments for the steps. Optional check: confirm that kahvi_unicode::normalize_code_points accepts lone surrogate code points (0xD800-0xDFFF) as u32 without panicking, since to_code_points can pass them through. Do not change the code-unit comparison to something else just because the spec mentions locale; the spec allows an implementation-defined comparison here.
- attempt 404: worker reported BLOCKED but changed files: none
- attempt 404: not merged (unrelated_changes); continue from this diff. The core logic is sound and can be kept. Before merging: (1) restore the original bytes of the spec-quoted NOTE comment lines so the diff only touches code and the new explanatory comment. (2) Check whether kahvi_core::ops already provides CodePointAt / UTF16EncodeCodePoint-style helpers (or a UTF-16 to code point conversion). The ticket says to use ops rather than re-implement, so to_code_points and from_code_points should be replaced with those if they exist. If they do not exist, keep them and say so in the report. (3) Optional: confirm kahvi_unicode::normalize_code_points NFC handles Hangul composition, since the 가 test depends on it.

