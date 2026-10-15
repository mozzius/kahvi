// diff: line, word and character diffs, unified patches and applying them.
import { diffLines, diffWords, diffChars, createTwoFilesPatch, applyPatch, parsePatch, structuredPatch } from "diff";
import { show } from "./lib/util.mjs";

const a = "one\ntwo\nthree\nfour\nfive\nsix\nseven\n";
const b = "one\n2\nthree\nfour\nfive and a half\nsix\nseven\neight\n";
show("lines", diffLines(a, b).map((p) => (p.added ? "+" : p.removed ? "-" : "=") + p.count));
show("words", diffWords("the quick brown fox", "the slow brown dog jumps").map((p) => (p.added ? "+" : p.removed ? "-" : "=") + p.value));
show("chars", diffChars("kitten", "sitting").map((p) => (p.added ? "+" : p.removed ? "-" : "=") + p.value).join("|"));
const patch = createTwoFilesPatch("a.txt", "b.txt", a, b, "old", "new", { context: 1 });
console.log(patch);
show("apply", applyPatch(a, patch) === b);
show("parse", parsePatch(patch)[0].hunks.map((h) => [h.oldStart, h.oldLines, h.newStart, h.newLines]));
show("structured", structuredPatch("x", "y", "a\nb\nc\n", "a\nc\nd\n").hunks[0].lines);
const big1 = Array.from({ length: 2000 }, (_, i) => "line " + i).join("\n");
const big2 = big1.replace("line 500", "LINE 500").replace("line 1500\n", "");
show("big", diffLines(big1, big2).filter((p) => p.added || p.removed).map((p) => (p.added ? "+" : "-") + p.value.trim()));
