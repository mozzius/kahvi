#!/usr/bin/env bash
# Builds the JSI shim (react-native/cpp) against React Native's JSI headers
# and runs harness/jsi/test.cpp. JSI is fetched once, pinned to RN v0.76.0.
#   harness/jsi/run.sh            (set KAHVI_GC_STRESS=1 to collect on every allocation)
set -euo pipefail
cd "$(dirname "$0")/../.."
WORK="${KAHVI_WORK:-$PWD/work}"
JSI="$WORK/jsi-v0.76.0"
if [ ! -f "$JSI/jsi/jsi.cpp" ]; then
  mkdir -p "$JSI/jsi"
  base=https://raw.githubusercontent.com/facebook/react-native/v0.76.0/packages/react-native/ReactCommon/jsi/jsi
  for f in jsi.h jsi-inl.h jsi.cpp instrumentation.h; do
    curl -sSfL "$base/$f" -o "$JSI/jsi/$f"
  done
fi
cargo build -q --release -p kahvi-ffi
out="$WORK/jsi-test"
${CXX:-g++} -std=c++17 -O1 -g -Wall -Wextra -Werror -Wno-unused-parameter \
  -I"$JSI" -Icrates/kahvi-ffi/include -Ireact-native/cpp \
  "$JSI/jsi/jsi.cpp" react-native/cpp/KahviJSI.cpp harness/jsi/test.cpp \
  target/release/libkahvi_ffi.a -lpthread -ldl -lm -o "$out"
"$out"
