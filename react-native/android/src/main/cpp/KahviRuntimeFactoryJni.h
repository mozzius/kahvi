// The Java/Kotlin side of KahviRuntimeFactory: an fbjni hybrid of React
// Native's JJSRuntimeFactory, like JHermesInstance.
#pragma once

#include <fbjni/fbjni.h>
#include <react/runtime/JSRuntimeFactory.h>
#include <react/runtime/jni/JJSRuntimeFactory.h>

#include "KahviInstance.h"
#include "kahvi.h"

namespace kahvi {

class KahviRuntimeFactoryJni
    : public facebook::jni::HybridClass<KahviRuntimeFactoryJni, facebook::react::JJSRuntimeFactory> {
 public:
  static constexpr auto kJavaDescriptor = "Ldev/kahvi/reactnative/KahviRuntimeFactory;";

  static facebook::jni::local_ref<jhybriddata> initHybrid(facebook::jni::alias_ref<jclass>) {
    return makeCxxInstance();
  }

  static void setCodeCacheDir(facebook::jni::alias_ref<jclass>, std::string path) {
    kahvi_set_code_cache_dir(path.data(), path.size());
  }

  static void registerNatives() {
    registerHybrid({
        makeNativeMethod("initHybrid", KahviRuntimeFactoryJni::initHybrid),
        makeNativeMethod("setCodeCacheDir", KahviRuntimeFactoryJni::setCodeCacheDir),
    });
  }

  std::unique_ptr<facebook::react::JSRuntime> createJSRuntime(
      std::shared_ptr<facebook::react::MessageQueueThread> /*msgQueueThread*/) noexcept override {
    return kahvi::createJSRuntime();
  }

 private:
  friend HybridBase;
};

} // namespace kahvi
