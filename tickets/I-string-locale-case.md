+++
id = "I-string-locale-case"
title = "String.prototype.toLocaleUpperCase / toLocaleLowerCase (ECMA-402)"
kind = "builtin"
tier = "sonnet"
scope = ["crates/kahvi-builtins/src/string/prototype_to_locale_upper_case.rs", "crates/kahvi-builtins/src/string/prototype_to_locale_lower_case.rs"]
read = ["docs/INTL.md", "docs/API.md", "crates/kahvi-builtins/src/support/intl.rs"]
spec = ["sup-string.prototype.tolocaleuppercase", "sup-string.prototype.tolocalelowercase"]
target = ["intl402/String/prototype/toLocaleUpperCase/", "intl402/String/prototype/toLocaleLowerCase/"]
needs = []
priority = 3400
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
- Case mapping is ICU4X's `icu::casemap::CaseMapper` with the resolved
  locale's language (TransformCase).
