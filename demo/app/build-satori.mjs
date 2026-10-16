// Bundles satori into one ES module for kahvi, which has no CommonJS and
// no WebAssembly. Runs on Node at build time (npm run build). satori 0.0.44
// is the last release whose layout engine is JavaScript: an asm.js build of
// Yoga (yoga-layout-prebuilt); later ones load Yoga as WebAssembly.
// yoga's Emscripten glue takes any runtime with `process` and `require` for
// Node and reaches for `fs`; it is told it is on the web instead (it only
// uses that to pick console.log for output). It also builds a UTF-16
// TextDecoder, which kahvi does not have; its own fallback is used instead.
// It is sloppy-mode code bundled into a strict module, hence `var _a`.
import { build } from 'esbuild';
import { readFile } from 'node:fs/promises';

await build({
  stdin: { contents: "export { default } from 'satori';", resolveDir: import.meta.dirname },
  bundle: true,
  format: 'esm',
  platform: 'neutral',
  mainFields: ['module', 'main'],
  minify: true,
  outfile: `${import.meta.dirname}/vendor/satori.mjs`,
  define: { 'process.env.NODE_ENV': '"production"' },
  plugins: [{
    name: 'yoga-not-node',
    setup(b) {
      b.onLoad({ filter: /yoga-layout-prebuilt[\\/].*nbind\.js$/ }, async (args) => {
        const src = await readFile(args.path, 'utf8');
        let contents = src;
        for (const [from, to] of [
          ['ENVIRONMENT_IS_WEB = typeof window === "object"', 'ENVIRONMENT_IS_WEB = true'],
          ['UTF16Decoder = typeof TextDecoder !== "undefined" ? new TextDecoder("utf-16le") : undefined', 'UTF16Decoder = undefined'],
        ]) {
          if (!contents.includes(from)) throw new Error(`yoga-layout-prebuilt: ${from} not found`);
          contents = contents.replace(from, to);
        }
        // Written for sloppy mode, where this assignment makes a global; a
        // bundled ES module is strict.
        return { contents: `var _a;\n${contents}`, loader: 'js' };
      });
    },
  }],
});
