//! Deciding pass/fail from what the engine did.

use crate::exec::{Exit, Output};
use crate::meta::Meta;
use crate::util::truncate;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Pass,
    Fail,
    Timeout,
    Crash,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Pass => "pass",
            Status::Fail => "fail",
            Status::Timeout => "timeout",
            Status::Crash => "crash",
        }
    }
}

pub struct Verdict {
    pub status: Status,
    pub reason: String,
    /// What the engine said it has not implemented, if that is why it failed.
    pub not_implemented: Option<String>,
}

const NOT_IMPLEMENTED: &str = "Uncaught InternalError: not implemented: ";

fn uncaught_line(stderr: &str) -> Option<&str> {
    stderr
        .lines()
        .map(str::trim_end)
        .find(|l| l.starts_with("Uncaught "))
}

/// The constructor name in an `Uncaught <Name>: message` line.
fn error_type(line: &str) -> &str {
    let rest = &line["Uncaught ".len()..];
    let end = rest
        .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '$'))
        .unwrap_or(rest.len());
    &rest[..end]
}

fn crash_reason(out: &Output) -> String {
    let panic = out.stderr.lines().find(|l| l.contains("panicked"));
    let detail = panic
        .or_else(|| out.stderr.lines().rev().find(|l| !l.trim().is_empty()))
        .unwrap_or("");
    let code = match out.exit {
        Exit::Code(c) => format!("exit {c}"),
        _ => "killed by signal".to_string(),
    };
    truncate(&format!("crash ({code}) {detail}"), 300)
}

pub fn classify(meta: &Meta, nonce: Option<&str>, out: &Output, timeout_secs: u64) -> Verdict {
    let fail = |reason: String| {
        let not_implemented = reason
            .strip_prefix(NOT_IMPLEMENTED)
            .map(|what| truncate(what.trim(), 120));
        Verdict {
            status: Status::Fail,
            reason: truncate(&reason, 300),
            not_implemented,
        }
    };
    let code = match out.exit {
        Exit::Timeout => {
            return Verdict {
                status: Status::Timeout,
                reason: format!("timed out after {timeout_secs}s"),
                not_implemented: None,
            };
        }
        _ if out.is_crash() => {
            return Verdict {
                status: Status::Crash,
                reason: crash_reason(out),
                not_implemented: None,
            };
        }
        Exit::Code(c) => c,
        Exit::Signal => unreachable!("signals are crashes"),
    };
    let uncaught = uncaught_line(&out.stderr);
    let pass = Verdict {
        status: Status::Pass,
        reason: String::new(),
        not_implemented: None,
    };

    if let Some(neg) = &meta.negative {
        if code == 0 {
            return fail(format!(
                "expected {} ({} phase) but the script completed normally",
                neg.error_type, neg.phase
            ));
        }
        return match uncaught {
            Some(line) if error_type(line) == neg.error_type => pass,
            Some(line) if line.starts_with(NOT_IMPLEMENTED) => fail(line.to_string()),
            Some(line) => fail(format!("expected {}, got: {line}", neg.error_type)),
            None => fail(format!(
                "expected {}, engine exited {code} without an `Uncaught` line",
                neg.error_type
            )),
        };
    }

    if code != 0 {
        return fail(match uncaught {
            Some(line) => line.to_string(),
            None => format!("exit {code}: {}", out.stderr.trim()),
        });
    }
    if let Some(line) = uncaught {
        return fail(format!("exit 0 but reported: {line}"));
    }
    let lines: Vec<&str> = out.stdout.lines().map(str::trim_end).collect();
    if let Some(nonce) = nonce
        && !lines.contains(&nonce)
    {
        return fail("completion marker missing: the script did not run to the end".into());
    }
    if meta.has_flag("async") {
        if let Some(l) = lines
            .iter()
            .find(|l| l.starts_with("Test262:AsyncTestFailure"))
        {
            return fail(l.to_string());
        }
        if !lines.contains(&"Test262:AsyncTestComplete") {
            return fail("async test never reported completion via $DONE".into());
        }
    }
    pass
}

/// Parse-only mode: a test passes if the engine's parser agrees with the
/// test about whether the source has an early error.
pub fn classify_parse_only(meta: &Meta, out: &Output, timeout_secs: u64) -> Verdict {
    let code = match out.exit {
        Exit::Timeout => {
            return Verdict {
                status: Status::Timeout,
                reason: format!("timed out after {timeout_secs}s"),
                not_implemented: None,
            };
        }
        _ if out.is_crash() => {
            return Verdict {
                status: Status::Crash,
                reason: crash_reason(out),
                not_implemented: None,
            };
        }
        Exit::Code(c) => c,
        Exit::Signal => unreachable!("signals are crashes"),
    };
    let uncaught = uncaught_line(&out.stderr);
    let expect_error = meta.negative.as_ref().filter(|n| n.phase == "parse");
    let pass = Verdict {
        status: Status::Pass,
        reason: String::new(),
        not_implemented: None,
    };
    let fail = |reason: String| Verdict {
        status: Status::Fail,
        reason: truncate(&reason, 300),
        not_implemented: None,
    };
    match (expect_error, code, uncaught) {
        (None, 0, None) => pass,
        (None, _, Some(line)) => fail(format!("should parse: {line}")),
        (None, c, None) => fail(format!("should parse: exit {c}")),
        (Some(neg), 0, _) => fail(format!("expected early {} but it parsed", neg.error_type)),
        (Some(neg), _, Some(line)) if error_type(line) == neg.error_type => pass,
        (Some(neg), _, line) => fail(format!(
            "expected early {}, got: {}",
            neg.error_type,
            line.unwrap_or("no error line")
        )),
    }
}
