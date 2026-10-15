#include <fbjni/fbjni.h>

#include "KahviRuntimeFactoryJni.h"

JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM* vm, void* /*reserved*/) {
  return facebook::jni::initialize(vm, [] { kahvi::KahviRuntimeFactoryJni::registerNatives(); });
}
