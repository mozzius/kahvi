//! kahvi-serve APP.mjs: an HTTP server whose request handling is JavaScript
//! running on kahvi.
//!
//! It is kahvi-run's host (console, timers, modules and packages from
//! node_modules, process.env) plus `Headers`, `Request`, `Response` and
//! `Kahvi.serve(handler)` / `Kahvi.readFile(path)` (web.js). The app module
//! calls `Kahvi.serve` with a fetch handler, as a Hono app's `fetch` is.
//!
//! One thread, one request at a time: each connection carries one HTTP/1.1
//! request (`Connection: close`). The handler's promise is driven by the
//! job queue and timers until it settles.

// Parsing and the heap allocate many small blocks; mimalloc is kahvi's allocator elsewhere too.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kahvi::{JsError, Object, Persistent, Runtime, Value};
use kahvi_core::object::typed_array::{
    allocate_array_buffer, array_buffer_data, array_buffer_data_mut,
};
use kahvi_core::prelude::*;
use kahvi_runtime::timers::{self, Timers};
use kahvi_runtime::{clone, console, inspect, loader, process};

const MAX_HEAD: usize = 64 * 1024;
const MAX_BODY: usize = 1024 * 1024;

#[derive(Default)]
struct Shared {
    handler: Option<Persistent>,
    response: Option<(u16, String, String, Vec<u8>)>,
}

struct Server {
    rt: Runtime,
    timers: Arc<Mutex<Timers>>,
    shared: Arc<Mutex<Shared>>,
    dispatch: Persistent,
}

fn new_buffer(agent: &mut Agent, bytes: Vec<u8>) -> JsResult<Value> {
    let ctor = agent.intrinsic("%ArrayBuffer%");
    let buffer = allocate_array_buffer(agent, ctor, bytes.len() as f64, None)?;
    array_buffer_data_mut(agent, buffer)
        .expect("an ArrayBuffer")
        .data = Some(bytes.into());
    Ok(Value::Object(buffer))
}

fn string_arg(agent: &mut Agent, args: &[Value], i: usize) -> JsResult<String> {
    let s = ops::to_string(agent, args.get_or_undefined(i))?;
    Ok(agent.str_lossy(s))
}

impl Server {
    fn new() -> Server {
        // As kahvi-run's Host::new.
        let mut rt = Runtime::new();
        let agent = rt.agent();
        agent.host_now = Some(kahvi_runtime::now_ms);
        agent.host_load_module = Some(loader::load_module);
        agent.host_import_meta = Some(loader::import_meta);
        let timers = Arc::new(Mutex::new(Timers::new()));
        console::install(&mut rt);
        timers::install(&mut rt, &timers);
        clone::install(&mut rt);
        kahvi::encoding::install(&mut rt);
        process::install(&mut rt);
        let global = rt.global();
        rt.set_global("globalThis", Value::Object(global));

        let shared = Arc::new(Mutex::new(Shared::default()));
        let s = shared.clone();
        let register = rt.function("register", 1, move |agent, _, args| {
            let handler = agent.persist(args.get_or_undefined(0));
            s.lock().unwrap().handler = Some(handler);
            Ok(Value::Undefined)
        });
        let s = shared.clone();
        let respond = rt.function("respond", 4, move |agent, _, args| {
            let status = ops::to_number(agent, args.get_or_undefined(0))? as u16;
            let reason = string_arg(agent, args, 1)?;
            let head = string_arg(agent, args, 2)?;
            let body = match args.get_or_undefined(3) {
                Value::Object(o) => array_buffer_data(agent, o)
                    .and_then(|d| d.data.as_deref())
                    .unwrap_or(&[])
                    .to_vec(),
                _ => Vec::new(),
            };
            s.lock().unwrap().response = Some((status, reason, head, body));
            Ok(Value::Undefined)
        });
        let read_file = rt.function("readFile", 1, |agent, _, args| {
            let path = string_arg(agent, args, 0)?;
            match std::fs::read(&path) {
                Ok(bytes) => new_buffer(agent, bytes),
                Err(e) => {
                    Err(agent.throw_error(ErrorKind::Error, &format!("readFile {path}: {e}")))
                }
            }
        });
        let install = rt
            .eval_script(include_str!("web.js"), "kahvi-serve:web.js")
            .expect("web.js evaluates");
        let global = Value::Object(rt.global());
        let dispatch = rt
            .call(
                install,
                Value::Undefined,
                &[
                    global,
                    Value::Object(register),
                    Value::Object(respond),
                    Value::Object(read_file),
                ],
            )
            .expect("web.js installs");
        let dispatch = rt.persist(dispatch);
        Server {
            rt,
            timers,
            shared,
            dispatch,
        }
    }

    fn describe(&mut self, e: &JsError) -> String {
        match e {
            JsError::Throw(v) => {
                let v = *v;
                self.rt.enter(|agent| inspect::format_arg(agent, v))
            }
            JsError::NotImplemented(_) => self.rt.describe_error(e),
        }
    }

    /// Runs promise jobs, then due timers, until `done` or nothing is left
    /// to run. Uncaught errors are reported and the loop carries on.
    /// `entry` is the app module's promise while it loads.
    fn drive(&mut self, entry: Option<Object>, done: impl Fn(&mut Self) -> bool) {
        loop {
            if let Err(e) = self.rt.run_jobs() {
                let msg = self.describe(&e);
                eprintln!("Uncaught {msg}");
            }
            // The app module's own rejection is reported by `load`.
            for p in self
                .rt
                .take_unhandled_rejections()
                .into_iter()
                .filter(|&p| Some(p) != entry)
            {
                let reason = match self.rt.promise_result(p) {
                    Some(Err(r)) => r,
                    _ => Value::Undefined,
                };
                let msg = self.describe(&JsError::Throw(reason));
                eprintln!("Uncaught (in promise) {msg}");
            }
            if done(self) {
                return;
            }
            let wait = self.timers.lock().unwrap().wait();
            let Some(wait) = wait else { return };
            if !wait.is_zero() {
                std::thread::sleep(wait);
            }
            let due = self.timers.lock().unwrap().take_due();
            if let Some((callback, args)) = due {
                let f = self.rt.persisted(&callback);
                let args: Vec<Value> = args.iter().map(|a| self.rt.persisted(a)).collect();
                if let Err(e) = self.rt.call(f, Value::Undefined, &args) {
                    let msg = self.describe(&e);
                    eprintln!("Uncaught {msg}");
                }
            }
        }
    }

    /// Evaluates the app module; it must call Kahvi.serve.
    fn load(&mut self, path: &Path) -> Result<(), String> {
        let key = path
            .canonicalize()
            .map_err(|e| format!("{}: {e}", path.display()))?;
        let source =
            std::fs::read_to_string(&key).map_err(|e| format!("{}: {e}", path.display()))?;
        let entry = match self.rt.eval_module(&key.to_string_lossy(), &source) {
            Ok(p) => p,
            Err(e) => return Err(format!("Uncaught {}", self.describe(&e))),
        };
        self.drive(Some(entry), |s| s.rt.promise_result(entry).is_some());
        match self.rt.promise_result(entry) {
            Some(Ok(_)) => {}
            Some(Err(reason)) => {
                return Err(format!(
                    "Uncaught {}",
                    self.describe(&JsError::Throw(reason))
                ));
            }
            None => return Err("the app module's top-level await never settled".into()),
        }
        if self.shared.lock().unwrap().handler.is_none() {
            return Err("the app module did not call Kahvi.serve".into());
        }
        Ok(())
    }

    /// Runs the handler for one request. Returns (status, reason, head, body).
    fn handle(
        &mut self,
        method: &str,
        url: &str,
        head: &str,
        body: Vec<u8>,
    ) -> (u16, String, String, Vec<u8>) {
        let handler = {
            let shared = self.shared.lock().unwrap();
            self.rt
                .persisted(shared.handler.as_ref().expect("a handler"))
        };
        let dispatch = self.rt.persisted(&self.dispatch);
        let args = self.rt.enter(|agent| -> JsResult<Vec<Value>> {
            Ok(vec![
                handler,
                Value::String(agent.string(method)),
                Value::String(agent.string(url)),
                Value::String(agent.string(head)),
                new_buffer(agent, body)?,
            ])
        });
        let called = args.and_then(|args| self.rt.call(dispatch, Value::Undefined, &args));
        if let Err(e) = called {
            let msg = self.describe(&e);
            eprintln!("kahvi-serve: dispatch failed: {msg}");
        } else {
            self.drive(None, |s| s.shared.lock().unwrap().response.is_some());
        }
        self.shared
            .lock()
            .unwrap()
            .response
            .take()
            .unwrap_or_else(|| {
                eprintln!("kahvi-serve: the handler's response never settled");
                (
                    500,
                    String::new(),
                    String::new(),
                    b"Internal Server Error\n".to_vec(),
                )
            })
    }
}

struct Incoming {
    method: String,
    target: String,
    head: String,
    host: String,
    proto: String,
    body: Vec<u8>,
}

fn read_request(stream: &TcpStream) -> Result<Incoming, &'static str> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    let mut total = 0;
    let mut next_line = |line: &mut String| -> Result<(), &'static str> {
        line.clear();
        let n = (&mut reader)
            .take((MAX_HEAD - total) as u64)
            .read_line(line)
            .map_err(|_| "read error")?;
        total += n;
        if n == 0 || !line.ends_with('\n') {
            return Err("truncated request head");
        }
        Ok(())
    };
    next_line(&mut line)?;
    let mut parts = line.split_whitespace();
    let (Some(method), Some(target), Some(_version)) = (parts.next(), parts.next(), parts.next())
    else {
        return Err("bad request line");
    };
    let (method, target) = (method.to_string(), target.to_string());
    let mut head = String::new();
    let (mut host, mut proto, mut length) =
        (String::from("localhost"), String::from("http"), 0usize);
    loop {
        next_line(&mut line)?;
        let l = line.trim_end_matches(['\r', '\n']);
        if l.is_empty() {
            break;
        }
        let Some((name, value)) = l.split_once(':') else {
            return Err("bad header");
        };
        let value = value.trim();
        match name.to_ascii_lowercase().as_str() {
            "host" => host = value.to_string(),
            "x-forwarded-proto" => proto = value.to_string(),
            "content-length" => length = value.parse().map_err(|_| "bad content-length")?,
            "transfer-encoding" => return Err("chunked request bodies are not supported"),
            _ => {}
        }
        head.push_str(l);
        head.push_str("\r\n");
    }
    if length > MAX_BODY {
        return Err("request body too large");
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body).map_err(|_| "truncated body")?;
    Ok(Incoming {
        method,
        target,
        head,
        host,
        proto,
        body,
    })
}

fn reason_phrase(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        413 => "Content Too Large",
        500 => "Internal Server Error",
        _ => "",
    }
}

fn write_response(
    mut stream: &TcpStream,
    method: &str,
    (status, reason, head, body): (u16, String, String, Vec<u8>),
) {
    let reason = if reason.is_empty() {
        reason_phrase(status).to_string()
    } else {
        reason
    };
    let mut out = format!("HTTP/1.1 {status} {reason}\r\n");
    for line in head.split("\r\n").filter(|l| !l.is_empty()) {
        let name = line.split(':').next().unwrap_or("").to_ascii_lowercase();
        if name != "content-length" && name != "connection" && name != "transfer-encoding" {
            out.push_str(line);
            out.push_str("\r\n");
        }
    }
    out.push_str(&format!(
        "content-length: {}\r\nconnection: close\r\n\r\n",
        body.len()
    ));
    let mut bytes = out.into_bytes();
    if method != "HEAD" {
        bytes.extend_from_slice(&body);
    }
    let _ = stream.write_all(&bytes);
    let _ = stream.flush();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, app] = args.as_slice() else {
        eprintln!("usage: kahvi-serve APP.mjs   (listens on $PORT, default 8080)");
        std::process::exit(2);
    };
    // kahvi recurses on the native stack, as in kahvi-run.
    let app = app.clone();
    let main = std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(move || serve(&app))
        .expect("spawning the main thread");
    std::process::exit(main.join().unwrap_or(70));
}

fn serve(app: &str) -> i32 {
    let mut server = Server::new();
    if let Err(e) = server.load(Path::new(app)) {
        eprintln!("kahvi-serve: {e}");
        return 1;
    }
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080u16);
    let listener = match TcpListener::bind(("0.0.0.0", port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("kahvi-serve: cannot listen on port {port}: {e}");
            return 1;
        }
    };
    println!("kahvi-serve: listening on :{port}");
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let _ = stream.set_read_timeout(Some(Duration::from_secs(10)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(30)));
        match read_request(&stream) {
            Ok(req) => {
                let url = format!("{}://{}{}", req.proto, req.host, req.target);
                let res = server.handle(&req.method, &url, &req.head, req.body);
                write_response(&stream, &req.method, res);
            }
            Err(msg) => {
                write_response(
                    &stream,
                    "GET",
                    (
                        400,
                        String::new(),
                        "content-type: text/plain\r\n".into(),
                        format!("{msg}\n").into_bytes(),
                    ),
                );
            }
        }
    }
    0
}
