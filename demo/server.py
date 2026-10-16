#!/usr/bin/env python3
"""HTTP server for the kahvi playground.

Serves the page that kahvi rendered at build time and runs submitted
JavaScript with `kahvi-run`, one process per request, with no environment,
an empty working directory and limits on wall time, CPU, memory and output.
"""

import json
import os
import resource
import shutil
import signal
import subprocess
import tempfile
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

HERE = os.path.dirname(os.path.abspath(__file__))
# Resolved now: the child gets an empty environment, so no PATH.
KAHVI_RUN = shutil.which(os.environ.get("KAHVI_RUN", "kahvi-run"))
INDEX = os.path.join(HERE, "index.html")

MAX_SOURCE = 64 * 1024
MAX_OUTPUT = 64 * 1024
OUTPUT_FILE_LIMIT = 1024 * 1024
WALL_SECONDS = 5
CPU_SECONDS = 4
# kahvi-run reserves a 512 MB stack for its main thread, so this leaves
# about 768 MB for the heap.
MEMORY_BYTES = 1280 * 1024 * 1024
RUN_SLOTS = threading.BoundedSemaphore(int(os.environ.get("RUN_SLOTS", "2")))


def limit_child():
    os.setsid()
    resource.setrlimit(resource.RLIMIT_CPU, (CPU_SECONDS, CPU_SECONDS))
    resource.setrlimit(resource.RLIMIT_AS, (MEMORY_BYTES, MEMORY_BYTES))
    resource.setrlimit(resource.RLIMIT_FSIZE, (OUTPUT_FILE_LIMIT, OUTPUT_FILE_LIMIT))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def read_capped(path):
    with open(path, "rb") as f:
        data = f.read(MAX_OUTPUT + 1)
    text = data[:MAX_OUTPUT].decode("utf-8", "replace")
    return text, len(data) > MAX_OUTPUT


def run(source):
    with tempfile.TemporaryDirectory(prefix="kahvi-") as tmp:
        main = os.path.join(tmp, "main.mjs")
        with open(main, "w", encoding="utf-8") as f:
            f.write(source)
        out_path = os.path.join(tmp, "stdout")
        err_path = os.path.join(tmp, "stderr")
        start = time.monotonic()
        timed_out = False
        with open(out_path, "wb") as out, open(err_path, "wb") as err:
            proc = subprocess.Popen(
                [KAHVI_RUN, "main.mjs"],
                cwd=tmp,
                env={},
                stdin=subprocess.DEVNULL,
                stdout=out,
                stderr=err,
                preexec_fn=limit_child,
            )
            try:
                proc.wait(timeout=WALL_SECONDS)
            except subprocess.TimeoutExpired:
                timed_out = True
                os.killpg(proc.pid, 9)
                proc.wait()
        elapsed = time.monotonic() - start
        stdout, out_cut = read_capped(out_path)
        stderr, err_cut = read_capped(err_path)
        status = proc.returncode
        if timed_out:
            reason = "wall-clock limit (5 s)"
        elif status in (-signal.SIGKILL, -signal.SIGXCPU):
            reason = "CPU time limit"
        elif status == -signal.SIGXFSZ:
            reason = "output limit (1 MB)"
        elif status == -signal.SIGABRT and "memory allocation" in stderr:
            reason = "memory limit"
        else:
            reason = None
        return {
            "stdout": stdout,
            "stderr": stderr,
            "truncated": out_cut or err_cut,
            "exitCode": status if status >= 0 else None,
            "signal": -status if status < 0 and not timed_out else None,
            "timedOut": timed_out,
            "limit": reason,
            "ms": round(elapsed * 1000),
        }


class Handler(BaseHTTPRequestHandler):
    server_version = "kahvi-demo"

    def send(self, code, body, content_type):
        self.send_response(code)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def send_json(self, code, obj):
        self.send(code, json.dumps(obj).encode(), "application/json")

    def do_GET(self):
        if self.path in ("/", "/index.html"):
            with open(INDEX, "rb") as f:
                self.send(200, f.read(), "text/html; charset=utf-8")
        elif self.path == "/healthz":
            self.send(200, b"ok\n", "text/plain")
        else:
            self.send(404, b"not found\n", "text/plain")

    def do_POST(self):
        if self.path != "/run":
            return self.send(404, b"not found\n", "text/plain")
        length = int(self.headers.get("Content-Length") or 0)
        if length > MAX_SOURCE:
            return self.send_json(413, {"error": "source is over 64 KB"})
        try:
            source = json.loads(self.rfile.read(length))["code"]
            if not isinstance(source, str):
                raise TypeError
        except (ValueError, KeyError, TypeError):
            return self.send_json(400, {"error": 'expected JSON {"code": "..."}'})
        if not RUN_SLOTS.acquire(blocking=False):
            return self.send_json(503, {"error": "busy, try again in a moment"})
        try:
            self.send_json(200, run(source))
        finally:
            RUN_SLOTS.release()


def main():
    if KAHVI_RUN is None:
        raise SystemExit("kahvi-run not found (set KAHVI_RUN)")
    port = int(os.environ.get("PORT", "8080"))
    server = ThreadingHTTPServer(("0.0.0.0", port), Handler)
    print(f"kahvi demo listening on :{port}", flush=True)
    server.serve_forever()


if __name__ == "__main__":
    main()
