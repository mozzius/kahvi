+++
id = "T-SMOKE"
title = "kahvi --help prints usage"
kind = "infra"
tier = "haiku"
scope = ["crates/kahvi-cli/src/main.rs"]
read = ["docs/ENGINE-CLI.md", "crates/kahvi-cli/src/main.rs"]
check = [
  "\"$KAHVI_BIN\" --help | grep -q '^usage: kahvi'",
  "test \"$(\"$KAHVI_BIN\" --help 2>&1 >/dev/null)\" = ''",
]
+++
Pipeline smoke test (not engine work). Make `kahvi --help` print the usage
line — the same text as the existing usage error, starting with
`usage: kahvi` — to stdout, and exit with status 0. Nothing on stderr.
Do not change any other behaviour.
