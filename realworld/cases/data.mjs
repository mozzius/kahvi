// Data processing: a seeded generator, generators and iterators, JSON round
// trips, Map/Set grouping, sorting with comparators, regular expressions,
// typed arrays, and async work through promises and timers.
import { fnv1a, show } from "./lib/util.mjs";

function* xorshift(seed) {
  let s = seed >>> 0;
  for (;;) {
    s ^= s << 13; s >>>= 0;
    s ^= s >>> 17;
    s ^= s << 5; s >>>= 0;
    yield s;
  }
}
const rng = xorshift(2463534242);
const next = (n) => rng.next().value % n;
const cities = ["Helsinki", "Tampere", "Turku", "Oulu", "Espoo", "Vantaa"];
const products = ["kahvi", "pulla", "tee", "leipä", "juusto"];
const orders = Array.from({ length: 20000 }, (_, id) => ({
  id, city: cities[next(cities.length)], product: products[next(products.length)],
  qty: 1 + next(9), cents: 100 + next(1900), when: `2024-${String(1 + next(12)).padStart(2, "0")}-${String(1 + next(28)).padStart(2, "0")}`,
  tags: next(3) === 0 ? ["gift"] : [],
}));

const text = JSON.stringify(orders);
show("json length", text.length);
show("json hash", fnv1a(text));
const back = JSON.parse(text, (k, v) => (k === "cents" ? BigInt(v) : v));
show("bigint cents total", String(back.reduce((s, o) => s + o.cents * BigInt(o.qty), 0n)));

const byCity = new Map();
for (const o of orders) {
  const entry = byCity.get(o.city) ?? { count: 0, revenue: 0, products: new Set() };
  entry.count++;
  entry.revenue += o.qty * o.cents;
  entry.products.add(o.product);
  byCity.set(o.city, entry);
}
show("by city", [...byCity].sort(([a], [b]) => a.localeCompare(b)).map(([c, e]) => `${c}:${e.count}:${(e.revenue / 100).toFixed(2)}:${e.products.size}`));

const grouped = Object.groupBy(orders, (o) => o.when.slice(0, 7));
show("months", Object.keys(grouped).sort().map((m) => `${m}=${grouped[m].length}`).join(" "));
const top = orders.toSorted((a, b) => b.qty * b.cents - a.qty * a.cents || a.id - b.id).slice(0, 5).map((o) => o.id);
show("top orders", top);
show("stable sort", orders.slice(0, 50).sort((a, b) => a.qty - b.qty).map((o) => o.id).join(","));

const report = orders.slice(0, 200).map((o) => `#${o.id} ${o.product}x${o.qty} @ ${o.city} on ${o.when}`).join("\n");
const dates = [...report.matchAll(/on (?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})/g)].map((m) => m.groups);
show("regex dates", [dates.length, dates[0], dates.at(-1)]);
show("replace", report.split("\n").slice(0, 3).map((l) => l.replace(/x(\d)/, (_, n) => ` times ${n}`).replaceAll("@", "in")));
show("unicode", ["leipä".normalize("NFD").length, [..."kahvi☕"].length, "ÄÖÅ".toLowerCase(), "straße".toUpperCase()]);

const qty = Uint8Array.from(orders, (o) => o.qty);
const hist = new Uint32Array(10);
for (const q of qty) hist[q]++;
show("qty histogram", [...hist]);
const view = new DataView(new ArrayBuffer(8));
view.setFloat64(0, 3.14159);
show("dataview", [view.getUint32(0).toString(16), view.getUint32(4).toString(16)]);

const delay = (ms, v) => new Promise((r) => setTimeout(() => r(v), ms));
const results = await Promise.all(cities.map((c, i) => delay(30 - i * 5, c.toUpperCase())));
show("promise.all", results);
const settled = await Promise.allSettled([delay(1, "a"), Promise.reject(new RangeError("nope")), delay(2, "c")]);
show("allSettled", settled.map((s) => s.status + ":" + (s.value ?? s.reason.message)));
async function* batches(list, size) {
  for (let i = 0; i < list.length; i += size) {
    await delay(0);
    yield list.slice(i, i + size);
  }
}
let batchCount = 0, batchTotal = 0;
for await (const batch of batches(orders, 2500)) { batchCount++; batchTotal += batch.reduce((s, o) => s + o.qty, 0); }
show("async batches", [batchCount, batchTotal]);
const order = [];
setTimeout(() => order.push("timeout"), 0);
queueMicrotask(() => order.push("microtask"));
await Promise.resolve().then(() => order.push("then"));
await delay(5);
show("event order", order);
