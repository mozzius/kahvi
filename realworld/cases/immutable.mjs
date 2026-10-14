// immutable: persistent List, Map, Set, Record and lazy Seq. The package has
// no "exports", so the ES module build is imported by its path.
import { List, Map, OrderedMap, Set, Record, Seq, Range, fromJS, is } from "immutable/dist/immutable.es.js";
import { show } from "./lib/util.mjs";

const list = List(Range(0, 2000));
const updated = list.set(1500, -1).push(2000).delete(0).insert(5, "x");
show("list", [list.size, updated.size, updated.get(1499), updated.get(5), updated.last(), list.get(1500)]);
const map = Map({ a: 1, b: 2 }).set("c", 3).update("a", (v) => v * 10).delete("b");
show("map", map.toJS());
show("deep", fromJS({ x: { y: [1, 2, { z: 3 }] } }).setIn(["x", "y", 2, "z"], 4).updateIn(["x", "y"], (l) => l.push(5)).toJS());
show("equals", [is(Map({ a: List([1]) }), Map({ a: List([1]) })), Map({ a: 1 }).hashCode() === Map({ a: 1 }).hashCode(), Set([1, 2]).equals(Set([2, 1]))]);
const Point = Record({ x: 0, y: 0 }, "Point");
const p = Point({ x: 3 });
show("record", [p.x, p.y, p.set("y", 4).toJS(), String(p)]);
let evaluated = 0;
const lazy = Seq(Range(1, Infinity)).map((n) => (evaluated++, n * n)).filter((n) => n % 3 === 1).take(5);
show("seq", [lazy.toArray(), evaluated]);
show("ordered", OrderedMap([["z", 1], ["a", 2], ["m", 3]]).sortBy((v, k) => k).keySeq().toArray());
show("groupBy", List([1, 2, 3, 4, 5, 6]).groupBy((n) => (n % 2 ? "odd" : "even")).map((l) => l.toArray()).toJS());
show("set ops", [Set([1, 2, 3]).union([3, 4]).toArray(), Set([1, 2, 3]).intersect([2, 3, 4]).toArray(), Set([1, 2, 3]).subtract([1]).toArray()]);
let big = Map();
for (let i = 0; i < 5000; i++) big = big.set("k" + i, i);
show("big map", [big.size, big.get("k4321"), big.filter((v) => v % 1000 === 0).keySeq().sort().toArray()]);
