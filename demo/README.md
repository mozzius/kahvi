# kahvi demo site

Everything here runs on kahvi:

- **The server** is a [Hono](https://hono.dev) app (`app/server.mjs`),
  unbundled from `node_modules`, run by `kahvi-serve` (`serve/`). That is
  kahvi-run's host (console, timers, ES modules and packages) plus an HTTP
  listener and the web classes a fetch handler needs (`serve/src/web.js`:
  `Headers`, `Request`, `Response`, `Kahvi.serve(handler)`,
  `Kahvi.readFile(path)`, `Kahvi.svgToPng(svg)`; `Intl` is on). Bodies are whole byte arrays, not streams. One
  request at a time, one request per connection.
- **The playground** runs code in the browser with kahvi compiled to
  WebAssembly (`wasm/`, served gzipped as `/kahvi.wasm`). Each run gets a
  new Web Worker and a new instance; the page terminates the worker after
  5 seconds. Timers are the browser's, driven through the worker
  (`app/public/worker.js`).

- **The OpenGraph card** (`/og.png`, `app/og.mjs`) is laid out by
  [satori](https://github.com/vercel/satori) on kahvi and rasterized by
  `Kahvi.svgToPng` (resvg, in the host). Each page view links
  `/og.png?seed=…` with a fresh seed, which picks the gradient. kahvi has no
  WebAssembly or CommonJS, so this uses satori 0.0.44 (the last with an
  asm.js Yoga), bundled into one module by `app/build-satori.mjs` at build
  time.

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
cd app && npm ci && npm run build && ../target/release/kahvi-serve server.mjs
```

## Railway

`railway.json` at the repository root points Railway at `demo/Dockerfile`
and health-checks `/healthz`. Railway sets `PORT` and passes the commit as
`RAILWAY_GIT_COMMIT_SHA`.
