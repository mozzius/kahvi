+++
id = "I-date-time-format"
title = "Intl.DateTimeFormat"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/intl_date_time_format/*.rs", "crates/kahvi-builtins/src/support/intl_date_time_format.rs"]
read = ["docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/support/intl.rs", "crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs", "crates/kahvi-builtins/src/disposable_stack/constructor.rs"]
spec = ["sec-intl.datetimeformat", "sec-chaindatetimeformat", "sec-createdatetimeformat", "sec-formatdatetime"]
target = ["intl402/DateTimeFormat/"]
needs = []
priority = 4900
+++
Implement this Intl service: the constructor, its internal slots and its
prototype methods, following the spec steps already in each stub (keep each
step as a comment directly above the code for it). Read docs/INTL.md first.

- The negotiation and option operations (ResolveOptions, GetOption, ...)
  are in `support/intl.rs`; do not re-implement them. Put this service's
  internal-slot struct and shared abstract operations in its own
  `support/intl_*.rs` file (in scope).
- Locale data and formatting come from ICU4X (`icu` crate). Never write
  tables of expected outputs.
- Work in steps: the constructor and resolvedOptions first, then the
  methods. Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing.
- No new Rust tests: test262 is the measure.
- Map the component options to an ICU4X field set and let ICU4X pick the
  pattern; resolvedOptions reports the components of the pattern used.
- Intl.DateTimeFormat called without `new` on an Intl object uses
  `support::intl::fallback_symbol` (ChainDateTimeFormat).
