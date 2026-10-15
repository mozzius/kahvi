#!/usr/bin/env python3
"""Generates the built-in skeleton of kahvi-builtins from spec.html.

For every built-in function clause (type="built-in function") it emits a
stub file with the spec steps as comments, and a table (src/generated.rs)
that installs every intrinsic object and function property with the right
name, length and attributes.

    python3 harness/gen/builtins.py [--force]

Stub files are only created when missing (they belong to tickets once they
exist); generated.rs and the module lists are always rewritten. --force
rewrites stub files too.
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "harness"))
import spec  # noqa: E402

OUT = ROOT / "crates" / "kahvi-builtins" / "src"

NATIVE_ERRORS = ["EvalError", "RangeError", "ReferenceError", "SyntaxError", "TypeError", "URIError"]
TYPED_ARRAYS = [
    "Int8Array", "Uint8Array", "Uint8ClampedArray", "Int16Array", "Uint16Array", "Int32Array",
    "Uint32Array", "BigInt64Array", "BigUint64Array", "Float16Array", "Float32Array", "Float64Array",
]

# Functions that are not installed as properties anywhere (created by
# abstract operations) or need special handling.
SKIP_OWNERS = {"%ThrowTypeError%"}

HEAD_RE = re.compile(
    r"^(?:(?P<acc>get|set)\s+)?(?P<path>[%\w.]+?)(?:\s*\[\s*%Symbol\.(?P<sym>\w+)%\s*\])?(?:\s*\((?P<params>.*)\))?\s*$"
)


def snake(s):
    s = s.replace("%", "").replace(".", "_").replace("NaN", "Nan")
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", s)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.lower().strip("_")


RUST_KEYWORDS = {"as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if",
                 "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
                 "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
                 "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof",
                 "unsized", "virtual", "yield", "try", "gen"}


def ident(s):
    return f"r#{s}" if s in RUST_KEYWORDS else s


def expected_length(params, body):
    """The "length" property: explicit in the prose, else the count of
    required parameters before the first optional or rest one."""
    # The spec source writes it as `The *"length"* property of this function is *2*<sub>𝔽</sub>.`
    m = re.search(r'(?:\*|<code>)"length"(?:\*|</code>) property of this (?:function|method) is \*\+?(\d+)\*', body)
    if m:
        return int(m.group(1))
    params = spec.strip_tags(params or "").strip()
    if not params:
        return 0
    required = params.split("[")[0]
    n = 0
    for p in required.split(","):
        p = p.strip()
        if not p or p.startswith("..."):
            break
        n += 1
    return n


def attributes(body):
    m = re.search(r"\{\s*\[\[Writable\]\]: \*(true|false)\*, \[\[Enumerable\]\]: \*(true|false)\*, \[\[Configurable\]\]: \*(true|false)\*\s*\}", body)
    if m:
        return tuple(x == "true" for x in m.groups())
    m = re.search(r"\{\s*\[\[Enumerable\]\]: \*(true|false)\*, \[\[Configurable\]\]: \*(true|false)\*\s*\}", body)
    if m:
        return (None,) + tuple(x == "true" for x in m.groups())
    return None


def owner_intrinsic(path):
    """'Array.prototype' -> '%Array.prototype%', '%TypedArray%.prototype' -> '%TypedArray.prototype%'."""
    if not path:
        return "%GlobalObject%"
    p = path.replace("%", "")
    return f"%{p}%"


class Fn:
    def __init__(self, ident_, head, owner, key, sym, acc, length, attrs, body, constructor):
        self.id = ident_
        self.head = head
        self.owner = owner
        self.key = key
        self.sym = sym
        self.acc = acc
        self.length = length
        self.attrs = attrs
        self.body = body
        self.constructor = constructor

    @property
    def display(self):
        o = self.owner.strip("%")
        k = f"[Symbol.{self.sym}]" if self.sym else self.key
        base = k if o == "GlobalObject" else (f"{o}{k}" if self.sym else f"{o}.{k}")
        return f"{self.acc} {base}" if self.acc else base

    @property
    def group(self):
        root = self.owner.strip("%").split(".")[0]
        if root in ("GlobalObject", "Intrinsic"):
            root = self.key if self.constructor else "global"
        return snake(root)

    @property
    def file(self):
        parts = self.owner.strip("%").split(".")[1:]
        if self.owner in ("%GlobalObject%", "%Intrinsic%"):
            parts = [] if self.constructor else [self.key]
        elif not self.sym:
            parts.append(self.key)
        if self.sym:
            parts.append("symbol_" + snake(self.sym))
        if self.acc:
            parts.insert(len(parts) - 1, self.acc)
        name = "_".join(snake(p) for p in parts) if parts else "constructor"
        if self.constructor:
            name = "constructor"
        return name


def expand_templates(head):
    if head.startswith("_NativeError_"):
        return [head.replace("_NativeError_", n) for n in NATIVE_ERRORS]
    if "_TypedArray_" in head:
        return [head.replace("_TypedArray_", n) for n in TYPED_ARRAYS]
    return [head]


CTOR_PROPS_RE = re.compile(r"Properties of the (\S+) (?:Constructors?|Intrinsic Object)$")
CTOR_LENGTH_RE = re.compile(r'has a \*"length"\* property whose value is \*\+?(\d+)\*')


def constructor_lengths(text):
    """Constructor "length" values, which the spec gives in the constructor's
    "Properties of the X Constructor" clause rather than under its heading."""
    lengths = {}
    for _cid, _typ, start, end in spec.clause_spans(text):
        body = text[start:end]
        m = spec.H1_RE.search(body)
        title = m and CTOR_PROPS_RE.match(spec.strip_tags(m.group(1)).strip())
        if not title:
            continue
        # Only the clause's own text, not its subclauses (the static methods).
        own = body[: body.find("<emu-clause", m.end())] if "<emu-clause" in body[m.end():] else body
        n = CTOR_LENGTH_RE.search(own)
        if n:
            for name in expand_templates(title.group(1)):
                lengths[name] = int(n.group(1))
    return lengths


def collect(text):
    fns = []
    seen = set()
    ctor_lengths = constructor_lengths(text)
    for cid, typ, start, end in spec.clause_spans(text):
        if typ != "built-in function":
            continue
        body = text[start:end]
        m = spec.H1_RE.search(body)
        if not m:
            continue
        head0 = spec.strip_tags(m.group(1)).strip()
        for head in expand_templates(head0):
            hm = HEAD_RE.match(head)
            if not hm:
                print(f"skip (unparsed heading): {head}", file=sys.stderr)
                continue
            path, sym, acc, params = hm.group("path"), hm.group("sym"), hm.group("acc"), hm.group("params")
            # Standalone intrinsic functions (%ThrowTypeError%) are created by hand.
            if path in SKIP_OWNERS:
                continue
            if sym:
                owner, key = owner_intrinsic(path), None
            elif "." in path.replace("%", "") or (path.startswith("%") and path.count("%") == 2 and "." in path):
                owner_path, key = path.rsplit(".", 1)
                owner = owner_intrinsic(owner_path)
            else:
                owner, key = "%GlobalObject%", path.replace("%", "")
            constructor = owner == "%GlobalObject%" and (key[:1].isupper() or path == "%TypedArray%")
            if path == "%TypedArray%" and not sym and not acc:
                # %TypedArray% is a constructor but not a global property.
                owner, key, constructor = "%Intrinsic%", "TypedArray", True
            sig = (owner, key, sym, acc)
            if sig in seen:
                continue
            seen.add(sig)
            length = 0 if acc else expected_length(params, body)
            # A setter whose prose gives it a parameter ("takes an argument _v_").
            if acc == "set" and re.search(r"takes an argument _\w+_", body):
                length = 1
            if constructor and key in ctor_lengths:
                length = ctor_lengths[key]
            fns.append(Fn(cid, head, owner, key, sym, acc, length, attributes(body), body, constructor))
    return fns


def stub(fn):
    steps = spec.render(fn.body)
    steps = "\n".join(("    // " + l).rstrip() for l in steps.split("\n"))
    name = ident(fn.file)
    return f"""//! {fn.head}
//!
//! Spec: https://tc39.es/ecma262/#{fn.id}

use kahvi_core::prelude::*;

/// {fn.display}
pub(crate) fn {name}(agent: &mut Agent, this: Value, args: &[Value], new_target: Option<Object>) -> JsResult<Value> {{
{steps}
    let _ = (this, args, new_target);
    Err(agent.not_implemented("{fn.display}"))
}}
"""


def rust_str(s):
    return '"' + s.replace("\\", "\\\\").replace('"', '\\"') + '"'


def main(argv):
    force = "--force" in argv
    text = spec.load()
    fns = collect(text)
    groups = {}
    for fn in fns:
        groups.setdefault(fn.group, []).append(fn)

    # Stub files and module lists.
    created = 0
    for group, gfns in sorted(groups.items()):
        d = OUT / group
        d.mkdir(parents=True, exist_ok=True)
        names = sorted({fn.file for fn in gfns})
        mods = "".join(f"pub(crate) mod {ident(n)};\n" for n in names)
        (d / "mod.rs").write_text(f"//! Generated by harness/gen/builtins.py: one file per built-in function.\n\n{mods}")
        for fn in gfns:
            path = d / f"{fn.file}.rs"
            if force or not path.exists():
                path.write_text(stub(fn))
                created += 1

    # The installation table.
    rows = []
    for fn in fns:
        key = f"Key::Symbol({rust_str(fn.sym)})" if fn.sym else f"Key::Str({rust_str(fn.key)})"
        kind = {"get": "Kind::Getter", "set": "Kind::Setter", None: "Kind::Method"}[fn.acc]
        if fn.constructor:
            kind = "Kind::Constructor"
        attrs = fn.attrs or (True, False, True)
        # Accessor properties have no [[Writable]]; the table stores false.
        attrs = tuple("true" if a else "false" for a in attrs)
        behaviour = f"{fn.group}::{ident(fn.file)}::{ident(fn.file)}"
        rows.append(
            f"    FnEntry {{ owner: {rust_str(fn.owner)}, key: {key}, kind: {kind}, length: {fn.length}, "
            f"writable: {attrs[0]}, enumerable: {attrs[1]}, configurable: {attrs[2]}, behaviour: {behaviour} }},"
        )
    mods = "".join(f'#[path = "{g}/mod.rs"]\npub(crate) mod {g};\n' for g in sorted(groups))
    gen = f"""//! Generated by harness/gen/builtins.py from spec.html. Do not edit: rerun the generator.
//!
//! Every built-in function property: where it is installed, its key, kind,
//! "length" and attributes, and the Rust function implementing it.

#![allow(clippy::all)]

use crate::table::{{FnEntry, Key, Kind}};

{mods}
pub(crate) static FUNCTIONS: &[FnEntry] = &[
{chr(10).join(rows)}
];
"""
    (OUT / "generated.rs").write_text(gen)
    print(f"{len(fns)} built-in functions in {len(groups)} groups; {created} stub files written")


if __name__ == "__main__":
    main(sys.argv[1:])
