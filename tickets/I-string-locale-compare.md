+++
id = "I-string-locale-compare"
title = "String.prototype.localeCompare (ECMA-402)"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/string/prototype_locale_compare.rs"]
read = ["docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/support/intl.rs", "crates/kahvi-builtins/src/support/intl_collator.rs"]
spec = ["sup-String.prototype.localeCompare"]
target = ["intl402/String/prototype/localeCompare/"]
needs = ["I-collator"]
priority = 3500
+++
This built-in already implements ECMA-262. ECMA-402 supersedes it (the
spec text below): when `agent.intl` is true, follow the ECMA-402 steps;
when it is false, keep today's ECMA-262 behaviour exactly as it is. Keep
each spec step as a comment directly above the code for it. Read
docs/INTL.md first.

- Construct the Intl service through its intrinsic (`ops::construct` on
  `agent.intrinsic("%Intl.X%")`), as the spec does, unless a support
  helper does the same thing without the observable lookups.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))`. No new Rust tests.
