//! `kahvi-run FILE`: runs an ES module and its event loop.

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let (Some(file), None) = (args.next(), args.next()) else {
        eprintln!("usage: kahvi-run FILE");
        return ExitCode::from(kahvi_runtime::EXIT_USAGE);
    };
    // Deep JavaScript recursion recurses on the Rust stack.
    let runner = std::thread::Builder::new()
        .stack_size(512 << 20)
        .spawn(move || kahvi_runtime::Host::new().run_main(std::path::Path::new(&file)));
    match runner.map(|h| h.join()) {
        Ok(Ok(code)) => ExitCode::from(code),
        _ => ExitCode::from(101),
    }
}
