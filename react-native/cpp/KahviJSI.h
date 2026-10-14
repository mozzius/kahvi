// A jsi::Runtime over kahvi's C ABI (crates/kahvi-ffi/include/kahvi.h).
#pragma once

#include <memory>

#include <jsi/jsi.h>

namespace kahvi {

/// A new runtime with its own realm. Like every jsi::Runtime it is used by
/// one thread at a time; all jsi values must be released before it is.
std::unique_ptr<facebook::jsi::Runtime> makeKahviRuntime();

} // namespace kahvi
