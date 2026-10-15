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

What is missing: an Android or iOS app build that uses the package, and a
`react-native-worklets` build over kahvi. Worklets makes its UI-thread
runtime with `makeHermesRuntime`.

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
  get() = getDefaultReactHost(applicationContext, reactNativeHost, KahviRuntimeFactory())
```

### iOS

Install pods with `USE_HERMES=0`, then return the factory from the React
Native factory delegate:

```swift
import ReactKahvi

override func createJSRuntimeFactory() -> JSRuntimeFactoryRef {
  jsrt_create_kahvi_factory()
}
```
