+++
id = "I-supported-values-of"
title = "Intl.supportedValuesOf"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/intl/supported_values_of.rs"]
read = ["docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/support/intl.rs"]
spec = ["sec-intl.supportedvaluesof", "sec-availablecalendars", "sec-availablecanonicalcollations", "sec-availablecanonicalcurrencies", "sec-availablecanonicalnumberingsystems", "sup-availablenamedtimezoneidentifiers", "sec-availableprimarytimezoneidentifiers", "sec-availablecanonicalunits"]
target = ["intl402/Intl/supportedValuesOf/"]
needs = []
priority = 3000
+++
Implement Intl.supportedValuesOf following the spec steps in the stub (keep
each step as a comment directly above the code for it). Read docs/INTL.md
first.

- Each list comes from the data ICU4X ships (calendars ICU4X's
  `icu::calendar` supports, collations in its collation data, the
  numbering systems in its decimal data, the IANA time zones of
  `icu::time`, ...). Never write lists to match test expectations; a list
  ICU4X cannot provide stays `not_implemented` naming it.
- No new Rust tests.
