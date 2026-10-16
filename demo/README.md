# kahvi playground

A web page where you type JavaScript and kahvi runs it.

- `render.mjs` renders the page. It runs on `kahvi-run` when the image is
  built, with the commit and the number of test262 runs in
  `conformance/expected-pass.txt`.
- `server.py` serves the page and `POST /run` (`{"code": "..."}`). Each run
  is a fresh `kahvi-run main.mjs` in an empty temporary directory with no
  environment, limited to 5 s wall time, 4 s CPU, 1.25 GB of address space
  (512 MB of it is kahvi-run's reserved stack) and 1 MB of output, two runs
  at a time (`RUN_SLOTS`).

The process limits are the sandbox: kahvi-run has no network or file APIs,
but a module can still `import` a `.js` or `.json` file by absolute path, so
run the image as it is built (unprivileged user, nothing else in it).

## Running

```sh
docker build -f demo/Dockerfile -t kahvi-demo .
docker run --rm -p 8080:8080 kahvi-demo
```

Without Docker: `cargo build --release -p kahvi-runtime`, render the page
with `COMMIT=$(git rev-parse HEAD) TEST262_PASSING=$(grep -cv '^#'
conformance/expected-pass.txt) target/release/kahvi-run demo/render.mjs >
demo/index.html`, then `KAHVI_RUN=target/release/kahvi-run python3
demo/server.py`.

## Railway

`railway.json` at the repository root points Railway at `demo/Dockerfile`
and health-checks `/healthz`. Railway sets `PORT`.
