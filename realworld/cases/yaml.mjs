// yaml: parse and stringify YAML 1.2, keeping comments through a Document.
import YAML from "yaml";
import { show } from "./lib/util.mjs";

const src = `# config
name: kahvi   # the engine
version: 1.2
features: [modules, "async/await", generators]
limits:
  stack: 0x1000
  heap: 1e9
  enabled: yes
anchors:
  base: &base { a: 1, b: 2 }
  derived: { <<: *base, b: 3 }
multiline: |
  line one
  line two
folded: >
  folded
  text
dates: 2001-12-14t21:59:43.10-05:00
nothing: ~
`;
show("parse", YAML.parse(src, { merge: true }));
const doc = YAML.parseDocument(src);
doc.set("name", "kahvi-run");
doc.getIn(["features"]).add("ESM");
console.log(doc.toString());
console.log(YAML.stringify({ list: [1, "two", { three: [3, null, true] }], "key with: colon": "value\nwith newline", empty: {} }));
const errs = YAML.parseDocument("a: 1\n b: 2\nc: [unterminated").errors;
show("errors", errs.map((e) => e.code));
show("all docs", YAML.parseAllDocuments("---\na: 1\n---\nb: 2\n").map((d) => d.toJS()));
