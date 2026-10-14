//! Running the engine as a child process with a timeout, an empty
//! environment and bounded output capture.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Bytes of stdout/stderr kept per stream; the rest is drained and dropped.
const CAPTURE_LIMIT: usize = 64 * 1024;

#[derive(Clone)]
pub struct Engine {
    /// Resolved program path plus fixed leading arguments.
    pub program: PathBuf,
    pub args: Vec<String>,
    pub timeout: Duration,
    /// Virtual memory cap in MiB (0 = none). Applied with `ulimit -v`.
    pub mem_limit_mb: u64,
}

pub enum Exit {
    Code(i32),
    Signal,
    Timeout,
}

pub struct Output {
    pub exit: Exit,
    pub stdout: String,
    pub stderr: String,
    pub millis: u64,
}

impl Output {
    pub fn is_crash(&self) -> bool {
        match self.exit {
            Exit::Signal => true,
            Exit::Code(c) => c == 101 || c >= 128,
            Exit::Timeout => false,
        }
    }
}

/// Finds `name` on PATH (unless it already contains a slash).
pub fn resolve_program(name: &str) -> Option<PathBuf> {
    if name.contains('/') {
        let p = PathBuf::from(name);
        return p.exists().then(|| p.canonicalize().unwrap_or(p));
    }
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|p| p.is_file())
}

fn capture<R: Read + Send + 'static>(mut r: R) -> std::thread::JoinHandle<Vec<u8>> {
    std::thread::spawn(move || {
        let mut kept = Vec::new();
        let mut buf = [0u8; 8192];
        loop {
            match r.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    let room = CAPTURE_LIMIT.saturating_sub(kept.len());
                    kept.extend_from_slice(&buf[..n.min(room)]);
                }
            }
        }
        kept
    })
}

impl Engine {
    pub fn run(&self, extra_args: &[String], cwd: &Path) -> Output {
        let mut cmd = if self.mem_limit_mb > 0 {
            let mut c = Command::new("/bin/sh");
            c.arg("-c")
                .arg("ulimit -v \"$0\" && exec \"$@\"")
                .arg((self.mem_limit_mb * 1024).to_string())
                .arg(&self.program);
            c
        } else {
            Command::new(&self.program)
        };
        cmd.args(&self.args)
            .args(extra_args)
            .current_dir(cwd)
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // The engine runs with an empty environment, except GC stress mode:
        // KAHVI_GC_STRESS set for the runner makes every engine run collect at
        // each safe point (docs/GC.md).
        if std::env::var_os("KAHVI_GC_STRESS").is_some() {
            cmd.env("KAHVI_GC_STRESS", "1");
        }

        let start = Instant::now();
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                return Output {
                    exit: Exit::Code(2),
                    stdout: String::new(),
                    stderr: format!("t262: failed to spawn engine: {e}"),
                    millis: 0,
                };
            }
        };
        let out = capture(child.stdout.take().expect("piped stdout"));
        let err = capture(child.stderr.take().expect("piped stderr"));

        let mut nap = Duration::from_micros(200);
        let exit = loop {
            match child.try_wait() {
                Ok(Some(status)) => {
                    break match status.code() {
                        Some(c) => Exit::Code(c),
                        None => Exit::Signal,
                    };
                }
                Ok(None) if start.elapsed() >= self.timeout => {
                    let _ = child.kill();
                    let _ = child.wait();
                    break Exit::Timeout;
                }
                Ok(None) => {
                    std::thread::sleep(nap);
                    nap = (nap * 2).min(Duration::from_millis(10));
                }
                Err(_) => break Exit::Signal,
            }
        };
        let millis = start.elapsed().as_millis() as u64;
        let stdout = String::from_utf8_lossy(&out.join().unwrap_or_default()).into_owned();
        let stderr = String::from_utf8_lossy(&err.join().unwrap_or_default()).into_owned();
        Output {
            exit,
            stdout,
            stderr,
            millis,
        }
    }
}
