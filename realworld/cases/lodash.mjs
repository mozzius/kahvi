// lodash-es: collections, objects, strings, and _.template (which compiles
// code with Function).
import _ from "lodash-es";
import { show } from "./lib/util.mjs";

const people = [
  { name: "ada", age: 36, team: "engine" },
  { name: "grace", age: 45, team: "compiler" },
  { name: "linus", age: 28, team: "engine" },
  { name: "barbara", age: 52, team: "runtime" },
];
show("groupBy", _.groupBy(people, "team"));
show("sortBy", _.sortBy(people, ["team", (p) => -p.age]).map((p) => p.name));
show("partition", _.partition(people, (p) => p.age > 40).map((g) => g.map((p) => p.name)));
show("chunk", _.chunk(_.range(0, 20, 3), 3));
show("zipObject", _.zipObject(["a", "b.c", "d"], [1, 2, 3]));
show("uniqBy", _.uniqBy([2.1, 1.2, 2.3, 3.4, 1.9], Math.floor));
show("intersectionWith", _.intersectionWith([{ x: 1 }, { x: 2 }], [{ x: 2 }], _.isEqual));
const deep = { a: [{ b: { c: 3 } }], d: new Map([["k", new Set([1, 2])]]), e: new Date(0), f: /x/gi };
const clone = _.cloneDeep(deep);
show("cloneDeep equal", [_.isEqual(deep, clone), clone.d !== deep.d, clone.d.get("k").has(2), clone.e.getTime(), String(clone.f)]);
show("merge", _.merge({ a: [{ b: 2 }, { d: 4 }] }, { a: [{ c: 3 }, { e: 5 }] }));
const o = {};
_.set(o, "x[0].y.z", 5);
show("set/get", [o, _.get(o, ["x", 0, "y", "z"]), _.get(o, "x[1].y", "default"), _.has(o, "x[0].y")]);
show("strings", [_.camelCase("Foo Bar_baz-qux"), _.kebabCase("fooBarBaz"), _.snakeCase("Ångström unit"), _.startCase("--foo-bar--"), _.words("fred, barney, & pebbles"), _.pad("abc", 9, "_-"), _.escape("<a href='x'>&</a>"), _.truncate("hi-diddly-ho there, neighborino", { length: 24, separator: /,? +/ })]);
const memo = _.memoize((n) => (n < 2 ? n : memo(n - 1) + memo(n - 2)));
show("memoize fib(80)", memo(80));
const tpl = _.template("<% _.forEach(users, function(u) { %><li><%- u %></li><% }); %> total=${ users.length }");
show("template", tpl({ users: ["fred", "<b>barney</b>"] }));
show("debounce is function", typeof _.debounce(() => {}, 10));
show("flow", _.flow([_.add, _.partial(_.multiply, 3), String])(2, 5));
show("countBy", _.countBy(["one", "two", "three"], "length"));
show("orderBy", _.orderBy(people, ["age"], ["desc"]).map((p) => p.age));
show("isEqual", [_.isEqual(NaN, NaN), _.isEqual([1, [2, { a: 3 }]], [1, [2, { a: 3 }]]), _.isEqual({ a: 1 }, { a: 1, b: undefined })]);
