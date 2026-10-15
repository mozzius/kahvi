#!/usr/bin/env bash
# Builds kahvi's C library for each Android ABI into android/libs/<abi>/ and
# copies its header to include/. Needs the Android NDK and cargo-ndk
# (`cargo install cargo-ndk`; `rustup target add aarch64-linux-android
# armv7-linux-androideabi x86_64-linux-android i686-linux-android`).
#   react-native/scripts/build-android.sh [abi ...]   (default: all four)
set -euo pipefail
pkg="$(cd "$(dirname "$0")/.." && pwd)"
repo="$(cd "$pkg/.." && pwd)"
abis=("$@")
[ ${#abis[@]} -gt 0 ] || abis=(arm64-v8a armeabi-v7a x86_64 x86)
mkdir -p "$pkg/include"
cp "$repo/crates/kahvi-ffi/include/kahvi.h" "$pkg/include/"
for abi in "${abis[@]}"; do
  (cd "$repo" && cargo ndk --platform 24 -t "$abi" build --release -p kahvi-ffi)
  case "$abi" in
    arm64-v8a) triple=aarch64-linux-android ;;
    armeabi-v7a) triple=armv7-linux-androideabi ;;
    x86_64) triple=x86_64-linux-android ;;
    x86) triple=i686-linux-android ;;
    *) echo "unknown ABI $abi" >&2; exit 2 ;;
  esac
  mkdir -p "$pkg/android/libs/$abi"
  cp "$repo/target/$triple/release/libkahvi_ffi.a" "$pkg/android/libs/$abi/"
done
