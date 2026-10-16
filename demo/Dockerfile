# kahvi playground: kahvi-run behind a small HTTP server.
# Build from the repository root: docker build -f demo/Dockerfile .

FROM rust:1.97-slim-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY crates crates
COPY harness/t262 harness/t262
RUN cargo build --release --locked -p kahvi-runtime

# The page is rendered by kahvi itself.
COPY conformance/expected-pass.txt demo/render.mjs ./
ARG RAILWAY_GIT_COMMIT_SHA
ARG COMMIT=${RAILWAY_GIT_COMMIT_SHA}
RUN COMMIT="${COMMIT:-unknown}" \
    TEST262_PASSING="$(grep -cv '^#' expected-pass.txt)" \
    target/release/kahvi-run render.mjs > index.html

FROM python:3.12-slim-bookworm
RUN useradd --system --no-create-home kahvi
COPY --from=build /src/target/release/kahvi-run /usr/local/bin/kahvi-run
COPY --from=build /src/index.html /app/index.html
COPY demo/server.py /app/server.py
USER kahvi
ENV PORT=8080 PYTHONUNBUFFERED=1
EXPOSE 8080
CMD ["python3", "/app/server.py"]
