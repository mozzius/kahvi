// meriyah: a second parser over the same corpus, with ranges and locations.
import { parse } from "meriyah";
import { small, large } from "./lib/corpus.mjs";
import { fnv1a, json, show } from "./lib/util.mjs";

const small_ast = parse(small, { module: true, next: true, ranges: true });
show("small body types", small_ast.body.map((n) => n.type));
show("small ast hash", fnv1a(json(small_ast)));
const big = parse(large, { module: true, loc: true, ranges: true });
show("large statements", big.body.length);
show("large ast hash", fnv1a(json(big)));
for (const bad of ["function f(a, a) { 'use strict' }", "for (const x of y, z) ;", "({ a: 1 } = 2)"]) {
  try {
    parse(bad, { module: true });
    show("no error", bad);
  } catch (e) {
    show("error", e.constructor.name + " " + e.description);
  }
}
