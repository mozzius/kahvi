+++
id = "I-date-to-locale-string"
title = "Date.prototype toLocaleString / toLocaleDateString / toLocaleTimeString (ECMA-402)"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/date/prototype_to_locale_string.rs", "crates/kahvi-builtins/src/date/prototype_to_locale_date_string.rs", "crates/kahvi-builtins/src/date/prototype_to_locale_time_string.rs"]
read = ["docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/support/intl.rs", "crates/kahvi-builtins/src/support/intl_date_time_format.rs"]
spec = ["sup-date.prototype.tolocalestring", "sup-date.prototype.tolocaledatestring", "sup-date.prototype.tolocaletimestring"]
target = ["intl402/Date/"]
needs = ["I-date-time-format"]
priority = 3200
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
