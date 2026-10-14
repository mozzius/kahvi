//! The `console` object.

use std::io::Write;

use kahvi::{Agent, JsResult, Runtime, Value};
use kahvi_core::prelude::*;

use crate::inspect::{format_arg, inspect};

#[derive(Clone, Copy)]
enum Stream {
    Out,
    Err,
}

pub fn install(rt: &mut Runtime) {
    let console = rt.enter(|agent| {
        let proto = agent.intrinsic("%Object.prototype%");
        agent.ordinary_object_create(Some(proto))
    });
    let methods: [(&str, Stream, &str); 6] = [
        ("log", Stream::Out, ""),
        ("info", Stream::Out, ""),
        ("debug", Stream::Out, ""),
        ("warn", Stream::Err, ""),
        ("error", Stream::Err, ""),
        ("trace", Stream::Err, "Trace: "),
    ];
    for (name, stream, prefix) in methods {
        let f = rt.function(name, 0, move |agent, _, args| {
            let line = format_args(agent, args)?;
            write_line(stream, &format!("{prefix}{line}"));
            Ok(Value::Undefined)
        });
        kahvi::define_hidden(rt.agent(), console, name, Value::Object(f));
    }
    let assert = rt.function("assert", 0, |agent, _, args| {
        if ops::to_boolean(agent, args.get_or_undefined(0)) {
            return Ok(Value::Undefined);
        }
        let rest = args.get(1..).unwrap_or(&[]);
        let line = format_args(agent, rest)?;
        if line.is_empty() {
            write_line(Stream::Err, "Assertion failed");
        } else {
            write_line(Stream::Err, &format!("Assertion failed: {line}"));
        }
        Ok(Value::Undefined)
    });
    kahvi::define_hidden(rt.agent(), console, "assert", Value::Object(assert));
    rt.set_global("console", Value::Object(console));
}

fn write_line(stream: Stream, line: &str) {
    // A closed pipe is not the script's error: drop the output.
    let _ = match stream {
        Stream::Out => writeln!(std::io::stdout().lock(), "{line}"),
        Stream::Err => writeln!(std::io::stderr().lock(), "{line}"),
    };
}

/// The console's formatting of its arguments: a leading string may hold
/// substitutions (`%s %d %i %f %o %O %j %c %%`); the rest are joined with spaces.
fn format_args(agent: &mut Agent, args: &[Value]) -> JsResult<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut rest = args;
    if let Some(&Value::String(s)) = args.first() {
        let fmt = agent.str_lossy(s);
        if fmt.contains('%') {
            let mut out = String::new();
            let mut next = 1;
            let mut chars = fmt.chars().peekable();
            while let Some(c) = chars.next() {
                if c != '%' {
                    out.push(c);
                    continue;
                }
                let Some(&spec) = chars.peek() else {
                    out.push('%');
                    break;
                };
                if spec == '%' {
                    chars.next();
                    out.push('%');
                    continue;
                }
                if !"sdifoOjc".contains(spec) || next >= args.len() {
                    out.push('%');
                    continue;
                }
                chars.next();
                let v = args[next];
                next += 1;
                match spec {
                    's' => out.push_str(&format_arg(agent, v)),
                    'd' | 'i' => {
                        let n = match v {
                            Value::Object(_) | Value::Symbol(_) => f64::NAN,
                            Value::BigInt(_) => {
                                out.push_str(&inspect(agent, v));
                                continue;
                            }
                            _ => ops::to_number(agent, v)?,
                        };
                        let n = if spec == 'i' { n.trunc() } else { n };
                        out.push_str(&inspect(agent, Value::Number(n)));
                    }
                    'f' => {
                        let n = match v {
                            Value::Symbol(_) => f64::NAN,
                            _ => ops::to_number(agent, v)?,
                        };
                        out.push_str(&inspect(agent, Value::Number(n)));
                    }
                    'j' => {
                        let json = agent.intrinsic("%JSON%");
                        let k = agent.key("stringify");
                        let stringify = ops::get(agent, json, k)?;
                        match ops::call(agent, stringify, Value::Object(json), &[v])? {
                            Value::String(s) => out.push_str(&agent.str_lossy(s)),
                            _ => out.push_str("undefined"),
                        }
                    }
                    'c' => {}
                    _ => out.push_str(&inspect(agent, v)),
                }
            }
            parts.push(out);
            rest = &args[next.min(args.len())..];
        } else {
            parts.push(fmt);
            rest = &args[1..];
        }
    }
    for &v in rest {
        parts.push(format_arg(agent, v));
    }
    Ok(parts.join(" "))
}
