#include "KahviInstance.h"

#include "KahviJSI.h"

namespace kahvi {

std::unique_ptr<facebook::react::JSRuntime> createJSRuntime() noexcept {
  std::unique_ptr<facebook::jsi::Runtime> runtime = makeKahviRuntime();
  // React Native reports the engine from Error.prototype.jsEngine, as
  // HermesInstance sets it.
  auto errorPrototype = runtime->global()
                            .getPropertyAsObject(*runtime, "Error")
                            .getPropertyAsObject(*runtime, "prototype");
  errorPrototype.setProperty(*runtime, "jsEngine", "kahvi");
  return std::make_unique<facebook::react::JSIRuntimeHolder>(std::move(runtime));
}

std::unique_ptr<facebook::react::JSRuntime> KahviRuntimeFactory::createJSRuntime(
    std::shared_ptr<facebook::react::MessageQueueThread> /*msgQueueThread*/) noexcept {
  return kahvi::createJSRuntime();
}

} // namespace kahvi
