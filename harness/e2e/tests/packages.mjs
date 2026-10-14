// Bare specifiers: packages in node_modules, package.json "main" and
// "exports" (conditions, subpaths, patterns, null), and what is not exported.
import { name } from 'plain';
import { extra } from 'plain/extra.js';
import mainField from 'main-field';
import exported from '@scope/exported';
import { feature } from '@scope/exported/feature';
import { util } from '@scope/exported/lib/util';

console.log(name, extra);
console.log(mainField);
console.log(exported, feature, util);
for (const spec of ['@scope/exported/dist/feature.mjs', '@scope/exported/private/x', 'missing-package', '#internal']) {
  try {
    await import(spec);
  } catch (e) {
    console.log(spec, e.constructor.name);
  }
}
