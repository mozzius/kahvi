// ES module loading: relative and absolute specifiers, import.meta, JSON
// modules, dynamic import() and a cycle.
import { double, counter } from './lib/math.mjs';
import data from './lib/data.json' with { type: 'json' };
import * as cyc from './lib/cycle-a.mjs';

console.log(double(21), counter());
console.log(data.name, data.list.length);
console.log(import.meta.url.slice(0, 8), import.meta.filename === import.meta.dirname + '/modules.mjs');
console.log(cyc.a(), cyc.fromB());
const m = await import('./lib/math.mjs');
console.log(m.counter(), m === (await import('./lib/math.mjs')));
try {
  await import('./lib/missing.mjs');
} catch (e) {
  console.log(e.constructor.name);
}
try {
  await import('some-package');
} catch (e) {
  console.log(e.constructor.name);
}
