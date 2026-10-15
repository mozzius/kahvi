# Runtime (M4)

Three layers, each usable on its own:

- **`kahvi`** (`crates/kahvi`): the embedding API. A `Runtime` is one agent
  with one realm and the ECMAScript built-ins; nothing from the outside
  world. It evaluates scripts and modules, calls functions, defines host
  functions backed by Rust closures (`Runtime::function`), runs the job
  queue and reports unhandled rejections. Values are handles valid until the
  next call that can run JavaScript; `persist` keeps one alive (a
  `Persistent` is a GC root until its last clone is dropped). `Runtime` is
  `Send`.
- **`kahvi-core` host hooks** that an embedder fills in: `host_now` (clock),
  `host_load_module` (HostLoadImportedModule), `host_import_meta`,
  `Job::Callback` (HostEnqueueGenericJob), `Agent::persist`,
  `Agent::host_function`. The engine crates never touch files, the clock or
  processes themselves.
- **`kahvi-runtime`** (`crates/kahvi-runtime`, binary `kahvi-run FILE`): a
  host for running ES modules from the file system.

## kahvi-run

The entry file is an ES module (ESM only: no CommonJS, no `require`).

- **Globals:** `console` (`log`, `info`, `debug` to stdout; `warn`, `error`,
  `trace` to stderr; `assert`), `setTimeout`, `setInterval`,
  `clearTimeout`, `clearInterval`, `queueMicrotask`, `globalThis`,
  `structuredClone` (no `transfer`; a DataCloneError is an `Error` named
  `DataCloneError`, there being no DOMException), `TextEncoder`,
  `TextDecoder` (UTF-8 only; other encodings throw not-implemented),
  `atob` and `btoa` (an InvalidCharacterError is an `Error` with that name),
  and `process.env` (the environment variables; nothing else of Node's
  `process`). The encoding globals come from `kahvi::encoding`, which the
  React Native host (`kahvi-ffi`) installs too, as Hermes has them.
- **Modules:** `./` and `../` specifiers resolve against the importing
  file, `/...` and `file://` URLs are absolute; JSON modules with
  `with { type: 'json' }`. `import.meta.url`, `.filename`, `.dirname`.
- **Packages:** a bare specifier (`name`, `@scope/name`, optionally with a
  subpath) is looked up in `node_modules` in the importing module's
  directory and each one above it. `package.json` `"exports"` decides what a
  package exposes (strings, arrays, the `kahvi`, `import` and `default`
  conditions, subpaths, `*` patterns, `null`); without it, `"main"` or
  `index.js`. `"imports"` (`#name`) and self-references are not supported.
- **Event loop:** evaluate the entry module; run microtasks; then repeatedly
  run the next due timer (by due time, then creation order) followed by the
  microtasks it queued, until no timers are left.
- **Exit status:** 0; 1 for an uncaught exception (`Uncaught Name:
  message` on stderr) or a promise rejected with no handler by the end of a
  turn (`Uncaught (in promise) ...`); 13 when the entry module's top-level
  await never settles; 2 for a bad invocation.
- **console formatting** (`inspect.rs`) reads internal slots and own
  properties without running JavaScript: getters show as `[Getter]`,
  proxies are not looked through. A leading string argument may hold
  `%s %d %i %f %o %O %j %c %%`.
- `KAHVI_GC_STRESS=1` collects at every safe point, as in `kahvi`;
  `KAHVI_GC_STRESS=N` at one in N on average (docs/GC.md).

## End-to-end tests

`./e2e [--gc-stress] [NAME...]` runs `harness/e2e/tests/*.mjs` with
`kahvi-run` and compares stdout, stderr and exit status with `NAME.out`,
`NAME.err` and the test's `// exit: N` comment. These cover what test262
cannot see (output, the event loop, files, exit statuses); engine semantics
stay test262's job. They are architect-owned.

## Next

- `error.stack` (source positions are already in the bytecode).
- More host APIs as real programs need them (`TextEncoder`, `URL`,
  `fetch`), each with e2e tests.
