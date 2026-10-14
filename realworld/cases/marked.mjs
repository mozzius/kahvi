// marked: render GitHub-flavoured Markdown to HTML.
import { marked, Lexer } from "marked";
import { fnv1a, show } from "./lib/util.mjs";

const md = `# Kahvi *engine*

A [JavaScript](https://tc39.es "spec") engine in **Rust**, with \`inline code\`.

1. First
   - nested *item*
   - [ ] task
2. Second

> quote with a  
> hard break

| Feature | Status |
|:--|--:|
| ESM | done |
| CJS | ~~no~~ |

\`\`\`js
const x = 1 < 2 && "a" > 'b';
\`\`\`

***

Autolink https://example.com and <b>raw html</b>&amp; escapes \\* here.
`;
console.log(marked.parse(md, { gfm: true }));
show("tokens", new Lexer().lex(md).map((t) => t.type));
const long = Array.from({ length: 400 }, (_, i) => `## Section ${i}\n\n- item *${i}* with [link](#${i})\n- \`code ${i}\`\n\nParagraph ${i} **bold** _em_.\n`).join("\n");
const html = marked.parse(long);
show("long html length", html.length);
show("long html hash", fnv1a(html));
