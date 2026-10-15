#!/usr/bin/env python3
"""Look up ECMA-262 and ECMA-402 clauses from the pinned specs (vendor/ecma262/spec.html,
vendor/ecma402/).

  ./spec sec-array.prototype.flatmap     print one clause, steps numbered as in the spec
  ./spec array.prototype.flatmap         same ("sec-" is optional)
  ./spec --find flatMap                  search clause titles
  ./spec --find "ToObject"               ...including abstract operations
  ./spec --refs sec-array.prototype.at   also print short abstract operations it calls

Algorithm steps are renumbered the way the published spec shows them
(1. / a. / i. / 1. ...), so code comments can cite them exactly.
"""

import html
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = Path(os.environ.get("KAHVI_VENDOR") or ROOT / "vendor") / "ecma262" / "spec.html"

OPEN_RE = re.compile(r"<(emu-clause|emu-annex)\b([^>]*)>")
CLOSE_RE = re.compile(r"</(emu-clause|emu-annex)>")
ID_RE = re.compile(r'\bid="([^"]+)"')
TYPE_RE = re.compile(r'\btype="([^"]+)"')
H1_RE = re.compile(r"<h1>(.*?)</h1>", re.S)

# Stage 3 proposals whose test262 features are scored (vendor/proposals/,
# each proposal repository's spec.emu as of 2026-10-15), with the clauses
# that define built-in functions. They are appended to the pinned spec, so a
# clause id the spec also has resolves to the spec's version.
PROPOSALS = {
    "iterator-chunking.emu": ["sec-iterator.prototype.chunks", "sec-iterator.prototype.windows"],
    "iterator-includes.emu": ["sec-iterator.prototype.includes"],
    "error-stack-accessor.emu": ["sec-get-error.prototype.stack", "sec-set-error.prototype.stack"],
}


# ECMA-402 (vendor/ecma402/, tc39/ecma402 spec/ as of 2026-10-15), in the
# order spec/index.html imports it. Its source does not mark built-in
# functions, so load() marks every clause whose heading is a function or
# accessor heading. Its locale-sensitive functions ("sup-" clauses, such as
# Number.prototype.toLocaleString) replace ECMA-262's.
ECMA402 = SPEC.parent.parent / "ecma402"
ECMA402_PARTS = [
    "conventions.html", "locales-currencies-tz.html", "requirements.html", "intl.html",
    "negotiation.html", "collator.html", "datetimeformat.html", "displaynames.html",
    "durationformat.html", "listformat.html", "locale.html", "numberformat.html",
    "pluralrules.html", "relativetimeformat.html", "segmenter.html",
    "locale-sensitive-functions.html", "annexes.html",
]
FUNCTION_HEAD_RE = re.compile(
    r"^(?:(?:get|set) )?[%\w.]+(?: \[ %Symbol\.\w+% \])?(?: \((?! \. \. \.).*\))?$"
)
# The clause ids that came from ECMA-402 (filled in by load()).
ECMA402_IDS = set()


def mark_ecma402_functions(part):
    """Adds type="built-in function" to the function clauses of an ECMA-402 file."""
    out = []
    last = 0
    for m in OPEN_RE.finditer(part):
        attrs = m.group(2)
        ident = ID_RE.search(attrs)
        if ident:
            ECMA402_IDS.add(ident.group(1))
        if TYPE_RE.search(attrs) or m.group(1) != "emu-clause":
            continue
        h1 = H1_RE.search(part, m.end())
        if not h1 or part.find("<emu-clause", m.end(), h1.start()) != -1:
            continue
        head = strip_tags(h1.group(1)).strip()
        if not FUNCTION_HEAD_RE.match(head) or not ("(" in head or head.startswith(("get ", "set "))):
            continue
        out.append(part[last:m.end() - 1])
        out.append(' type="built-in function">')
        last = m.end()
    out.append(part[last:])
    return "".join(out)


def load():
    text = SPEC.read_text(encoding="utf-8")
    for name, functions in PROPOSALS.items():
        part = (SPEC.parent.parent / "proposals" / name).read_text(encoding="utf-8")
        for cid in functions:
            tag = f'<emu-clause id="{cid}">'
            assert tag in part, f"{name}: no clause {cid}"
            part = part.replace(tag, f'<emu-clause id="{cid}" type="built-in function">')
        text += "\n" + part
    for name in ECMA402_PARTS:
        text += "\n" + mark_ecma402_functions((ECMA402 / name).read_text(encoding="utf-8"))
    return text


def clause_spans(text):
    """Yields (id, type, start, end) for every clause, end exclusive of the close tag."""
    stack = []
    events = [(m.start(), "open", m) for m in OPEN_RE.finditer(text)]
    events += [(m.start(), "close", m) for m in CLOSE_RE.finditer(text)]
    events.sort(key=lambda e: e[0])
    for pos, kind, m in events:
        if kind == "open":
            ident = ID_RE.search(m.group(2))
            typ = TYPE_RE.search(m.group(2))
            stack.append((ident.group(1) if ident else None, typ.group(1) if typ else "", m.end()))
        elif stack:
            ident, typ, start = stack.pop()
            if ident:
                yield ident, typ, start, pos


def strip_tags(s):
    s = re.sub(r'<emu-xref href="#([^"]+)"[^>]*>\s*</emu-xref>', r"[\1]", s)
    s = re.sub(r"<sup>", "^", s)
    s = re.sub(r"<sub>", "_", s)
    s = re.sub(r"<[^>]+>", "", s)
    return html.unescape(s)


def roman(n):
    vals = [(10, "x"), (9, "ix"), (5, "v"), (4, "iv"), (1, "i")]
    out = ""
    for v, r in vals:
        while n >= v:
            out += r
            n -= v
    return out


def label(depth, n):
    style = depth % 3
    if style == 0:
        return str(n)
    if style == 1:
        s = ""
        while n:
            n, rem = divmod(n - 1, 26)
            s = chr(ord("a") + rem) + s
        return s
    return roman(n)


def render_alg(body):
    lines = [l for l in body.split("\n") if l.strip()]
    if not lines:
        return ""
    base = min(len(l) - len(l.lstrip()) for l in lines)
    counters = []
    out = []
    for line in lines:
        indent = (len(line) - len(line.lstrip()) - base) // 2
        text = line.strip()
        m = re.match(r"^(\d+\.|\*)\s+(.*)$", text)
        if not m:
            out.append("   " * (indent + 1) + strip_tags(text))
            continue
        step = re.sub(r"^\[[^\]]*\]\s*", "", m.group(2))
        if m.group(1) == "*":
            out.append("   " * indent + "• " + strip_tags(step))
            continue
        del counters[indent + 1 :]
        while len(counters) <= indent:
            counters.append(0)
        counters[indent] += 1
        out.append("   " * indent + f"{label(indent, counters[indent])}. {strip_tags(step)}")
    return "\n".join(out)


def render(body):
    # Replace nested clauses with a one-line pointer.
    def sub_clause(inner):
        ident = ID_RE.search(inner[: inner.find(">")])
        title = H1_RE.search(inner)
        name = strip_tags(title.group(1)).strip() if title else ""
        return f"\n[subclause {ident.group(1) if ident else '?'}: {name}]\n"

    body = remove_nested(body, sub_clause)
    parts = []
    pos = 0
    for m in re.finditer(r"<emu-alg\b[^>]*>(.*?)</emu-alg>", body, re.S):
        parts.append(render_prose(body[pos : m.start()]))
        parts.append(render_alg(m.group(1)))
        pos = m.end()
    parts.append(render_prose(body[pos:]))
    text = "\n\n".join(p for p in parts if p.strip())
    return re.sub(r"\n{3,}", "\n\n", text)


def remove_nested(body, repl):
    out = []
    depth = 0
    pos = 0
    start = None
    tokens = sorted(
        [(m.start(), m.end(), "open") for m in OPEN_RE.finditer(body)]
        + [(m.start(), m.end(), "close") for m in CLOSE_RE.finditer(body)]
    )
    for s, e, kind in tokens:
        if kind == "open":
            if depth == 0:
                out.append(body[pos:s])
                start = s
            depth += 1
        else:
            depth -= 1
            if depth == 0 and start is not None:
                out.append(repl(body[start:e]))
                pos = e
    out.append(body[pos:])
    return "".join(out)


def render_prose(s):
    s = re.sub(r"<h1>(.*?)</h1>", "", s, flags=re.S)
    s = re.sub(r"<emu-note\b[^>]*>", "\nNOTE: ", s)
    s = re.sub(r"<dt>(.*?)</dt>\s*<dd>(.*?)</dd>", lambda m: f"\n{m.group(1)}: {m.group(2)}\n", s, flags=re.S)
    s = re.sub(r"</(p|li|tr|emu-note|emu-grammar|dl)>", "\n", s)
    s = re.sub(r"<(li)\b[^>]*>", "\n  - ", s)
    s = re.sub(r"</t[dh]>", " | ", s)
    s = strip_tags(s)
    lines = [re.sub(r"[ \t]+", " ", l).strip() for l in s.split("\n")]
    return "\n".join(l for l in lines if l)


def find_clause(text, wanted):
    wanted = wanted.lower()
    if not wanted.startswith("sec-"):
        candidates = (wanted, "sec-" + wanted)
    else:
        candidates = (wanted,)
    for ident, typ, start, end in clause_spans(text):
        if ident.lower() in candidates:
            return ident, typ, text[start:end]
    return None


def clause_index(text):
    """Maps an operation name (from its <h1>) to (id, start, end)."""
    index = {}
    for ident, typ, start, end in clause_spans(text):
        if typ not in ("abstract operation", "implementation-defined abstract operation"):
            continue
        m = H1_RE.search(text, start, min(end, start + 2000))
        if m:
            name = strip_tags(m.group(1)).split("(")[0].strip()
            index.setdefault(name, (ident, start, end))
    return index


def referenced_ops(text, body, seen, max_chars=1500, max_ops=10):
    """Short abstract operations called from body's algorithm steps."""
    index = clause_index(text)
    algs = " ".join(re.findall(r"<emu-alg\b[^>]*>(.*?)</emu-alg>", remove_nested(body, lambda _: ""), re.S))
    out = []
    for name in dict.fromkeys(re.findall(r"\b([A-Z][A-Za-z0-9]+)\(", algs)):
        if name not in index or len(out) >= max_ops:
            continue
        ident, start, end = index[name]
        if ident in seen:
            continue
        rendered = render(text[start:end])
        if len(rendered) <= max_chars:
            seen.add(ident)
            title = H1_RE.search(text, start, end)
            out.append((" ".join(strip_tags(title.group(1)).split()), ident, rendered))
    return out


def main(argv):
    if not argv or argv[0] in ("-h", "--help"):
        print(__doc__.strip())
        return 0
    refs = argv[0] == "--refs"
    if refs:
        argv = argv[1:]
    text = load()
    if argv[0] == "--find":
        needle = " ".join(argv[1:]).lower()
        hits = 0
        for ident, typ, start, end in clause_spans(text):
            m = H1_RE.search(text, start, min(end, start + 2000))
            title = " ".join(strip_tags(m.group(1)).split()) if m else ""
            if needle in title.lower() or needle in ident.lower():
                print(f"{ident:<55} {title}" + (f"  [{typ}]" if typ else ""))
                hits += 1
        if not hits:
            print(f"no clause title matches {needle!r}", file=sys.stderr)
            return 1
        return 0
    found = find_clause(text, argv[0])
    if not found:
        print(f"no clause with id {argv[0]!r}; try --find", file=sys.stderr)
        return 1
    ident, typ, body = found
    title = H1_RE.search(body)
    print(" ".join(strip_tags(title.group(1)).split()) if title else ident)
    print(f"  id: {ident}" + (f"   type: {typ}" if typ else ""))
    print()
    print(render(body))
    if refs:
        for title, rid, rendered in referenced_ops(text, body, {ident}):
            print(f"\n--- referenced: {title}\n  id: {rid}\n")
            print(rendered)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
