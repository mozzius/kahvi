// js-yaml: a second YAML implementation, with schemas and dump options.
import * as yaml from "js-yaml";
import { show } from "./lib/util.mjs";

const src = `
server:
  host: example.org
  ports: [80, 443]
  tls: { cert: /etc/c.pem, key: /etc/k.pem }
users:
  - name: ada
    roles: [admin]
  - name: grace
    roles: []
when: 2020-02-02
octal: 0o17
`;
const data = yaml.load(src);
show("load", JSON.stringify(data, (k, v) => (v instanceof Uint8Array ? [...v] : v)));
show("json schema", yaml.load("a: 2020-02-02\nb: yes", { schema: yaml.JSON_SCHEMA }));
console.log(yaml.dump({ nested: { list: [1, [2, 3], { a: "b" }], text: "multi\nline\n", quoted: "yes", n: -0.5 } }, { indent: 4, sortKeys: true }));
console.log(yaml.dump([{ a: 1, b: [1, 2] }, "plain"], { flowLevel: 1 }));
try {
  yaml.load("a: [1, 2\nb: 3");
} catch (e) {
  show("error", e.name + " " + e.reason + " " + e.mark.line + ":" + e.mark.column);
}
show("loadAll", yaml.loadAll("--- 1\n--- [2]\n--- {c: 3}\n"));
