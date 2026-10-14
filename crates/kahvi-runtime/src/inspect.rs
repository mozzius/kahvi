//! Formatting values for the console (in the spirit of Node's
//! `util.inspect`). It reads internal slots and own properties directly and
//! never runs JavaScript: getters are shown, not called, and proxies are not
//! looked through.

use kahvi_builtins::introspect::{self, Kind};
use kahvi_core::object::proxy::ProxyData;
use kahvi_core::ops::promise::promise_result;
use kahvi_core::prelude::*;

/// How deep nested objects are shown before they become `[Object]`.
const MAX_DEPTH: usize = 2;
/// Lines longer than this are broken, one entry per line.
const LINE_WIDTH: usize = 80;
/// Array elements and collection entries shown before "... N more items".
const MAX_ITEMS: usize = 100;

/// Formats `v` as a console argument: strings as they are, everything else
/// inspected.
pub fn format_arg(agent: &mut Agent, v: Value) -> String {
    match v {
        Value::String(s) => agent.str_lossy(s),
        _ => inspect(agent, v),
    }
}

/// `util.inspect(v)`.
pub fn inspect(agent: &mut Agent, v: Value) -> String {
    let mut f = Formatter {
        agent,
        seen: Vec::new(),
    };
    f.value(v, 0)
}

struct Formatter<'a> {
    agent: &'a mut Agent,
    /// The objects being formatted (for cycles).
    seen: Vec<Object>,
}

impl Formatter<'_> {
    fn value(&mut self, v: Value, depth: usize) -> String {
        match v {
            Value::Undefined => "undefined".into(),
            Value::Null => "null".into(),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => number(self.agent, n),
            Value::BigInt(b) => {
                let s = kahvi_core::ops::bigint::to_js_string(self.agent, b, 10);
                format!("{}n", self.agent.str_lossy(s))
            }
            Value::String(s) => quote(&self.agent.str_lossy(s)),
            Value::Symbol(s) => symbol(self.agent, s),
            Value::Object(o) => self.object(o, depth),
        }
    }

    fn object(&mut self, o: Object, depth: usize) -> String {
        if self.seen.contains(&o) {
            return "[Circular]".into();
        }
        if let Some(p) = self.agent.native::<ProxyData>(o) {
            return if p.callable {
                "[Function: Proxy]".into()
            } else {
                "Proxy [Object]".into()
            };
        }
        let callable = ops::is_callable(self.agent, Value::Object(o));
        let is_array = matches!(self.agent.obj(o).kind, ObjectKind::Array);
        // The header (what kind of object) and the brackets around its entries.
        let (header, open, close, mut entries, show_keys) = if callable {
            (function_header(self.agent, o), "{", "}", Vec::new(), true)
        } else if is_array {
            if depth > MAX_DEPTH {
                return "[Array]".into();
            }
            let prefix = self.class_prefix(o, "Array");
            self.seen.push(o);
            let items = self.array_items(o, depth);
            self.seen.pop();
            (prefix, "[", "]", items, false)
        } else {
            match self.special(o, depth) {
                Some(Special::Done(s)) => return s,
                Some(Special::Braced(header, entries)) => (header, "{", "}", entries, true),
                None => {
                    if depth > MAX_DEPTH {
                        return "[Object]".into();
                    }
                    (self.class_prefix(o, "Object"), "{", "}", Vec::new(), true)
                }
            }
        };
        if callable && depth > MAX_DEPTH {
            return header;
        }
        self.seen.push(o);
        let props = self.properties(o, depth, !show_keys);
        self.seen.pop();
        entries.extend(props);
        if entries.is_empty() {
            return if callable {
                header
            } else if header.is_empty() {
                format!("{open}{close}")
            } else {
                format!("{header} {open}{close}")
            };
        }
        let one_line = format!("{open} {} {close}", entries.join(", "));
        let body = if one_line.len() + header.len() <= LINE_WIDTH && !one_line.contains('\n') {
            one_line
        } else {
            let indent = "  ";
            let inner: Vec<String> = entries
                .iter()
                .map(|e| format!("{indent}{}", e.replace('\n', &format!("\n{indent}"))))
                .collect();
            format!("{open}\n{}\n{close}", inner.join(",\n"))
        };
        if header.is_empty() {
            body
        } else {
            format!("{header} {body}")
        }
    }

    /// `Foo ` for an instance of a class other than `default`, `[Object:
    /// null prototype]` without a prototype, else nothing.
    fn class_prefix(&mut self, o: Object, default: &str) -> String {
        let Some(proto) = self.agent.obj(o).prototype else {
            return format!("[{default}: null prototype]");
        };
        match constructor_name(self.agent, proto) {
            Some(name) if name != default => name,
            _ => String::new(),
        }
    }

    fn array_items(&mut self, o: Object, depth: usize) -> Vec<String> {
        let len_key = self.agent.key("length");
        let len = match self.own_data(o, len_key) {
            Some(Value::Number(n)) => n as u64,
            _ => 0,
        };
        let mut out = Vec::new();
        let mut holes = 0u64;
        let shown = len.min(MAX_ITEMS as u64);
        for i in 0..shown {
            match self.own_data(o, PropertyKey::Index(i as u32)) {
                Some(v) => {
                    if holes > 0 {
                        out.push(empty_items(holes));
                        holes = 0;
                    }
                    out.push(self.value(v, depth + 1));
                }
                None => holes += 1,
            }
        }
        if holes > 0 {
            out.push(empty_items(holes));
        }
        if len > shown {
            out.push(format!(
                "... {} more item{}",
                len - shown,
                plural(len - shown)
            ));
        }
        out
    }

    /// Objects with internal slots worth showing.
    fn special(&mut self, o: Object, depth: usize) -> Option<Special> {
        let prim = match &self.agent.obj(o).kind {
            ObjectKind::Error => return Some(Special::Done(self.error(o))),
            ObjectKind::BooleanObject(b) => Some(("Boolean", Value::Boolean(*b))),
            ObjectKind::NumberObject(n) => Some(("Number", Value::Number(*n))),
            ObjectKind::StringObject(s) => Some(("String", Value::String(*s))),
            ObjectKind::SymbolObject(s) => Some(("Symbol", Value::Symbol(*s))),
            ObjectKind::BigIntObject(b) => Some(("BigInt", Value::BigInt(*b))),
            _ => None,
        };
        if let Some((label, prim)) = prim {
            let p = self.value(prim, depth + 1);
            return Some(Special::Done(format!("[{label}: {p}]")));
        }
        if let Some(r) = promise_result(self.agent, o) {
            let state = match r {
                Ok(v) => self.value(v, depth + 1),
                Err(e) => format!("<rejected> {}", self.value(e, depth + 1)),
            };
            return Some(Special::Braced("Promise".into(), vec![state]));
        }
        if self
            .agent
            .native::<kahvi_core::ops::promise::PromiseData>(o)
            .is_some()
        {
            return Some(Special::Braced("Promise".into(), vec!["<pending>".into()]));
        }
        match introspect::kind(self.agent, o) {
            Kind::Map(entries) => {
                if depth > MAX_DEPTH {
                    return Some(Special::Done("[Map]".into()));
                }
                self.seen.push(o);
                let n = entries.len();
                let items = entries
                    .into_iter()
                    .take(MAX_ITEMS)
                    .map(|(k, v)| {
                        let k = self.value(k, depth + 1);
                        let v = self.value(v, depth + 1);
                        format!("{k} => {v}")
                    })
                    .collect();
                self.seen.pop();
                Some(Special::Braced(format!("Map({n})"), items))
            }
            Kind::Set(values) => {
                if depth > MAX_DEPTH {
                    return Some(Special::Done("[Set]".into()));
                }
                self.seen.push(o);
                let n = values.len();
                let items = values
                    .into_iter()
                    .take(MAX_ITEMS)
                    .map(|v| self.value(v, depth + 1))
                    .collect();
                self.seen.pop();
                Some(Special::Braced(format!("Set({n})"), items))
            }
            Kind::Date(t) => Some(Special::Done(if t.is_nan() {
                "Invalid Date".into()
            } else {
                iso_date(t)
            })),
            Kind::RegExp(source, flags) => Some(Special::Done(format!(
                "/{}/{}",
                self.agent.str_lossy(source),
                self.agent.str_lossy(flags)
            ))),
            Kind::Other => None,
        }
    }

    /// `Name: message`, plus the error's own properties other than the usual ones.
    fn error(&mut self, o: Object) -> String {
        let stack_key = self.agent.key("stack");
        if let Some(Value::String(s)) = self.own_data(o, stack_key) {
            return self.agent.str_lossy(s);
        }
        kahvi::describe_thrown(self.agent, Value::Object(o))
    }

    /// The own enumerable properties as `key: value` entries (the indices
    /// too unless `skip_indices`).
    fn properties(&mut self, o: Object, depth: usize, skip_indices: bool) -> Vec<String> {
        let Ok(keys) = o.internal_own_property_keys(self.agent) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for k in keys {
            if skip_indices && matches!(k, PropertyKey::Index(_)) {
                continue;
            }
            let Ok(Some(d)) = o.internal_get_own_property(self.agent, k) else {
                continue;
            };
            if d.enumerable != Some(true) {
                continue;
            }
            let name = key_name(self.agent, k);
            let value = match (d.value, d.get, d.set) {
                (Some(v), _, _) => self.value(v, depth + 1),
                (None, Some(g), Some(s)) if !g.is_undefined() && !s.is_undefined() => {
                    "[Getter/Setter]".into()
                }
                (None, Some(g), _) if !g.is_undefined() => "[Getter]".into(),
                (None, _, Some(s)) if !s.is_undefined() => "[Setter]".into(),
                _ => "undefined".into(),
            };
            out.push(format!("{name}: {value}"));
        }
        out
    }

    /// An own data property's value, if `o` has one.
    fn own_data(&mut self, o: Object, k: PropertyKey) -> Option<Value> {
        match o.internal_get_own_property(self.agent, k) {
            Ok(Some(d)) => d.value,
            _ => None,
        }
    }
}

enum Special {
    /// The whole formatted value.
    Done(String),
    /// A header and entries shown in braces (with the own properties after them).
    Braced(String, Vec<String>),
}

fn plural(n: u64) -> &'static str {
    if n == 1 { "" } else { "s" }
}

fn empty_items(n: u64) -> String {
    format!("<{n} empty item{}>", plural(n))
}

fn number(agent: &mut Agent, n: f64) -> String {
    if n == 0.0 && n.is_sign_negative() {
        return "-0".into();
    }
    let s = ops::number_to_js_string(agent, n);
    agent.str_lossy(s)
}

fn symbol(agent: &Agent, s: Symbol) -> String {
    match agent.symbol_description(s) {
        Some(d) => format!("Symbol({})", agent.str_lossy(d)),
        None => "Symbol()".into(),
    }
}

/// A string as a single-quoted literal.
fn quote(s: &str) -> String {
    let q = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(q);
    for c in s.chars() {
        match c {
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '\\' => out.push_str("\\\\"),
            c if c == q => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 => out.push_str(&format!("\\x{:02X}", c as u32)),
            c => out.push(c),
        }
    }
    out.push(q);
    out
}

/// A property key as shown in an object: bare if it is an identifier.
fn key_name(agent: &Agent, k: PropertyKey) -> String {
    match k {
        PropertyKey::Index(i) => format!("'{i}'"),
        PropertyKey::String(s) => {
            let s = agent.str_lossy(s);
            let ident = !s.is_empty()
                && s.chars().enumerate().all(|(i, c)| {
                    c == '_' || c == '$' || c.is_alphabetic() || (i > 0 && c.is_ascii_digit())
                });
            if ident { s } else { quote(&s) }
        }
        PropertyKey::Symbol(s) => format!("[{}]", symbol(agent, s)),
    }
}

/// `[Function: name]`, `[class Name]` or `[Function (anonymous)]`.
fn function_header(agent: &mut Agent, f: Object) -> String {
    let name_key = agent.key("name");
    let name = match f.internal_get_own_property(agent, name_key) {
        Ok(Some(d)) => match d.value {
            Some(Value::String(s)) => agent.str_lossy(s),
            _ => String::new(),
        },
        _ => String::new(),
    };
    let is_class = match &agent.obj(f).kind {
        ObjectKind::Function(ef) => ef.is_class_constructor,
        _ => false,
    };
    match (is_class, name.is_empty()) {
        (true, false) => format!("[class {name}]"),
        (true, true) => "[class (anonymous)]".into(),
        (false, false) => format!("[Function: {name}]"),
        (false, true) => "[Function (anonymous)]".into(),
    }
}

/// The `name` of `proto.constructor`, read from own data properties only.
fn constructor_name(agent: &mut Agent, proto: Object) -> Option<String> {
    let ck = agent.key("constructor");
    let nk = agent.key("name");
    let c = match proto.internal_get_own_property(agent, ck) {
        Ok(Some(d)) => d.value?,
        _ => return None,
    };
    let Value::Object(c) = c else { return None };
    match c.internal_get_own_property(agent, nk) {
        Ok(Some(d)) => match d.value? {
            Value::String(s) => Some(agent.str_lossy(s)),
            _ => None,
        },
        _ => None,
    }
}

/// Date.prototype.toISOString's format.
fn iso_date(t: f64) -> String {
    let ms_per_day = 86_400_000.0;
    let day = (t / ms_per_day).floor();
    let time = t - day * ms_per_day;
    // Days since 1970-01-01 to a civil date (Howard Hinnant's algorithm).
    let z = day as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    let time = time as i64;
    let (h, mi, s, ms) = (
        time / 3_600_000,
        time / 60_000 % 60,
        time / 1000 % 60,
        time % 1000,
    );
    let year = if (0..=9999).contains(&y) {
        format!("{y:04}")
    } else if y < 0 {
        format!("-{:06}", -y)
    } else {
        format!("+{y:06}")
    };
    format!("{year}-{m:02}-{d:02}T{h:02}:{mi:02}:{s:02}.{ms:03}Z")
}
