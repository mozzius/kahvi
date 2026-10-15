// immer: copy-on-write updates through Proxy, patches, Map and Set drafts.
import { produce, produceWithPatches, applyPatches, enablePatches, enableMapSet, current, original, isDraft, freeze } from "immer";
import { show } from "./lib/util.mjs";

enablePatches();
enableMapSet();
const base = { todos: [{ id: 1, text: "write engine", done: false }, { id: 2, text: "run tests", done: false }], meta: { count: 2 } };
const next = produce(base, (draft) => {
  draft.todos[1].done = true;
  draft.todos.push({ id: 3, text: "benchmark", done: false });
  draft.meta.count = draft.todos.length;
  show("inside", [isDraft(draft), isDraft(draft.todos), original(draft.meta).count, current(draft.meta).count]);
});
show("next", next);
show("sharing", [base.todos[0] === next.todos[0], base.todos[1] === next.todos[1], base === next, Object.isFrozen(next), Object.isFrozen(next.todos[2])]);
show("noop", produce(base, () => {}) === base);
const [state, patches, inverse] = produceWithPatches(base, (d) => {
  d.todos.splice(0, 1);
  delete d.meta.count;
  d.meta.owner = "kahvi";
});
show("patches", patches);
show("inverse", inverse);
show("roundtrip", JSON.stringify(applyPatches(state, inverse)) === JSON.stringify(base));
const m = produce(new Map([["a", { n: 1 }]]), (d) => {
  d.get("a").n++;
  d.set("b", { n: 10 });
});
show("map", [...m].map(([k, v]) => k + "=" + v.n));
const s = produce(new Set([1, 2, 3]), (d) => {
  d.delete(2);
  d.add(4);
});
show("set", [...s]);
show("return replace", produce({ a: 1 }, () => ({ b: 2 })));
const frozen = freeze({ deep: { x: 1 } }, true);
show("freeze", Object.isFrozen(frozen.deep));
try {
  produce(base, (d) => {
    d.meta.count = 5;
    return { other: true };
  });
} catch (e) {
  show("error", e.message.split("\n")[0]);
}
