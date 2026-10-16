# social-app on kahvi, iOS Release (Samuel's working setup)

Copied as-is from Samuel's social-app worktree, for folding into react-native/.
social-app base: 477e04b67b818823dad398dbbad7e6c88128aa40 Nightly source-language update

- plugins/withKahviRuntime.js: Expo config plugin. Adds `import ReactKahvi` and a
  `createJSRuntimeFactory() -> jsrt_create_kahvi_factory()` override to the generated
  AppDelegate, sets `expo.jsEngine = jsc` in Podfile.properties, and prepends
  `USE_HERMES=0` / `USE_KAHVI=1` to the Podfile.
- patches/: the full pnpm patches social-app needs (RN 0.86.3, Expo 57.0.24,
  ExpoModulesCore 57.0.18, ExpoModulesJSI 57.1.0, Reanimated 4.7.1, worklets 0.13.0),
  each with its .md note. Several extend patches social-app already carries;
  social-app-patches-vs-upstream.diff shows only what changed relative to social-app main.
- social-app-config.diff: app.config.js (jsEngine jsc, plugin), package.json
  (`ios` script passes USE_HERMES=0 USE_KAHVI=1, react-native-kahvi link), pnpm-workspace.yaml.

Learnings from the handoff:
- build-apple.sh needs IPHONEOS_DEPLOYMENT_TARGET=15.1 (Rust's default iOS 10 target hides
  ___chkstk_darwin). KahviJSI.h wraps its C++ declarations in #ifdef __cplusplus so Swift module
  scanning doesn't try to include <memory>. Both are in this branch.
- Don't set USE_THIRD_PARTY_JSC=1 (pods then depend on React-jsc). USE_KAHVI selects kahvi and
  emits the non-Hermes flags (-DUSE_HERMES=0); Expo's own podspec needs those flags too.
- "Eager bundle does not match new options" only means Expo rebundles. If Podfile.lock still
  has Hermes, the Release bundle is Hermes bytecode and kahvi rejects it: regenerate Pods with
  the kahvi Podfile flags.
