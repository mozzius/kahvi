#!/usr/bin/env bash
# Compiles (syntax only) the React Native engine package (react-native/)
# against React Native's own headers, pinned to RN v0.86.3 (social-app's
# version), with folly and fbjni headers and the JDK's jni.h. Building and
# running it in an app needs the Android NDK or Xcode: see
# react-native/README.md.
#   harness/rn/check.sh
set -euo pipefail
cd "$(dirname "$0")/../.."
WORK="${KAHVI_WORK:-$PWD/work}/rn-headers"
RN_TAG=v0.86.3
FOLLY_TAG=v2024.11.18.00
FBJNI_TAG=v0.7.0
mkdir -p "$WORK"
if [ ! -d "$WORK/react-native-$RN_TAG" ]; then
  git clone -q --depth 1 --branch "$RN_TAG" --filter=blob:none --sparse \
    https://github.com/facebook/react-native.git "$WORK/react-native-$RN_TAG"
  git -C "$WORK/react-native-$RN_TAG" sparse-checkout set \
    packages/react-native/ReactCommon packages/react-native/ReactAndroid/src/main/jni/react/runtime
fi
if [ ! -d "$WORK/folly-$FOLLY_TAG" ]; then
  git clone -q --depth 1 --branch "$FOLLY_TAG" --filter=blob:none --sparse \
    https://github.com/facebook/folly.git "$WORK/folly-$FOLLY_TAG"
  git -C "$WORK/folly-$FOLLY_TAG" sparse-checkout set folly
fi
if [ ! -d "$WORK/fbjni-$FBJNI_TAG" ]; then
  git clone -q --depth 1 --branch "$FBJNI_TAG" \
    https://github.com/facebookincubator/fbjni.git "$WORK/fbjni-$FBJNI_TAG"
fi
RN="$WORK/react-native-$RN_TAG/packages/react-native"
JDK="${JAVA_HOME:-$(dirname "$(dirname "$(readlink -f "$(command -v javac || command -v java)")")")}"
FOLLY_FLAGS=(-DFOLLY_NO_CONFIG=1 -DFOLLY_HAVE_CLOCK_GETTIME=1 -DFOLLY_CFG_NO_COROUTINES=1
  -DFOLLY_MOBILE=1 -DFOLLY_HAVE_PTHREAD=1)
INCLUDES=(-Ireact-native/cpp -Icrates/kahvi-ffi/include -I"$WORK/folly-$FOLLY_TAG"
  -I"$RN/ReactCommon" -I"$RN/ReactCommon/jsi" -I"$RN/ReactCommon/jsitooling"
  -I"$RN/ReactCommon/callinvoker" -I"$RN/ReactCommon/runtimeexecutor"
  -I"$RN/ReactAndroid/src/main/jni" -I"$WORK/fbjni-$FBJNI_TAG/cxx"
  -I"$JDK/include" -I"$JDK/include/linux")
CXX="${CXX:-clang++}"
check() {
  "$CXX" -std=c++20 -fsyntax-only -Wall -Wextra -Werror -Wno-unused-parameter \
    "${FOLLY_FLAGS[@]}" "${INCLUDES[@]}" "$@"
}
check react-native/cpp/KahviJSI.cpp
check react-native/cpp/KahviInstance.cpp
check react-native/android/src/main/cpp/OnLoad.cpp
# The Apple factory is Objective-C++ only in its #import lines.
sed 's/^#import/#include/' react-native/apple/RCTKahviInstanceFactory.mm > "$WORK/RCTKahviInstanceFactory.cpp"
check -Ireact-native/apple "$WORK/RCTKahviInstanceFactory.cpp"
echo "rn: all files compile against React Native $RN_TAG"
