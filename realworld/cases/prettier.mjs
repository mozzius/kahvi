// prettier: format JavaScript, Markdown and CSS with the standalone build.
import * as prettier from "prettier/standalone";
import * as babel from "prettier/plugins/babel";
import * as estree from "prettier/plugins/estree";
import * as markdown from "prettier/plugins/markdown";
import * as postcss from "prettier/plugins/postcss";
import { small, large } from "./lib/corpus.mjs";
import { fnv1a, show } from "./lib/util.mjs";

const plugins = [babel, estree, markdown, postcss];
const js = await prettier.format(small, { parser: "babel", plugins });
console.log(js);
const big = await prettier.format(large, { parser: "babel", plugins, printWidth: 60, singleQuote: true });
show("large formatted lines", big.split("\n").length);
show("large formatted hash", fnv1a(big));
console.log(await prettier.format("#  Title\n*  one\n*   two\n\n|a|b|\n|-|-|\n|1|2|\n", { parser: "markdown", plugins }));
console.log(await prettier.format("a{color:red;margin:0 0 0 0}@media (max-width:100px){.b>.c{display:none}}", { parser: "css", plugins }));
try {
  await prettier.format("let = ;", { parser: "babel", plugins });
} catch (e) {
  show("error", e.constructor.name + " " + e.message.split("\n")[0]);
}
