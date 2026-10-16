+++
id = "I-number-format-range"
title = "Intl.NumberFormat formatRange"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/intl_number_format/prototype_format_range.rs", "crates/kahvi-builtins/src/intl_number_format/prototype_format_range_to_parts.rs", "crates/kahvi-builtins/src/support/intl_number_format.rs"]
read = ["docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/intl_number_format/prototype_format_to_parts.rs"]
spec = ["sec-intl.numberformat.prototype.formatrange", "sec-intl.numberformat.prototype.formatrangetoparts", "sec-partitionnumberrangepattern", "sec-formatapproximately", "sec-collapsenumberrange", "sec-formatnumericrange", "sec-formatnumericrangetoparts"]
target = ["intl402/NumberFormat/prototype/formatRange", "intl402/NumberFormat/prototype/formatRangeToParts"]
needs = ["I-number-format"]
priority = 4950
+++
`formatRange` and `formatRangeToParts`, on top of PartitionNumberPattern
from I-number-format, following the spec steps already in each stub (keep
each step as a comment directly above the code for it).

- The range separator and approximately sign come from ICU4X data; what
  ICU4X has no data for throws not-implemented naming it.
- Never write tables of expected outputs. No new Rust tests.
