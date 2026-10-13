//! `kahvi` command-line entry point.
//!
//! The command-line contract the test262 runner depends on is documented in
//! `docs/ENGINE-CLI.md`.

mod host;

use std::process::ExitCode;

use kahvi_core::prelude::*;
use kahvi_core::vm;

struct Args {
    preludes: Vec<String>,
    module: bool,
    parse_only: bool,
    file: String,
}

fn parse_args() -> Result<Args, String> {
    let mut preludes = Vec::new();
    let mut module = false;
    let mut parse_only = false;
    let mut file = None;
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--prelude" => preludes.push(it.next().ok_or("--prelude needs a file")?),
            "--module" => module = true,
            "--parse-only" => parse_only = true,
            "--version" => {
                println!("kahvi {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option {arg}")),
            _ if file.is_none() => file = Some(arg),
            _ => return Err("only one script file may be given".into()),
        }
    }
    let file = file.ok_or("usage: kahvi [--prelude FILE]... [--module] [--parse-only] FILE")?;
    Ok(Args {
        preludes,
        module,
        parse_only,
        file,
    })
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(args) => args,
        Err(msg) => {
            eprintln!("kahvi: {msg}");
            return ExitCode::from(2);
        }
    };
    for path in args.preludes.iter().chain(std::iter::once(&args.file)) {
        if let Err(err) = std::fs::read(path) {
            eprintln!("kahvi: cannot read {path}: {err}");
            return ExitCode::from(2);
        }
    }
    if args.parse_only {
        return parse_only(&args);
    }
    // Deep JavaScript recursion recurses on the Rust stack.
    let runner = std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(move || run(&args));
    match runner.map(|h| h.join()) {
        Ok(Ok(code)) => ExitCode::from(code),
        _ => ExitCode::from(101),
    }
}

/// Evaluates the preludes and the file in one realm, then drains the job queue.
fn run(args: &Args) -> u8 {
    let mut agent = Agent::new();
    agent.set_intrinsics_installer(kahvi_builtins::install);
    agent.host_now = Some(host::now_ms);
    let realm = match host::new_realm(&mut agent) {
        Ok(r) => r,
        Err(e) => return report(&mut agent, e),
    };
    if args.module {
        return report(&mut agent, JsError::NotImplemented("modules".into()));
    }
    for path in args.preludes.iter().chain(std::iter::once(&args.file)) {
        let source = match std::fs::read(path) {
            Ok(b) => String::from_utf8_lossy(&b).into_owned(),
            Err(err) => {
                eprintln!("kahvi: cannot read {path}: {err}");
                return 2;
            }
        };
        let result = vm::parse_script(&mut agent, &source, path, realm)
            .and_then(|s| vm::evaluate_script(&mut agent, &s));
        if let Err(e) = result {
            return report(&mut agent, e);
        }
    }
    agent.contexts.push(kahvi_core::agent::ExecutionContext {
        function: None,
        realm,
        script_or_module: None,
    });
    let jobs = agent.run_jobs();
    agent.contexts.pop();
    match jobs {
        Ok(()) => 0,
        Err(e) => report(&mut agent, e),
    }
}

/// Prints `Uncaught <Name>: <message>` for an uncaught exception.
fn report(agent: &mut Agent, e: JsError) -> u8 {
    match e {
        JsError::NotImplemented(what) => {
            eprintln!("Uncaught InternalError: not implemented: {what}")
        }
        JsError::Throw(Value::Object(o)) => {
            let name = error_name(agent, o).unwrap_or_else(|| "Error".into());
            let mk = agent.key("message");
            let message = ops::get(agent, o, mk)
                .ok()
                .filter(|v| !v.is_undefined())
                .and_then(|v| ops::to_string(agent, v).ok())
                .map(|s| agent.str_lossy(s))
                .unwrap_or_default();
            eprintln!("Uncaught {name}: {message}");
        }
        JsError::Throw(v) => {
            let s = ops::to_string(agent, v)
                .map(|s| agent.str_lossy(s))
                .unwrap_or_else(|_| "exception".into());
            eprintln!("Uncaught {s}");
        }
    }
    1
}

/// The `name` of the thrown object's constructor, else its own `name`.
fn error_name(agent: &mut Agent, o: Object) -> Option<String> {
    let ck = PropertyKey::String(agent.names.constructor);
    let nk = PropertyKey::String(agent.names.name);
    if let Ok(Value::Object(c)) = ops::get(agent, o, ck)
        && let Ok(Value::String(s)) = ops::get(agent, c, nk)
    {
        return Some(agent.str_lossy(s));
    }
    match ops::get(agent, o, nk) {
        Ok(Value::String(s)) => Some(agent.str_lossy(s)),
        _ => None,
    }
}

/// `--parse-only`: report early errors without running anything.
fn parse_only(args: &Args) -> ExitCode {
    let bytes = match std::fs::read(&args.file) {
        Ok(b) => b,
        Err(err) => {
            eprintln!("kahvi: cannot read {}: {err}", args.file);
            return ExitCode::from(2);
        }
    };
    let source = String::from_utf8_lossy(&bytes);
    let result = if args.module {
        kahvi_parser::parse_module(&source).map(|_| ())
    } else {
        kahvi_parser::parse_script(&source, &kahvi_parser::ParseOptions::default()).map(|_| ())
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let (line, col) = err.line_col(&source);
            eprintln!(
                "Uncaught SyntaxError: {} ({}:{line}:{col})",
                err.message, args.file
            );
            ExitCode::from(1)
        }
    }
}
