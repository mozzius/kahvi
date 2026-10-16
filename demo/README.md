# kahvi demo site

Everything here runs on kahvi:

- **The server** is a [Hono](https://hono.dev) app (`app/server.mjs`),
  unbundled from `node_modules`, run by `kahvi-serve` (`serve/`). That is
  kahvi-run's host (console, timers, ES modules and packages) plus an HTTP
  listener and the web classes a fetch handler needs (`serve/src/web.js`:
  `Headers`, `Request`, `Response`, `Kahvi.serve(handler)`,
  `Kahvi.readFile(path)`). Bodies are whole byte arrays, not streams. One
  request at a time, one request per connection.
- **The playground** runs code in the browser with kahvi compiled to
  WebAssembly (`wasm/`, served gzipped as `/kahvi.wasm`). Each run gets a
  new Web Worker and a new instance; the page terminates the worker after
  5 seconds. Timers are the browser's, driven through the worker
  (`app/public/worker.js`).

`demo/` is a Cargo workspace of its own; its crates use the engine crates by
path.

## Running

```sh
docker build -f demo/Dockerfile -t kahvi-demo .
docker run --rm -p 8080:8080 kahvi-demo
```

Without Docker, from `demo/`:

```sh
cargo build --release -p kahvi-serve
cargo build --release --target wasm32-unknown-unknown -p kahvi-playground
gzip -9c target/wasm32-unknown-unknown/release/kahvi_playground.wasm > app/public/kahvi.wasm.gz
cp ../conformance/expected-pass.txt app/
cd app && npm ci && ../target/release/kahvi-serve server.mjs
```

## Railway

`railway.json` at the repository root points Railway at `demo/Dockerfile`
and health-checks `/healthz`. Railway sets `PORT` and passes the commit as
`RAILWAY_GIT_COMMIT_SHA`.
