# react-native-kahvi

kahvi as React Native's JavaScript engine, in place of Hermes. It needs
React Native 0.79 or later with the new architecture. It is built the same
way as `@react-native-community/javascriptcore`.

- `cpp/KahviJSI.{h,cpp}`: a `jsi::Runtime` over kahvi's C ABI
  (`crates/kahvi-ffi`). `kahvi::makeKahviRuntime()` makes one.
- `cpp/KahviInstance.{h,cpp}`: `kahvi::KahviRuntimeFactory`, React Native's
  `JSRuntimeFactory` for kahvi (what `HermesInstance` is for Hermes).
- `android/`: `dev.kahvi.reactnative.KahviRuntimeFactory` (Kotlin, with an
  fbjni hybrid) in `libkahviruntimefactory.so`, published with prefab so
  that other native libraries can call `makeKahviRuntime`.
- `apple/`: `jsrt_create_kahvi_factory()` and the `react-native-kahvi` pod.

## Status

What is checked:

- `harness/jsi/run.sh` runs the JSI shim against RN 0.86.3's JSI, normally
  and under `KAHVI_GC_STRESS=1`. CI runs it on every PR.
- `harness/rn/check.sh` compiles every C++ file here against RN 0.86.3's
  headers, folly and fbjni. CI runs it on every PR. It only checks syntax
  and types: nothing here has been linked or run in an app yet.

What is missing: an Android or iOS app build that uses the package.

## Building kahvi's library

The package links kahvi's C library, built from this repository:

```sh
react-native/scripts/build-android.sh   # android/libs/<abi>/libkahvi_ffi.a (NDK + cargo-ndk)
react-native/scripts/build-apple.sh     # apple/KahviFFI.xcframework (macOS + Xcode)
```

## Using it

Hermes compiles the bundle to Hermes bytecode, so turn Hermes off.

### Android

In `android/gradle.properties`:

```properties
hermesEnabled=false
```

In `MainApplication`, pass the factory to the React host:

```kotlin
import dev.kahvi.reactnative.KahviRuntimeFactory

override val reactHost: ReactHost
  get() = getDefaultReactHost(applicationContext, reactNativeHost, KahviRuntimeFactory(applicationContext))
```

### react-native-worklets (Reanimated 4)

Worklets makes its UI-thread runtimes with `makeHermesRuntime`.
`patches/react-native-worklets+0.13.0.patch` (for patch-package) makes
them kahvi runtimes instead, links worklets to this package, and turns off
its Hermes sampling profiler. `harness/rn/check.sh` compiles the patched
files.

### Expo

Add the config plugin, which does the Android steps above at prebuild:

```js
plugins: ["react-native-kahvi"]
```

Expo 57's `ExpoReactHostFactory.getDefaultReactHost` takes a
`jsRuntimeFactory` but always makes a Hermes one;
`patches/expo+57.0.24.patch` makes it use the one it is given.
`harness/rn/social-app-android.sh` builds Bluesky's app this way
(`.github/workflows/social-app-android.yml`).

### iOS

Install pods with `USE_HERMES=0`, then return the factory from the React
Native factory delegate:

```swift
import ReactKahvi

override func createJSRuntimeFactory() -> JSRuntimeFactoryRef {
  jsrt_create_kahvi_factory()
}
```

Learned from Bluesky's app running in an iOS Release build (2026-10-16):

- Don't set `USE_THIRD_PARTY_JSC=1` to turn Hermes off on RN 0.86: it makes
  the pods depend on `React-jsc`. The pods (Expo's too) need the non-Hermes
  compile flags instead, including `-DUSE_HERMES=0`.
- Expo's "Eager bundle does not match new options" message does not mean
  the engine changed. If `ios/Podfile.lock` still lists Hermes, the Release
  bundle is Hermes bytecode, which kahvi rejects: regenerate the pods with
  Hermes off.
- Expo 57 apps also need a patch to `ExpoModulesJSI` 57.1.0 so its
  standalone runtime helper makes kahvi runtimes, besides the worklets
  patch above.
