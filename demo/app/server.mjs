// The demo site: a Hono app, served by kahvi-serve (kahvi, not Node).
import { Hono } from 'hono';
import { card, newSeed } from './og.mjs';

const decoder = new TextDecoder();
const read = (path) => Kahvi.readFile(`${import.meta.dirname}/${path}`);
const readText = (path) => decoder.decode(read(path));

// test262 runs in kahvi's passing ratchet at the commit this was built from.
const passing = readText('expected-pass.txt')
  .split('\n')
  .filter((line) => line && !line.startsWith('#')).length;
const commit = (process.env.COMMIT || 'unknown').slice(0, 12);
const started = new Date().toISOString().replace(/\.\d+Z$/, 'Z');

const passingText = String(passing).replace(/\B(?=(\d{3})+(?!\d))/g, ',');
const page = readText('public/index.html')
  .replaceAll('{{passing}}', passingText)
  .replaceAll('{{commit}}', commit)
  .replaceAll('{{started}}', started);
const worker = readText('public/worker.js');
// kahvi itself, compiled to WebAssembly, gzipped at build time.
const wasm = read('public/kahvi.wasm.gz');

let served = 0;
const app = new Hono();

app.use(async (c, next) => {
  const t = Date.now();
  await next();
  served++;
  console.log(`${c.req.method} ${c.req.path} ${c.res.status} ${Date.now() - t}ms`);
});

// Each page view links its own card (see og.mjs).
app.get('/', (c) => {
  const origin = `${c.req.header('x-forwarded-proto') ?? 'http'}://${c.req.header('host')}`;
  return c.html(page.replaceAll('{{og-image}}', `${origin}/og.png?seed=${newSeed()}`));
});
app.get('/og.png', async (c) => {
  const seed = (c.req.query('seed') ?? newSeed()).slice(0, 16);
  const png = await card(seed, { passing: passingText, commit });
  return c.body(png, 200, { 'content-type': 'image/png', 'cache-control': 'public, max-age=86400, immutable' });
});
app.get('/worker.js', (c) => c.body(worker, 200, { 'content-type': 'text/javascript; charset=utf-8' }));
app.get('/kahvi.wasm', (c) => {
  if (!/\bgzip\b/.test(c.req.header('accept-encoding') ?? '')) {
    return c.text('This file is only served gzipped.\n', 406);
  }
  return c.body(wasm, 200, {
    'content-type': 'application/wasm',
    'content-encoding': 'gzip',
    'cache-control': 'public, max-age=3600',
  });
});
app.get('/api/status', (c) =>
  c.json({ engine: 'kahvi', server: 'hono on kahvi-serve', commit, test262Passing: passing, started, requestsServed: served }),
);
app.get('/healthz', (c) => c.text('ok\n'));

Kahvi.serve(app);
