// Renders the playground page. Runs on kahvi-run at image build time:
//   kahvi-run demo/render.mjs > index.html
// with COMMIT and TEST262_PASSING in the environment.

const commit = (process.env.COMMIT || 'unknown').slice(0, 12);
const passing = Number(process.env.TEST262_PASSING || 0);
const built = new Date().toISOString().replace(/\.\d+Z$/, 'Z');

const escape = (s) =>
  String(s).replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);

const examples = {
  'Hello': `console.log('Hello from kahvi!');
console.log([1, 2, 3].map((x) => x ** 2));`,
  'Classes and generators': `class Fib {
  *[Symbol.iterator]() {
    let [a, b] = [0n, 1n];
    for (;;) { yield a; [a, b] = [b, a + b]; }
  }
}
const out = [];
for (const n of new Fib()) {
  if (out.length === 20) break;
  out.push(n);
}
console.log(out.join(', '));`,
  'Promises and timers': `const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
for (const name of ['one', 'two', 'three']) {
  await sleep(100);
  console.log(name, 'after', 100, 'ms');
}
queueMicrotask(() => console.log('a microtask'));`,
  'RegExp and Unicode': `const s = 'Kahvi on hyvää ☕ 2026-10-16';
const m = s.match(/(?<y>\\d{4})-(?<m>\\d{2})-(?<d>\\d{2})/);
console.log(m.groups);
console.log([...s.matchAll(/\\p{L}+/gu)].map((x) => x[0]));
console.log('☕'.codePointAt(0).toString(16));`,
  'Proxy and Reflect': `const log = [];
const p = new Proxy({}, {
  get(t, k, r) { log.push(String(k)); return Reflect.get(t, k, r); },
});
p.a; p.b; p[Symbol.toPrimitive];
console.log(log);`,
  'An error': `function deep(n) { return n === 0 ? null.x : deep(n - 1); }
deep(3);`,
};

const options = Object.keys(examples)
  .map((k) => `<option>${escape(k)}</option>`)
  .join('');

const page = `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>kahvi playground</title>
<style>
:root { --bg: #faf8f5; --fg: #2b2118; --muted: #7a6a5c; --panel: #fff; --line: #e4ddd4; --accent: #8a4b1f; --err: #b3261e; }
@media (prefers-color-scheme: dark) {
  :root { --bg: #1b1714; --fg: #ece4dc; --muted: #a8998b; --panel: #241f1b; --line: #3a322c; --accent: #e0a46e; --err: #f2b8b5; }
}
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--fg); font: 16px/1.5 system-ui, sans-serif; }
main { max-width: 860px; margin: 0 auto; padding: 24px 16px 48px; }
h1 { margin: 0 0 4px; font-size: 28px; }
p { margin: 8px 0; }
a { color: var(--accent); }
.muted { color: var(--muted); font-size: 14px; }
.bar { display: flex; gap: 8px; align-items: center; margin: 16px 0 8px; flex-wrap: wrap; }
select, button { font: inherit; padding: 6px 12px; border-radius: 6px; border: 1px solid var(--line); background: var(--panel); color: var(--fg); }
button { background: var(--accent); color: var(--bg); border-color: var(--accent); cursor: pointer; }
button:disabled { opacity: .6; cursor: default; }
textarea, pre { width: 100%; font: 14px/1.45 ui-monospace, SFMono-Regular, Menlo, monospace; background: var(--panel); color: var(--fg); border: 1px solid var(--line); border-radius: 8px; padding: 12px; }
textarea { height: 280px; resize: vertical; tab-size: 2; }
pre { min-height: 80px; white-space: pre-wrap; word-break: break-word; margin: 0; }
.err { color: var(--err); }
.stats { display: flex; gap: 24px; flex-wrap: wrap; margin: 16px 0; }
.stats div { font-size: 14px; color: var(--muted); }
.stats b { display: block; font-size: 22px; color: var(--fg); }
</style>
</head>
<body>
<main>
<h1>kahvi playground</h1>
<p><a href="https://github.com/mozzius/kahvi">kahvi</a> is a JavaScript engine written from scratch in Rust: parser, bytecode VM, garbage collector, regex engine and every built-in. Code you run here is executed by <code>kahvi-run</code> on the server, as an ES module, with a 5 second limit and about 768 MB of heap.</p>
<div class="stats">
  <div><b>${escape(String(passing).replace(/\B(?=(\d{3})+(?!\d))/g, ','))}</b>test262 runs passing</div>
  <div><b>${escape(commit)}</b>commit</div>
  <div><b>${escape(built)}</b>built</div>
</div>
<div class="bar">
  <select id="ex" aria-label="Example">${options}</select>
  <button id="run">Run</button>
  <span class="muted" id="meta"></span>
</div>
<textarea id="code" spellcheck="false" aria-label="JavaScript source"></textarea>
<div class="bar"><span class="muted">Output</span></div>
<pre id="out"></pre>
<p class="muted">Available: the ECMAScript built-ins plus <code>console</code>, timers, <code>queueMicrotask</code>, <code>structuredClone</code>, <code>TextEncoder</code>/<code>TextDecoder</code>, <code>atob</code>/<code>btoa</code>. There is no network or file access. This page was rendered by kahvi too, when the image was built.</p>
</main>
<script>
const EXAMPLES = ${JSON.stringify(examples).replace(/</g, '\\u003c')};
const code = document.getElementById('code');
const ex = document.getElementById('ex');
const out = document.getElementById('out');
const meta = document.getElementById('meta');
const btn = document.getElementById('run');
ex.onchange = () => { code.value = EXAMPLES[ex.value]; };
code.value = EXAMPLES[ex.value];
code.onkeydown = (e) => { if ((e.metaKey || e.ctrlKey) && e.key === 'Enter') run(); };
btn.onclick = run;
async function run() {
  btn.disabled = true; meta.textContent = 'running…'; out.textContent = '';
  try {
    const res = await fetch('/run', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ code: code.value }) });
    const r = await res.json();
    if (r.error) { out.innerHTML = ''; const s = document.createElement('span'); s.className = 'err'; s.textContent = r.error; out.append(s); meta.textContent = ''; return; }
    out.textContent = r.stdout;
    if (r.stderr) { const s = document.createElement('span'); s.className = 'err'; s.textContent = r.stderr; out.append(s); }
    const end = r.limit ? 'stopped: ' + r.limit : r.signal ? 'killed by signal ' + r.signal : 'exit ' + r.exitCode;
    meta.textContent = end + ' · ' + r.ms + ' ms' + (r.truncated ? ' · output truncated' : '');
  } catch (e) { out.textContent = String(e); meta.textContent = ''; }
  finally { btn.disabled = false; }
}
</script>
</body>
</html>
`;

console.log(page);
