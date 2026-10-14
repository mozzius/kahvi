// JavaScript source for the parser, formatter and minifier cases: a mix of
// modern syntax, plus a generated module large enough to exercise deep
// recursion, big arrays and long strings.

export const small = `
import { a as b, default as c } from "./x.js";
export const answer = 6 * 7;
export default class Point extends Base {
  #x = 0; static count = 0;
  constructor(x, y = 1, ...rest) { super(); this.#x = x; this.y = y ?? rest[0]; }
  get x() { return this.#x; }
  static { Point.count++; }
  *[Symbol.iterator]() { yield this.#x; yield* [this.y]; }
  async load(url) { for await (const chunk of fetchAll(url)) { this.y += chunk?.length ?? 0; } }
}
const re = /(?<year>\\d{4})-(?<month>\\d{2})/gu;
const t = tag\`a\${1 + 2}b\${\`nested \${3}\`}\`;
label: for (let i = 0, j = 10; i < j; i++, j--) { if (i % 2) continue label; else break label; }
const { p, q: [r, , s = 5] = [], ...others } = obj;
x ||= 1; y &&= 2; z ??= 3; n = 1_000_000n ** 2n;
try { throw new Error("e"); } catch { void 0; } finally { delete obj[key]; }
switch (typeof v) { case "number": v++; break; default: v = !v; }
`;

function generated(n) {
  const parts = [];
  for (let i = 0; i < n; i++) {
    parts.push(`
export function f${i}(a, b = ${i}) {
  const list = [${Array.from({ length: 8 }, (_, k) => (k * i) % 97).join(", ")}];
  let total = 0;
  for (const v of list) {
    if (v % 3 === 0) total += v * a; else if (v % 3 === 1) total -= b; else total ^= v;
  }
  const obj = { id: ${i}, name: "item-${i}", nested: { deep: { deeper: [total, a, b] } } };
  return a > ${i % 10} ? obj.nested.deep.deeper.reduce((x, y) => x + y, 0) : \`f${i}: \${total}\`;
}`);
  }
  return parts.join("\n");
}

export const large = generated(300);
