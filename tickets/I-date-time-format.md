+++
id = "I-date-time-format"
title = "Intl.DateTimeFormat"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/intl_date_time_format/*.rs", "crates/kahvi-builtins/src/support/intl_date_time_format.rs"]
read = ["crates/kahvi-builtins/src/support/intl_dtf_icu.rs", "docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/support/intl.rs", "crates/kahvi-builtins/src/intl_collator/supported_locales_of.rs", "crates/kahvi-builtins/src/disposable_stack/constructor.rs"]
spec = ["sec-formatdatetime", "sec-formatdatetimepattern", "sec-partitiondatetimepattern", "sec-formatdatetimetoparts", "sec-intl.datetimeformat.prototype.format", "sec-intl.datetimeformat.prototype.formatRange", "sec-formatdatetimerange"]
target = ["intl402/DateTimeFormat/"]
needs = []
priority = 4900
+++
The constructor, CreateDateTimeFormat and resolvedOptions are done (the
architect's ICU4X layer is `support/intl_dtf_icu.rs`; read it first). This
ticket is the formatting: the `format` getter, FormatDateTime,
`formatToParts`, `formatRange` and `formatRangeToParts`, following the spec
steps already in each stub (keep each step as a comment directly above the
code for it). Read docs/INTL.md first.

- Format with ICU4X, never by hand: `intl_dtf_icu::formatter(&prefs,
  &slots.fields)` gives the `DateTimeFormatter` for the format the
  constructor chose (`Prefs` from the slots: [[Locale]], [[Calendar]],
  [[NumberingSystem]], [[HourCycle]]). Convert the time value (epoch
  milliseconds) to an ICU4X zoned date-time in [[TimeZone]] (a named zone
  or a UTC offset) and format it; for formatToParts, map ICU4X's parts
  (`writeable` parts, `icu::datetime::parts`) to the spec's part types.
- [[HourCycle]] "h24" has no ICU4X pattern: throw not-implemented there.
- A range is two formatted dates unless ICU4X has range patterns for the
  field set; whatever ICU4X cannot do throws not-implemented naming it.
- Never write tables of expected outputs. No new Rust tests.
