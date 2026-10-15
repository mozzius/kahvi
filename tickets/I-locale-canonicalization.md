+++
id = "I-locale-canonicalization"
title = "Unicode extension value aliases (CanonicalizeUValue, -t- fields)"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/support/intl.rs"]
read = ["docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/support/intl.rs"]
spec = ["sec-canonicalizeuvalue", "sec-canonicalizeunicodelocaleid"]
target = ["intl402/Intl/getCanonicalLocales/"]
needs = []
priority = 2900
+++
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
