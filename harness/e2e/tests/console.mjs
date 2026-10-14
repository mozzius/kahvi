// console formatting: primitives, objects, arrays, collections, functions,
// cycles, depth limits, substitutions and the streams.
console.log('str', 1, -0, 10n, true, null, undefined, Symbol('s'));
console.log({ a: 1, 'b-c': 'x', [Symbol('k')]: [1, , 3] });
console.log([], {}, [[]], { d: { e: { f: { g: 1 } } } });
console.log(new Map([['k', { v: 1 }]]), new Set(['a', 1]));
console.log(function named() {}, class K {}, () => {}, Math.max);
console.log(new Date(0), /x+/gu, new Number(3), new String('ab'), Object.create(null));
console.log(Promise.resolve([1]), new Promise(() => {}));
class Point {
  constructor() {
    this.x = 1;
    this.y = 2;
  }
}
console.log(new Point(), { get g() { return 1; }, set s(v) {} });
const cyc = { name: 'c' };
cyc.self = cyc;
console.log(cyc);
console.log('%s has %d items (%i%%)', 'cart', 3.5, 99.9, 'extra');
console.log('%o and %j', [1], { a: [2] });
console.log("it's");
console.info('info');
console.debug('debug');
console.warn('warn', { w: 1 });
console.error(new RangeError('bad range'));
console.assert(true, 'not shown');
console.assert(false, 'shown', 1);
console.log(Array.from({ length: 105 }, (_, i) => i).length, [1, 2, 3].map((x) => x * 2));
