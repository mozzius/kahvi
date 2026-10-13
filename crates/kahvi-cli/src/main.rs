//! `kahvi` command-line entry point.
//!
//! The command-line contract the test262 runner depends on is documented in
//! `docs/ENGINE-CLI.md`. Until the engine exists (M1) every script fails with
//! an `InternalError: not implemented` report, which the runner counts as
//! "not implemented" rather than as an ordinary failure.

use std::process::ExitCode;

struct Args {
    preludes: Vec<String>,
    module: bool,
    file: String,
}

fn parse_args() -> Result<Args, String> {
    let mut preludes = Vec::new();
    let mut module = false;
    let mut file = None;
    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--prelude" => preludes.push(it.next().ok_or("--prelude needs a file")?),
            "--module" => module = true,
            "--version" => {
                println!("kahvi {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            _ if arg.starts_with('-') => return Err(format!("unknown option {arg}")),
            _ if file.is_none() => file = Some(arg),
            _ => return Err("only one script file may be given".into()),
        }
    }
    let file = file.ok_or("usage: kahvi [--prelude FILE]... [--module] FILE")?;
    Ok(Args {
        preludes,
        module,
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
    let _ = args.module;
    eprintln!("Uncaught InternalError: not implemented: engine");
    ExitCode::from(1)
}
