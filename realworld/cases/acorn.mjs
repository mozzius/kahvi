// acorn: parse modern syntax and a large generated module to ESTree.
import * as acorn from "acorn";
import { small, large } from "./lib/corpus.mjs";
import { fnv1a, json, show } from "./lib/util.mjs";

const opts = { ecmaVersion: "latest", sourceType: "module" };
const ast = acorn.parse(small, opts);
show("small body types", ast.body.map((n) => n.type));
show("small ast hash", fnv1a(json(ast)));
show("class", JSON.stringify(ast.body[2].declaration.body.body.map((m) => [m.type, m.key?.name ?? m.key?.type ?? null])));

const big = acorn.parse(large, { ...opts, locations: true });
show("large statements", big.body.length);
show("large ast hash", fnv1a(json(big)));

let count = 0;
const types = {};
(function walk(node) {
  if (node && typeof node.type === "string") {
    count++;
    types[node.type] = (types[node.type] || 0) + 1;
  }
  for (const key in node) {
    const v = node[key];
    if (v && typeof v === "object" && key !== "loc") walk(v);
  }
})(big);
show("large node count", count);
show("large node types", Object.keys(types).sort().map((k) => k + "=" + types[k]).join(" "));

for (const bad of ["let let = 1", "a => { 'use strict'; with (a) {} }", "class { }", "x = 08n"]) {
  try {
    acorn.parse(bad, opts);
    show("no error", bad);
  } catch (e) {
    show("error", e.constructor.name + " " + e.message + " @" + e.pos);
  }
}
const tokens = [...acorn.tokenizer("let x = a?.b ?? `t${1}`;", opts)].map((t) => t.type.label);
show("tokens", tokens.join(" "));
