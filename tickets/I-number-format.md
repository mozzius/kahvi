+++
id = "I-number-format"
title = "Intl.NumberFormat formatting"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/intl_number_format/*.rs", "crates/kahvi-builtins/src/support/intl_number_format.rs"]
read = ["docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/support/intl.rs", "crates/kahvi-builtins/src/intl_number_format/constructor.rs"]
spec = ["sec-intl.numberformat.prototype.format", "sec-formatnumber", "sec-tointlmathematicalvalue", "sec-formatnumerictostring", "sec-torawprecision", "sec-torawfixed", "sec-partitionnumberpattern", "sec-formatnumbertoparts", "sec-intl.numberformat.prototype.formattoparts"]
target = ["intl402/NumberFormat/"]
needs = []
priority = 5000
+++
The constructor, SetNumberFormatDigitOptions and resolvedOptions are done.
This ticket is the formatting: the `format` getter, FormatNumeric,
`formatToParts`, ToIntlMathematicalValue and FormatNumericToString,
following the spec steps already in each stub (keep each step as a comment
directly above the code for it). Read docs/INTL.md first.

- An Intl mathematical value is exact: keep it as a decimal
  (`icu::decimal::input::Decimal`, which parses strings and BigInts), never
  as an f64 once ToIntlMathematicalValue has read a string or BigInt.
- FormatNumericToString, ToRawPrecision and ToRawFixed are spec
  algorithms on decimals: implement them as written, with the rounding
  modes and increments from the slots. Make FormatNumericToString
  `pub(crate)` in `support/intl_number_format.rs` (Intl.PluralRules uses it).
- PartitionNumberPattern takes its patterns and symbols from ICU4X
  (`icu::decimal::DecimalFormatter` for digits, grouping and signs; the
  `icu::experimental` currency, percent, unit and compact formatters for
  the other styles and notations). Map ICU4X's parts to the spec's part
  types for formatToParts. Whatever ICU4X has no data for throws
  not-implemented naming it.
- Never write tables of expected outputs. No new Rust tests.
