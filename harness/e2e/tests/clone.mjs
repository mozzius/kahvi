// structuredClone and process.env.
const inner = { n: 1 };
const original = {
  num: 1.5, str: 'hi', big: 10n, nul: null, undef: undefined, bool: true,
  arr: [1, , 3], date: new Date(0), re: /a+/gi,
  map: new Map([[inner, 'v'], ['k', inner]]), set: new Set([inner, 2]),
  wrapped: [new Number(4), new String('s'), new Boolean(false)],
  err: new RangeError('bad'), buf: new Uint8Array([1, 2, 3]),
  view: new DataView(new ArrayBuffer(4), 1, 2), inner, again: inner,
};
original.self = original;
const c = structuredClone(original);
console.log(c !== original, c.self === c, c.inner !== inner, c.inner === c.again);
console.log(c.num, c.str, c.big, c.nul, c.undef, c.bool, 1 in c.arr, c.arr.length);
console.log(c.date instanceof Date, c.date.getTime(), String(c.re), c.re.lastIndex);
console.log(c.map.get(c.inner), c.map.get('k') === c.inner, c.set.has(c.inner), c.set.size);
console.log(typeof c.wrapped[0], c.wrapped[0] + 1, c.wrapped[1].length, c.wrapped[2].valueOf());
console.log(c.err instanceof RangeError, c.err.message, c.err !== original.err);
console.log(c.buf instanceof Uint8Array, [...c.buf].join(), c.buf.buffer !== original.buf.buffer);
console.log(c.view.byteOffset, c.view.byteLength, c.view.buffer.byteLength);
class Point { constructor() { this.x = 1; } }
const p = structuredClone(new Point());
console.log(Object.getPrototypeOf(p) === Object.prototype, p.x);
const getter = structuredClone({ get g() { return 'got'; } });
console.log(Object.getOwnPropertyDescriptor(getter, 'g').value);
for (const bad of [() => {}, Symbol('s'), { f() {} }, new WeakMap(), new Proxy({}, {}), Promise.resolve()]) {
  try {
    structuredClone(bad);
    console.log('cloned?');
  } catch (e) {
    console.log(e.name, e instanceof Error);
  }
}
console.log(typeof process.env, typeof process.env.PATH, process.env.NODE_ENV);
