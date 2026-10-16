# kahvi demo site: a Hono app on kahvi-serve, and a playground that runs
# kahvi compiled to WebAssembly in the browser.
# Build from the repository root: docker build -f demo/Dockerfile .

FROM rust:1.97-slim-bookworm AS rust
RUN rustup target add wasm32-unknown-unknown
WORKDIR /src
# kahvi's workspace (the engine crates inherit its settings).
COPY Cargo.toml Cargo.lock ./
COPY crates crates
COPY harness/t262 harness/t262
COPY demo/Cargo.toml demo/Cargo.lock demo/
COPY demo/.cargo demo/.cargo
COPY demo/serve demo/serve
COPY demo/wasm demo/wasm
WORKDIR /src/demo
RUN cargo build --release --locked -p kahvi-serve \
 && cargo build --release --locked --target wasm32-unknown-unknown -p kahvi-playground \
 && gzip -9 -c target/wasm32-unknown-unknown/release/kahvi_playground.wasm > kahvi.wasm.gz

FROM node:22-bookworm-slim AS packages
WORKDIR /app
COPY demo/app/package.json demo/app/package-lock.json ./
RUN npm ci --omit=dev --no-audit --no-fund

FROM debian:bookworm-slim
RUN useradd --system --no-create-home kahvi
COPY --from=rust /src/demo/target/release/kahvi-serve /usr/local/bin/kahvi-serve
WORKDIR /app
COPY --from=packages /app/node_modules node_modules
COPY demo/app/server.mjs demo/app/package.json ./
COPY demo/app/public public
COPY --from=rust /src/demo/kahvi.wasm.gz public/kahvi.wasm.gz
COPY conformance/expected-pass.txt ./
ARG RAILWAY_GIT_COMMIT_SHA
ENV COMMIT=${RAILWAY_GIT_COMMIT_SHA} PORT=8080
USER kahvi
EXPOSE 8080
CMD ["kahvi-serve", "server.mjs"]
