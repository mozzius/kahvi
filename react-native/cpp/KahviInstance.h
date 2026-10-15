// React Native's engine interface (JSRuntimeFactory, RN >= 0.79 new
// architecture) for kahvi, shared by Android and Apple platforms.
#pragma once

#include <memory>

#include <react/runtime/JSRuntimeFactory.h>

namespace kahvi {

/// Creates a kahvi runtime for React Native's JS thread.
std::unique_ptr<facebook::react::JSRuntime> createJSRuntime() noexcept;

/// A JSRuntimeFactory making kahvi runtimes (what HermesInstance is for Hermes).
class KahviRuntimeFactory : public facebook::react::JSRuntimeFactory {
 public:
  std::unique_ptr<facebook::react::JSRuntime> createJSRuntime(
      std::shared_ptr<facebook::react::MessageQueueThread> msgQueueThread) noexcept override;
};

} // namespace kahvi
