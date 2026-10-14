// Host shim for V8's mjsunit, evaluated before mjsunit.js (see run.py).
// The prepared copy of the suite calls V8 runtime functions as
// __v8_natives.Name(...). Only these are defined: the optimization hints,
// which have no effect a program can observe, and ArrayBufferDetach, which is
// the spec's DetachArrayBuffer. Tests that call any other runtime function are
// skipped, so they never reach this object.
var __v8_natives = (function () {
  var natives = Object.create(null);
  var hints = [
    "PrepareFunctionForOptimization", "OptimizeFunctionOnNextCall",
    "OptimizeMaglevOnNextCall", "NeverOptimizeFunction", "OptimizeOsr",
    "DeoptimizeFunction", "DeoptimizeNow", "EnsureFeedbackVectorForFunction",
    "ClearFunctionFeedback", "CompileBaseline", "BaselineOsr",
    "DisableOptimizationFinalization", "FinalizeOptimization",
    "WaitForBackgroundOptimization", "PretenureAllocationSite",
  ];
  for (var i = 0; i < hints.length; i++) natives[hints[i]] = function () {};
  natives.ArrayBufferDetach = function (buffer) { $262.detachArrayBuffer(buffer); };
  return Object.freeze(natives);
})();
