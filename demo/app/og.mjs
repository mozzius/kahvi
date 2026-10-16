// The OpenGraph card: laid out by satori on kahvi, rasterized by the host.
// The gradient comes from a seed, so a given /og.png?seed=… URL always
// draws the same card and every page view links a different one.
import satori from './vendor/satori.mjs';

const font = (weight) => ({
  name: 'Inter',
  weight,
  style: 'normal',
  data: Kahvi.readFile(`${import.meta.dirname}/node_modules/@fontsource/inter/files/inter-latin-${weight}-normal.woff`),
});
const fonts = [font(400), font(800)];

// mulberry32: a small seeded PRNG.
function random(seed) {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export const newSeed = () => Math.floor(Math.random() * 2 ** 32).toString(36);

// satori 0.0.44's gradient parser only takes hex colors.
function hsl(h, s, l) {
  const k = (n) => (n + h / 30) % 12;
  const a = s * Math.min(l, 1 - l);
  const f = (n) => l - a * Math.max(-1, Math.min(k(n) - 3, 9 - k(n), 1));
  const hex = (x) => Math.round(x * 255).toString(16).padStart(2, '0');
  return `#${hex(f(0))}${hex(f(8))}${hex(f(4))}`;
}

const h = (type, style, ...children) =>
  ({ type, props: { style, children: children.length === 1 ? children[0] : children } });

export async function card(seed, { passing, commit }) {
  const rand = random(parseInt(seed, 36) || 0);
  const hue = Math.floor(rand() * 360);
  const spread = 40 + Math.floor(rand() * 100);
  const angle = Math.floor(rand() * 360);
  const stops = [
    hsl(hue, 0.8, 0.55),
    hsl((hue + spread / 2) % 360, 0.75, 0.45),
    hsl((hue + spread) % 360, 0.85, 0.3),
  ];
  const stat = (value, label) =>
    h('div', { display: 'flex', flexDirection: 'column', marginRight: 64 },
      h('div', { fontSize: 44, fontWeight: 800 }, value),
      h('div', { fontSize: 24, opacity: 0.8 }, label));
  const svg = await satori(
    h('div', {
      width: '100%', height: '100%', display: 'flex', flexDirection: 'column', justifyContent: 'space-between',
      padding: 72, color: 'white', fontFamily: 'Inter',
      backgroundImage: `linear-gradient(${angle}deg, ${stops.join(', ')})`,
    },
      h('div', { display: 'flex', flexDirection: 'column' },
        h('div', { fontSize: 120, fontWeight: 800, letterSpacing: -4 }, 'kahvi'),
        h('div', { fontSize: 40, maxWidth: 900 }, 'A JavaScript engine written from scratch in Rust. Try it in your browser.')),
      h('div', { display: 'flex', alignItems: 'flex-end' },
        stat(passing, 'test262 runs passing'),
        stat(commit.slice(0, 7), 'commit'),
        h('div', { display: 'flex', flexGrow: 1, justifyContent: 'flex-end', fontSize: 22, opacity: 0.7 },
          `drawn by satori on kahvi · seed ${seed}`))),
    { width: 1200, height: 630, fonts },
  );
  return Kahvi.svgToPng(svg);
}
