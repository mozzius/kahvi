# Real-world tests

Real npm packages run under `kahvi-run`, checked against Node. test262 tests
features one at a time; these run tens of thousands of lines of real library
code at once (parsers, a formatter, a minifier, schema validation, Proxy-based
state) and catch what falls between the features.

```
./rw                       # all cases (builds the release kahvi-run first)
./rw prettier zod          # some cases
./rw --gc-stress           # collect at every safe point
./rw --record              # rewrite cases/*.out from Node
./rw --update-expected     # rewrite expected-pass.txt from this run
```

- `package.json` / `package-lock.json` pin every package; `./rw` runs
  `npm ci` when `node_modules/` is missing. Packages load unbundled, through
  kahvi-run's `node_modules` resolver.
- `cases/NAME.mjs` is an ES module that drives one package on fixed inputs
  and prints plain strings (never objects, so console formatting does not
  matter). `cases/lib/` holds shared helpers and inputs, not cases.
- `cases/NAME.out` is Node's stdout for the case. A case passes when
  kahvi-run prints exactly that and exits 0. Both run with `TZ=UTC` and an
  otherwise empty environment.
- `expected-pass.txt` is the ratchet: a listed case that fails is a
  regression and `./rw` exits 1. Other failures are reported, grouped by
  their first line of stderr, without failing the run.

Rules, as for test262: goldens only ever come from Node (CI re-records them
and fails on any difference), packages are never patched, and the engine never
special-cases a package. A failure is minimised to the smallest snippet that
differs from Node and fixed in the engine or the runtime.

CI: `.github/workflows/realworld.yml`, report-only (not a required check).
