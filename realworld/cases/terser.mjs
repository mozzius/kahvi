// terser: minify, compress and mangle, with a source map.
import { minify } from "terser";
import { small, large } from "./lib/corpus.mjs";
import { fnv1a, show } from "./lib/util.mjs";

const code = `
function fib(n) { return n < 2 ? n : fib(n - 1) + fib(n - 2); }
const unused = 1;
export function run(limit) {
  const results = [];
  for (let i = 0; i < limit; i++) { if (true) { results.push(fib(i)); } else { results.push(-1); } }
  return results.join(",") + " " + (2 * 3 + 4) + "!";
}
`;
const out = await minify(code, { module: true, compress: { passes: 2 }, mangle: true, sourceMap: true });
console.log(out.code);
show("map", JSON.parse(out.map).mappings);
const s = await minify(small, { module: true, compress: false, mangle: false });
console.log(s.code);
const big = await minify(large, { module: true, toplevel: true });
show("large minified length", big.code.length);
show("large minified hash", fnv1a(big.code));
try {
  await minify("function (", {});
} catch (e) {
  show("error", e.constructor.name + " " + e.message);
}
