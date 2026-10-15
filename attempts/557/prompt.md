# Ticket I-locale-canonicalization: Unicode extension value aliases (CanonicalizeUValue, -t- fields)

`support/intl.rs` canonicalizes language tags with ICU4X's
`LocaleCanonicalizer`, which does not replace deprecated Unicode extension
values. Finish CanonicalizeUValue (time zone `tz` aliases such as
`cnckg` -> `cnsha`) and canonicalize transformed extension (`-t-`) field
values (`m0-names` -> `m0-prprname`), from CLDR's common/bcp47 alias data as
ICU4X exposes it. Keep each spec step as a comment directly above the code.

- Only change the canonicalization functions; every Intl service calls
  this file, so keep the signatures.
- If ICU4X does not carry the alias data, reply `BLOCKED:` saying which
  data is missing rather than writing a table by hand.
- No new Rust tests.

## Files you may change

- `crates/kahvi-builtins/src/support/intl.rs`

## Read first

- `docs/INTL.md`
- `docs/API.md`
- `crates/kahvi-builtins/src/support/intl.rs`

## Spec

CanonicalizeUValue ( _ukey_: a Unicode locale extension sequence key defined in Unicode Technical Standard #35 Part 1 Core Section 3.6.1 Key and Type Definitions, _uvalue_: a String, ): a String
  id: sec-canonicalizeuvalue   type: abstract operation

description: The returned String is the canonical and case-regularized form of _uvalue_ as a value of _ukey_.

1. Let _lowerValue_ be the ASCII-lowercase of _uvalue_.
2. Let _canonicalized_ be the String value resulting from canonicalizing _lowerValue_ as a value of key _ukey_ per Unicode Technical Standard #35 Part 1 Core, Annex C LocaleId Canonicalization Section 5 Canonicalizing Syntax, Processing LocaleIds.
3. NOTE: It is recommended that implementations use the 'u' extension data in common/bcp47 provided by the Common Locale Data Repository (available at https://cldr.unicode.org/).
4. Return _canonicalized_.


CanonicalizeUnicodeLocaleId ( _locale_: a language tag, ): a Unicode canonicalized locale identifier
  id: sec-canonicalizeunicodelocaleid   type: abstract operation

description: It returns the canonical and case-regularized form of _locale_.

1. Let _localeId_ be the String value resulting from performing the Processing LocaleIds algorithm to transform _locale_ to canonical form per Unicode Technical Standard #35 Part 1 Core, Annex C LocaleId Canonicalization.
2. If _localeId_ contains a substring that is a Unicode locale extension sequence, then
   a. Let _extension_ be the String value consisting of the substring of the Unicode locale extension sequence within _localeId_.
   b. Let _newExtension_ be *"-u"*.
   c. Let _components_ be UnicodeExtensionComponents(_extension_).
   d. For each element _attr_ of _components_.[[Attributes]], do
      i. Set _newExtension_ to the string-concatenation of _newExtension_, *"-"*, and _attr_.
   e. For each Record { [[Key]], [[Value]] } _keyword_ of _components_.[[Keywords]], do
      i. Set _newExtension_ to the string-concatenation of _newExtension_, *"-"*, and _keyword_.[[Key]].
      ii. If _keyword_.[[Value]] is not the empty String, then
         1. Set _newExtension_ to the string-concatenation of _newExtension_, *"-"*, and _keyword_.[[Value]].
   f. Assert: _newExtension_ is not *"-u"*.
   g. Set _localeId_ to a copy of _localeId_ in which the first appearance of substring _extension_ has been replaced with _newExtension_.
3. Return _localeId_.

NOTE:
Step [step-canonicalizeunicodelocaleid-u-extension] ensures that a Unicode locale extension sequence in the returned language tag contains:
- only the first instance of any attribute duplicated in the input, and
- only the first keyword for a given key in the input.


## Done when

- Every test262 run under intl402/Intl/getCanonicalLocales/ passes (check with `./t262 intl402/Intl/getCanonicalLocales/`). 0 pass on the main branch right now.

