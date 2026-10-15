# Engine CLI contract

The test262 runner (`harness/t262`) drives the engine binary
(`target/debug/kahvi`) through this contract. It is owned by the harness:
changing it means changing the runner, which only the architect does.

```
kahvi [--prelude FILE]... [--module] [--no-intl] FILE
```

- Each `--prelude` file is evaluated, in order, as a **script** in the same
  realm before `FILE`. The runner uses this only for module tests (test262
  harness files are scripts even when the test is a module).
- `FILE` is evaluated as a script, or as a module with `--module`. Module
  specifiers starting with `./` resolve relative to `FILE`'s directory.
- After evaluation the engine runs the job queue (promise jobs) until empty,
  then exits.
- The global `Intl` exists unless `--no-intl` is given (docs/INTL.md). The
  mjsunit runner passes it while ECMA-402 is incomplete.

## Host bindings

Defined on the global object (writable, configurable, non-enumerable) before
any code runs:

- `print(value)` — writes `ToString(value)` and `\n` to stdout.
- `$262` — as specified in `vendor/test262/INTERPRETING.md`
  (`createRealm`, `evalScript`, `detachArrayBuffer`, `gc`, `global`,
  `agent` may be absent until SharedArrayBuffer work).

## Exit status and error reporting

| Situation | stderr | exit |
|---|---|---|
| Completed normally | (anything) | 0 |
| Uncaught exception (including early/parse errors) | a line `Uncaught <Name>: <message>` | 1 |
| Bad invocation / unreadable file | `kahvi: ...` | 2 |

`<Name>` is the `name` of the thrown value's constructor (e.g. `TypeError`,
`SyntaxError`, `Test262Error`). For a thrown primitive the line is
`Uncaught <ToString(value)>`.

Anything the engine has not implemented yet must throw an `InternalError`
whose message starts with `not implemented: ` followed by what is missing,
e.g. `Uncaught InternalError: not implemented: Array.prototype.flatMap`.
The runner groups these to show what to build next. Never return a
plausible-looking value instead.

A Rust panic (exit 101), a signal, or any exit code ≥ 128 is a **crash** and
always fails the test.
