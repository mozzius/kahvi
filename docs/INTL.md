# Intl (ECMA-402)

kahvi implements the ECMAScript Internationalization API from the vendored
spec (`vendor/ecma402/`, `./spec` reads it like ECMA-262) on top of
[ICU4X](https://github.com/unicode-org/icu4x) 2.x with its compiled CLDR data.
test262's `intl402/` directory is the measure, as for everything else.

## The `intl` flag

`Agent::intl` decides whether the global `Intl` exists. It is on in the
test262 engine (`kahvi-cli`) and off in `kahvi-run` and the React Native host
while services are missing: code that feature-detects `Intl` should keep
taking its fallback path rather than reach a not-implemented stub. It is
turned on there once the services apps use (DateTimeFormat, NumberFormat,
PluralRules, Collator, Locale, RelativeTimeFormat, ListFormat, Segmenter)
pass.

The locale-sensitive ECMA-262 functions that ECMA-402 replaces
(`toLocaleString`, `localeCompare`, `toLocaleUpperCase`, …; the `sup-`
clauses) check `agent.intl`: when it is off they keep their ECMA-262
behaviour unchanged.

## Layout

- `crates/kahvi-builtins/src/intl*/`: one stub per built-in function,
  generated like every other built-in (`harness/gen/builtins.py`), installed
  on `%Intl%` and the `%Intl.X%` intrinsics.
- `support/intl.rs`: the shared foundations, written by the architect:
  - language tags: `is_well_formed_language_tag`,
    `canonicalize_unicode_locale_id`, `canonicalize_u_value`,
    `unicode_extension_components`,
    `insert_unicode_extension_and_canonicalize`;
  - `canonicalize_locale_list`, `available_locales`, `default_locale`;
  - negotiation: `lookup_matching_locale_by_prefix`,
    `lookup_matching_locale_by_best_fit` (the same algorithm),
    `resolve_locale`, `resolve_options`, `filter_locales`;
  - options: `get_options_object`, `coerce_options_to_object`,
    `get_option` (and `get_string_option`), `get_number_option`,
    `default_number_option`, `get_boolean_or_string_number_format_option`;
  - `IntlLocaleSlots` (the Intl.Locale internal slots, which
    CanonicalizeLocaleList reads) and `fallback_symbol` (%Intl%.[[FallbackSymbol]]).
- `support/intl_<service>.rs`: one per service, owned by that service's
  ticket: its internal-slot struct and the abstract operations several of
  its built-ins share (e.g. CreateDateTimeFormat, FormatNumericToString).

## Internal slots

A service object is an ordinary object with `ObjectKind::Native` slots: a
struct in `support/intl_<service>.rs` that derives `Debug` and implements
`Trace` (trace any `Object`/`Value` it holds, such as [[BoundFormat]]).

```rust
let o = ops::ordinary_create_from_constructor(
    agent, new_target, "%Intl.Collator.prototype%",
    ObjectKind::Native(Box::new(CollatorSlots { .. })),
)?;
// The brand check ("has an [[InitializedCollator]] internal slot"):
let Some(slots) = agent.native::<CollatorSlots>(o) else { /* TypeError */ };
```

Keep the spec's slots as fields (resolved option strings, digit counts,
…). The ICU4X object that does the work (a `Collator`, a
`DateTimeFormatter`, …) can live in the struct too: the `icu` crate is
built with its `sync` feature, so its types are `Send`. Build it once, in
the constructor, from the resolved options.

## ICU4X map

| Service | ICU4X |
| --- | --- |
| Locale | `icu::locale` (`Locale`, `LocaleExpander` for maximize/minimize) |
| Collator | `icu::collator` |
| DateTimeFormat | `icu::datetime` (field sets), `icu::calendar`, `icu::time` |
| NumberFormat | `icu::decimal` (and its decimal input type); currency, units and compact notation in `icu::experimental` |
| PluralRules | `icu::plurals` |
| ListFormat | `icu::list` |
| Segmenter | `icu::segmenter` (the `*_utf16` APIs: JS strings are UTF-16) |
| RelativeTimeFormat, DisplayNames, DurationFormat | `icu::experimental` |
| toLocaleUpperCase / toLocaleLowerCase | `icu::casemap` |

`[[AvailableLocales]]` is `available_locales()` for every service.
`localeData` for `resolve_options` is a closure giving, for a locale and a
relevant extension key, the supported values with the default first
(`None` is *null*), taken from ICU4X: the collations, numbering systems,
calendars and hour cycles it has for that locale.

Implementation-defined results (patterns, names, separators, the default
for a locale) come from ICU4X data, never from tables written to match
test expectations.

## Known gaps in the foundations

- CanonicalizeUValue has the CLDR aliases for `ca`, `ks`, `ms` and the
  boolean keys but not time zone (`tz`) aliases, and transformed (`-t-`)
  extension values are not canonicalized.
- LookupMatchingLocaleByBestFit is the lookup algorithm.
- DefaultLocale is always `en-US`.

## Out of scope

Temporal and `Intl.Era-monthcode` stay skipped (`conformance/skip.txt`).
