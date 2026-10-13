#!/usr/bin/env python3
"""Look up ECMA-262 clauses from the pinned spec (vendor/ecma262/spec.html).

  ./spec sec-array.prototype.flatmap     print one clause, steps numbered as in the spec
  ./spec array.prototype.flatmap         same ("sec-" is optional)
  ./spec --find flatMap                  search clause titles
  ./spec --find "ToObject"               ...including abstract operations

Algorithm steps are renumbered the way the published spec shows them
(1. / a. / i. / 1. ...), so code comments can cite them exactly.
"""

import html
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "vendor" / "ecma262" / "spec.html"

OPEN_RE = re.compile(r"<(emu-clause|emu-annex)\b([^>]*)>")
CLOSE_RE = re.compile(r"</(emu-clause|emu-annex)>")
ID_RE = re.compile(r'\bid="([^"]+)"')
TYPE_RE = re.compile(r'\btype="([^"]+)"')
H1_RE = re.compile(r"<h1>(.*?)</h1>", re.S)


def load():
    return SPEC.read_text(encoding="utf-8")


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


def main(argv):
    if not argv or argv[0] in ("-h", "--help"):
        print(__doc__.strip())
        return 0
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
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
