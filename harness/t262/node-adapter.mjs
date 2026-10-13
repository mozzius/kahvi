// Reference engine adapter: lets the t262 runner drive Node, so the runner
// itself can be validated against a mature engine (it should pass the vast
// majority of in-scope tests). Speaks the contract in docs/ENGINE-CLI.md.
//
//   target/debug/t262 run built-ins/Array/ --mem-limit-mb 0 \
//     --engine "node --experimental-vm-modules harness/t262/node-adapter.mjs"

import fs from 'node:fs';
import path from 'node:path';
import vm from 'node:vm';
import { pathToFileURL } from 'node:url';

const argv = process.argv.slice(2);
const preludes = [];
let isModule = false;
let file;
for (let i = 0; i < argv.length; i++) {
  if (argv[i] === '--prelude') preludes.push(argv[++i]);
  else if (argv[i] === '--module') isModule = true;
  else file = argv[i];
}

let reported = false;
function report(e) {
  if (reported) return;
  reported = true;
  let line;
  if (e !== null && (typeof e === 'object' || typeof e === 'function')) {
    let name, message;
    try { name = e.constructor && e.constructor.name; } catch {}
    try { message = e.message; } catch {}
    line = `Uncaught ${name || 'Object'}: ${message}`;
  } else {
    line = `Uncaught ${String(e)}`;
  }
  process.stderr.write(line + '\n');
  process.exitCode = 1;
}
process.on('uncaughtException', report);
process.on('unhandledRejection', () => {}); // async tests report through $DONE

function newRealm() {
  const context = vm.createContext();
  const global = vm.runInContext('globalThis', context);
  const define = (k, v) =>
    Object.defineProperty(global, k, { value: v, writable: true, configurable: true, enumerable: false });
  const modules = new Map();
  const realm = { context, global, modules };
  define('print', (v) => { process.stdout.write(String(v) + '\n'); });
  define('$262', {
    global,
    createRealm: () => newRealm().global.$262,
    evalScript: (src) => runScript(realm, String(src), 'evalScript'),
    detachArrayBuffer: (buf) => { structuredClone(buf, { transfer: [buf] }); return null; },
    gc: () => { throw new Error('gc not available'); },
  });
  return realm;
}

function loadModule(realm, p) {
  let m = realm.modules.get(p);
  if (m) return m;
  if (p.endsWith('.json')) {
    const value = JSON.parse(fs.readFileSync(p, 'utf8'));
    m = new vm.SyntheticModule(['default'], function () { this.setExport('default', value); },
      { identifier: p, context: realm.context });
  } else {
    m = new vm.SourceTextModule(fs.readFileSync(p, 'utf8'), {
      identifier: p,
      context: realm.context,
      initializeImportMeta(meta) { meta.url = pathToFileURL(p).href; },
      importModuleDynamically: (spec, ref) => importDynamic(realm, spec, ref),
    });
  }
  realm.modules.set(p, m);
  return m;
}

const linker = (realm) => (spec, ref) => loadModule(realm, path.resolve(path.dirname(ref.identifier), spec));

async function importDynamic(realm, spec, ref) {
  const base = ref && ref.identifier && path.isAbsolute(ref.identifier) ? path.dirname(ref.identifier) : path.dirname(file);
  const m = loadModule(realm, path.resolve(base, String(spec)));
  if (m.status === 'unlinked') await m.link(linker(realm));
  if (m.status === 'linked') await m.evaluate();
  return m;
}

function runScript(realm, source, filename) {
  return new vm.Script(source, {
    filename,
    importModuleDynamically: (spec, ref) => importDynamic(realm, spec, ref),
  }).runInContext(realm.context);
}

const realm = newRealm();
try {
  for (const p of preludes) runScript(realm, fs.readFileSync(p, 'utf8'), p);
  if (isModule) {
    const root = loadModule(realm, path.resolve(file));
    await root.link(linker(realm));
    await root.evaluate();
  } else {
    runScript(realm, fs.readFileSync(file, 'utf8'), path.resolve(file));
  }
} catch (e) {
  report(e);
}
