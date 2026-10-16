#!/usr/bin/env bash
# Builds kahvi's C library for iOS devices and simulators into
# apple/KahviFFI.xcframework and copies its header to include/. Run on macOS
# with Xcode (`rustup target add aarch64-apple-ios aarch64-apple-ios-sim
# x86_64-apple-ios`).
set -euo pipefail
# The pod's minimum iOS version. Rust's default (iOS 10) links against
# symbols such as ___chkstk_darwin that the app's newer target hides.
export IPHONEOS_DEPLOYMENT_TARGET="${IPHONEOS_DEPLOYMENT_TARGET:-15.1}"
pkg="$(cd "$(dirname "$0")/.." && pwd)"
repo="$(cd "$pkg/.." && pwd)"
mkdir -p "$pkg/include"
cp "$repo/crates/kahvi-ffi/include/kahvi.h" "$pkg/include/"
cd "$repo"
for t in aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios; do
  cargo build --release -p kahvi-ffi --target "$t"
done
sim="target/apple-ios-sim/release"
mkdir -p "$sim"
lipo -create target/aarch64-apple-ios-sim/release/libkahvi_ffi.a \
  target/x86_64-apple-ios/release/libkahvi_ffi.a -output "$sim/libkahvi_ffi.a"
rm -rf "$pkg/apple/KahviFFI.xcframework"
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libkahvi_ffi.a \
  -library "$sim/libkahvi_ffi.a" \
  -output "$pkg/apple/KahviFFI.xcframework"
