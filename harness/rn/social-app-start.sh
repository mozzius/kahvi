#!/usr/bin/env bash
# Installs the social-app APK from harness/rn/social-app-android.sh on the
# running emulator, starts it, and saves its log and a screenshot to OUT_DIR.
# Fails if the app is not running a minute later.
#   harness/rn/social-app-start.sh APK_DIR OUT_DIR
set -euo pipefail
apk="$(ls "$1"/*.apk | head -1)"
out="$2"
mkdir -p "$out"
app=xyz.blueskyweb.app
adb install -r "$apk"
adb logcat -c
adb shell monkey -p "$app" -c android.intent.category.LAUNCHER 1
sleep 60
adb exec-out screencap -p >"$out/screen.png"
adb logcat -d >"$out/logcat.txt"
adb logcat -d -b crash >"$out/crash.txt" || true
grep -E 'ReactNativeJS|kahvi|AndroidRuntime|libc  |DEBUG   ' "$out/logcat.txt" >"$out/app.txt" || true
tail -n 200 "$out/app.txt"
if ! adb shell pidof "$app" >/dev/null; then
  echo "$app is not running"
  cat "$out/crash.txt"
  exit 1
fi
echo "$app is running"
