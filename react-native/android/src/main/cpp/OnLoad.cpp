#include <fbjni/fbjni.h>

#include "kahvi.h"

#include "KahviRuntimeFactoryJni.h"

JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM* vm, void* /*reserved*/) {
  return facebook::jni::initialize(vm, [] {
    kahvi::KahviRuntimeFactoryJni::registerNatives();
    // kahvi runs deep JavaScript calls on its own larger-stack threads; host
    // functions called there use JNI, so attach them with the app's class
    // loader.
    kahvi_set_thread_wrapper([](void (*run)(void*), void* data) {
      facebook::jni::ThreadScope::WithClassLoader([&] { run(data); });
    });
  });
}
