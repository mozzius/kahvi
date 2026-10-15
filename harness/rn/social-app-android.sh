#!/usr/bin/env bash
# Builds bluesky-social/social-app's Android release APK with kahvi in place
# of Hermes (react-native/README.md). Needs the Android SDK and NDK, JDK 17+,
# Node 24, pnpm and cargo-ndk; .github/workflows/social-app-android.yml runs it.
#   harness/rn/social-app-android.sh REF ABI OUT_DIR
# REF is a social-app commit; ABI is one Android ABI (x86_64 for an emulator,
# arm64-v8a for a phone). The APK lands in OUT_DIR.
set -euo pipefail
ref="$1"
abi="$2"
out="$(mkdir -p "$3" && cd "$3" && pwd)"
repo="$(cd "$(dirname "$0")/../.." && pwd)"
pkg="$repo/react-native"
work="${KAHVI_WORK:-$repo/work}/social-app"

# kahvi's static library and header, inside the package.
"$pkg/scripts/build-android.sh" "$abi"

rm -rf "$work"
git init -q "$work"
git -C "$work" fetch -q --depth 1 https://github.com/bluesky-social/social-app "$ref"
git -C "$work" checkout -q FETCH_HEAD
cd "$work"

pnpm install --frozen-lockfile
pnpm add "react-native-kahvi@file:$pkg"

# Patches for packages that embed Hermes themselves (react-native/patches).
# pnpm links node_modules/<name> into its store, so patch the real directory.
for p in "$pkg"/patches/*.patch; do
  name="$(basename "$p" .patch)"
  name="${name%+*}"
  (cd "$(realpath "node_modules/$name")" && patch -p3 --forward <"$p")
done

# The config plugin turns Hermes off and hands Expo kahvi's runtime factory.
node -e '
  const fs = require("fs");
  const f = "app.config.js";
  const src = fs.readFileSync(f, "utf8");
  const out = src.replace(/plugins:\s*\[/, (m) => m + "\n      \"react-native-kahvi\",");
  if (out === src) throw new Error("no plugins array in app.config.js");
  fs.writeFileSync(f, out);
'

EXPO_NO_GIT_STATUS=1 pnpm exec expo prebuild --platform android --clean --no-install
grep -q 'hermesEnabled=false' android/gradle.properties
grep -q 'KahviRuntimeFactory()' android/app/src/main/java/xyz/blueskyweb/app/MainApplication.kt

cd android
SENTRY_DISABLE_AUTO_UPLOAD=true ./gradlew --no-daemon assembleRelease \
  -PreactNativeArchitectures="$abi"
cp app/build/outputs/apk/release/*.apk "$out/"
ls -l "$out"
