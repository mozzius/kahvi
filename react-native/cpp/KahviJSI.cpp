// A jsi::Runtime over kahvi's C ABI. Every jsi pointer value (string, symbol,
// bigint, object, property name) owns one KahviValue handle, released when
// jsi invalidates it. Host callbacks turn C++ exceptions into JS throws:
// a jsi::JSError rethrows its value, anything else becomes an Error whose
// message is what().

#include "KahviJSI.h"

#include <string>
#include <vector>

#include <kahvi.h>

namespace kahvi {

namespace jsi = facebook::jsi;

namespace {

class KahviRuntimeImpl final : public jsi::Runtime {
 public:
  KahviRuntimeImpl() : rt_(kahvi_runtime_new()) {}

  ~KahviRuntimeImpl() override {
    kahvi_runtime_free(rt_);
  }

  // Runtime

  jsi::Value evaluateJavaScript(
      const std::shared_ptr<const jsi::Buffer>& buffer,
      const std::string& sourceURL) override {
    KahviValue out{};
    check(kahvi_eval(
        rt_,
        reinterpret_cast<const char*>(buffer->data()),
        buffer->size(),
        sourceURL.data(),
        sourceURL.size(),
        &out));
    return take(out);
  }

  std::shared_ptr<const jsi::PreparedJavaScript> prepareJavaScript(
      const std::shared_ptr<const jsi::Buffer>& buffer,
      std::string sourceURL) override {
    // kahvi has no serialized bytecode: preparing keeps the source.
    return std::make_shared<Prepared>(buffer, std::move(sourceURL));
  }

  jsi::Value evaluatePreparedJavaScript(
      const std::shared_ptr<const jsi::PreparedJavaScript>& js) override {
    auto prepared = std::static_pointer_cast<const Prepared>(js);
    return evaluateJavaScript(prepared->buffer, prepared->sourceURL);
  }

  void queueMicrotask(const jsi::Function& callback) override {
    kahvi_queue_microtask(rt_, borrow(callback));
  }

  bool drainMicrotasks(int /*maxMicrotasksHint*/) override {
    // The hint allows stopping early; kahvi always drains the queue.
    check(kahvi_run_jobs(rt_));
    return true;
  }

  jsi::Object global() override {
    return takeAs<jsi::Object>(kahvi_global(rt_));
  }

  std::string description() override {
    return "kahvi";
  }

  bool isInspectable() override {
    return false;
  }

 protected:
  // Pointer values

  PointerValue* cloneSymbol(const PointerValue* pv) override {
    return clonePointer(pv);
  }
  PointerValue* cloneBigInt(const PointerValue* pv) override {
    return clonePointer(pv);
  }
  PointerValue* cloneString(const PointerValue* pv) override {
    return clonePointer(pv);
  }
  PointerValue* cloneObject(const PointerValue* pv) override {
    return clonePointer(pv);
  }
  PointerValue* clonePropNameID(const PointerValue* pv) override {
    return clonePointer(pv);
  }

  // Property names: a string or a symbol.

  jsi::PropNameID createPropNameIDFromAscii(const char* str, size_t length)
      override {
    return takeAs<jsi::PropNameID>(kahvi_string_new(rt_, str, length));
  }
  jsi::PropNameID createPropNameIDFromUtf8(const uint8_t* utf8, size_t length)
      override {
    return takeAs<jsi::PropNameID>(kahvi_string_new(
        rt_, reinterpret_cast<const char*>(utf8), length));
  }
  jsi::PropNameID createPropNameIDFromString(const jsi::String& str) override {
    return takeAs<jsi::PropNameID>(kahvi_value_clone(borrow(str)));
  }
  jsi::PropNameID createPropNameIDFromSymbol(const jsi::Symbol& sym) override {
    return takeAs<jsi::PropNameID>(kahvi_value_clone(borrow(sym)));
  }
  std::string utf8(const jsi::PropNameID& name) override {
    KahviValue v = borrow(name);
    if (v.tag == KAHVI_SYMBOL) {
      return ownedUtf8(kahvi_symbol_to_string(rt_, v));
    }
    return stringUtf8(v);
  }
  bool compare(const jsi::PropNameID& a, const jsi::PropNameID& b) override {
    return kahvi_strict_equals(rt_, borrow(a), borrow(b));
  }

  std::string symbolToString(const jsi::Symbol& sym) override {
    return ownedUtf8(kahvi_symbol_to_string(rt_, borrow(sym)));
  }

  // BigInts

  jsi::BigInt createBigIntFromInt64(int64_t v) override {
    return takeAs<jsi::BigInt>(kahvi_bigint_from_i64(rt_, v));
  }
  jsi::BigInt createBigIntFromUint64(uint64_t v) override {
    return takeAs<jsi::BigInt>(kahvi_bigint_from_u64(rt_, v));
  }
  bool bigintIsInt64(const jsi::BigInt& b) override {
    bool i64 = false, u64 = false;
    kahvi_bigint_truncate(rt_, borrow(b), &i64, &u64);
    return i64;
  }
  bool bigintIsUint64(const jsi::BigInt& b) override {
    bool i64 = false, u64 = false;
    kahvi_bigint_truncate(rt_, borrow(b), &i64, &u64);
    return u64;
  }
  uint64_t truncate(const jsi::BigInt& b) override {
    bool i64 = false, u64 = false;
    return kahvi_bigint_truncate(rt_, borrow(b), &i64, &u64);
  }
  jsi::String bigintToString(const jsi::BigInt& b, int radix) override {
    if (radix < 2 || radix > 36) {
      throw jsi::JSINativeException("Invalid radix " + std::to_string(radix));
    }
    return takeAs<jsi::String>(
        kahvi_bigint_to_string(rt_, borrow(b), static_cast<uint32_t>(radix)));
  }

  // Strings

  jsi::String createStringFromAscii(const char* str, size_t length) override {
    return takeAs<jsi::String>(kahvi_string_new(rt_, str, length));
  }
  jsi::String createStringFromUtf8(const uint8_t* utf8, size_t length)
      override {
    return takeAs<jsi::String>(kahvi_string_new(
        rt_, reinterpret_cast<const char*>(utf8), length));
  }
  std::string utf8(const jsi::String& str) override {
    return stringUtf8(borrow(str));
  }

  // Objects

  jsi::Object createObject() override {
    return takeAs<jsi::Object>(kahvi_object_new(rt_));
  }

  jsi::Object createObject(std::shared_ptr<jsi::HostObject> ho) override {
    static const KahviHostObjectClass cls = {
        hostObjectGet, hostObjectSet, hostObjectPropertyNames, hostObjectFinalize};
    auto* context = new HostObjectContext{this, std::move(ho)};
    return takeAs<jsi::Object>(kahvi_host_object_new(rt_, &cls, context));
  }

  std::shared_ptr<jsi::HostObject> getHostObject(const jsi::Object& o) override {
    auto* context =
        static_cast<HostObjectContext*>(kahvi_host_object_context(rt_, borrow(o)));
    if (!context) {
      throw jsi::JSINativeException("Object is not a HostObject");
    }
    return context->hostObject;
  }

  jsi::HostFunctionType& getHostFunction(const jsi::Function& f) override {
    auto* context = static_cast<HostFunctionContext*>(
        kahvi_host_function_context(rt_, borrow(f)));
    if (!context) {
      throw jsi::JSINativeException("Function is not a HostFunction");
    }
    return context->function;
  }

  bool hasNativeState(const jsi::Object& o) override {
    return kahvi_get_native_state(rt_, borrow(o)) != nullptr;
  }

  std::shared_ptr<jsi::NativeState> getNativeState(const jsi::Object& o) override {
    auto* context = static_cast<NativeStateContext*>(
        kahvi_get_native_state(rt_, borrow(o)));
    return context ? context->state : nullptr;
  }

  void setNativeState(const jsi::Object& o, std::shared_ptr<jsi::NativeState> state)
      override {
    auto* context = new NativeStateContext{std::move(state)};
    if (!kahvi_set_native_state(rt_, borrow(o), context, nativeStateFinalize)) {
      delete context;
      throw jsi::JSINativeException("Native state cannot be set on this object");
    }
  }

  jsi::Value getProperty(const jsi::Object& o, const jsi::PropNameID& name)
      override {
    return get(borrow(o), borrow(name));
  }
  jsi::Value getProperty(const jsi::Object& o, const jsi::String& name) override {
    return get(borrow(o), borrow(name));
  }
  bool hasProperty(const jsi::Object& o, const jsi::PropNameID& name) override {
    return has(borrow(o), borrow(name));
  }
  bool hasProperty(const jsi::Object& o, const jsi::String& name) override {
    return has(borrow(o), borrow(name));
  }
  void setPropertyValue(
      const jsi::Object& o,
      const jsi::PropNameID& name,
      const jsi::Value& value) override {
    check(kahvi_set(rt_, borrow(o), borrow(name), borrow(value)));
  }
  void setPropertyValue(
      const jsi::Object& o,
      const jsi::String& name,
      const jsi::Value& value) override {
    check(kahvi_set(rt_, borrow(o), borrow(name), borrow(value)));
  }

  bool isArray(const jsi::Object& o) const override {
    return kahvi_is_array(rt_, borrow(o));
  }
  bool isArrayBuffer(const jsi::Object& o) const override {
    return kahvi_is_array_buffer(rt_, borrow(o));
  }
  bool isFunction(const jsi::Object& o) const override {
    return kahvi_is_function(rt_, borrow(o));
  }
  bool isHostObject(const jsi::Object& o) const override {
    return kahvi_host_object_context(rt_, borrow(o)) != nullptr;
  }
  bool isHostFunction(const jsi::Function& f) const override {
    return kahvi_host_function_context(rt_, borrow(f)) != nullptr;
  }

  jsi::Array getPropertyNames(const jsi::Object& o) override {
    KahviValue out{};
    check(kahvi_property_names(rt_, borrow(o), &out));
    return takeAs<jsi::Array>(out);
  }

  jsi::WeakObject createWeakObject(const jsi::Object& o) override {
    KahviValue out{};
    check(kahvi_weak_new(rt_, borrow(o), &out));
    return takeAs<jsi::WeakObject>(out);
  }
  jsi::Value lockWeakObject(const jsi::WeakObject& w) override {
    KahviValue out{};
    check(kahvi_weak_lock(rt_, borrow(w), &out));
    return take(out);
  }

  // Arrays and ArrayBuffers

  jsi::Array createArray(size_t length) override {
    KahviValue out{};
    check(kahvi_array_new(rt_, length, &out));
    return takeAs<jsi::Array>(out);
  }

  jsi::ArrayBuffer createArrayBuffer(
      std::shared_ptr<jsi::MutableBuffer> buffer) override {
    // The buffer keeps the MutableBuffer alive until it is detached or
    // collected.
    uint8_t* data = buffer->data();
    size_t length = buffer->size();
    auto* holder = new std::shared_ptr<jsi::MutableBuffer>(std::move(buffer));
    KahviValue out;
    check(kahvi_array_buffer_new_external(
        rt_, data, length, holder,
        [](void* context) {
          delete static_cast<std::shared_ptr<jsi::MutableBuffer>*>(context);
        },
        &out));
    return takeAs<jsi::ArrayBuffer>(out);
  }

  size_t size(const jsi::Array& a) override {
    size_t length = 0;
    check(kahvi_length(rt_, borrow(a), &length));
    return length;
  }
  size_t size(const jsi::ArrayBuffer& b) override {
    size_t length = 0;
    kahvi_array_buffer_data(rt_, borrow(b), &length);
    return length;
  }
  uint8_t* data(const jsi::ArrayBuffer& b) override {
    size_t length = 0;
    return kahvi_array_buffer_data(rt_, borrow(b), &length);
  }

  jsi::Value getValueAtIndex(const jsi::Array& a, size_t i) override {
    return get(borrow(a), index(i));
  }
  void setValueAtIndexImpl(const jsi::Array& a, size_t i, const jsi::Value& value)
      override {
    check(kahvi_set(rt_, borrow(a), index(i), borrow(value)));
  }

  // Functions

  jsi::Function createFunctionFromHostFunction(
      const jsi::PropNameID& name,
      unsigned int paramCount,
      jsi::HostFunctionType func) override {
    std::string n = utf8(name);
    auto* context = new HostFunctionContext{this, std::move(func)};
    return takeAs<jsi::Function>(kahvi_function_new(
        rt_,
        n.data(),
        n.size(),
        paramCount,
        hostFunctionCall,
        context,
        hostFunctionFinalize));
  }

  jsi::Value call(
      const jsi::Function& f,
      const jsi::Value& jsThis,
      const jsi::Value* args,
      size_t count) override {
    std::vector<KahviValue> argv = borrowAll(args, count);
    KahviValue out{};
    check(kahvi_call(rt_, borrow(f), borrow(jsThis), argv.data(), count, &out));
    return take(out);
  }

  jsi::Value callAsConstructor(
      const jsi::Function& f,
      const jsi::Value* args,
      size_t count) override {
    std::vector<KahviValue> argv = borrowAll(args, count);
    KahviValue out{};
    check(kahvi_construct(rt_, borrow(f), argv.data(), count, &out));
    return take(out);
  }

  bool strictEquals(const jsi::Symbol& a, const jsi::Symbol& b) const override {
    return kahvi_strict_equals(rt_, borrow(a), borrow(b));
  }
  bool strictEquals(const jsi::BigInt& a, const jsi::BigInt& b) const override {
    return kahvi_strict_equals(rt_, borrow(a), borrow(b));
  }
  bool strictEquals(const jsi::String& a, const jsi::String& b) const override {
    return kahvi_strict_equals(rt_, borrow(a), borrow(b));
  }
  bool strictEquals(const jsi::Object& a, const jsi::Object& b) const override {
    return kahvi_strict_equals(rt_, borrow(a), borrow(b));
  }

  bool instanceOf(const jsi::Object& o, const jsi::Function& f) override {
    bool result = false;
    check(kahvi_instance_of(rt_, borrow(o), borrow(f), &result));
    return result;
  }

  void setExternalMemoryPressure(const jsi::Object&, size_t) override {
    // A scheduling hint only: kahvi's collector does not account for
    // memory held outside its heap.
  }

 private:
  struct Pointer final : PointerValue {
    explicit Pointer(KahviValue v) : value(v) {}
    void invalidate() noexcept override {
      kahvi_value_release(value);
      delete this;
    }
    KahviValue value;
  };

  struct Prepared final : jsi::PreparedJavaScript {
    Prepared(std::shared_ptr<const jsi::Buffer> b, std::string url)
        : buffer(std::move(b)), sourceURL(std::move(url)) {}
    std::shared_ptr<const jsi::Buffer> buffer;
    std::string sourceURL;
  };

  struct HostFunctionContext {
    KahviRuntimeImpl* runtime;
    jsi::HostFunctionType function;
  };

  struct HostObjectContext {
    KahviRuntimeImpl* runtime;
    std::shared_ptr<jsi::HostObject> hostObject;
  };

  struct NativeStateContext {
    std::shared_ptr<jsi::NativeState> state;
  };

  // Conversions. borrow() returns the handle a jsi value already owns;
  // take() wraps a handle the C ABI handed over; release() hands one back.

  static KahviValue borrow(const jsi::Pointer& p) {
    return static_cast<const Pointer*>(getPointerValue(p))->value;
  }

  static KahviValue borrow(const jsi::Value& v) {
    KahviValue k{};
    if (v.isUndefined()) {
      k.tag = KAHVI_UNDEFINED;
    } else if (v.isNull()) {
      k.tag = KAHVI_NULL;
    } else if (v.isBool()) {
      k.tag = KAHVI_BOOLEAN;
      k.boolean = v.getBool();
    } else if (v.isNumber()) {
      k.tag = KAHVI_NUMBER;
      k.number = v.getNumber();
    } else {
      k = static_cast<const Pointer*>(getPointerValue(v))->value;
    }
    return k;
  }

  static std::vector<KahviValue> borrowAll(const jsi::Value* args, size_t count) {
    std::vector<KahviValue> argv;
    argv.reserve(count);
    for (size_t i = 0; i < count; i++) {
      argv.push_back(borrow(args[i]));
    }
    return argv;
  }

  template <typename T>
  static T takeAs(KahviValue k) {
    return make<T>(new Pointer(k));
  }

  static jsi::Value take(KahviValue k) {
    switch (k.tag) {
      case KAHVI_UNDEFINED:
        return jsi::Value();
      case KAHVI_NULL:
        return jsi::Value(nullptr);
      case KAHVI_BOOLEAN:
        return jsi::Value(k.boolean);
      case KAHVI_NUMBER:
        return jsi::Value(k.number);
      case KAHVI_STRING:
        return jsi::Value(takeAs<jsi::String>(k));
      case KAHVI_SYMBOL:
        return jsi::Value(takeAs<jsi::Symbol>(k));
      case KAHVI_BIGINT:
        return jsi::Value(takeAs<jsi::BigInt>(k));
      case KAHVI_OBJECT:
        return jsi::Value(takeAs<jsi::Object>(k));
    }
    throw jsi::JSINativeException("kahvi returned a value with an unknown tag");
  }

  static KahviValue release(const jsi::Value& v) {
    return kahvi_value_clone(borrow(v));
  }

  static PointerValue* clonePointer(const PointerValue* pv) {
    if (!pv) {
      return nullptr;
    }
    return new Pointer(kahvi_value_clone(static_cast<const Pointer*>(pv)->value));
  }

  static KahviValue index(size_t i) {
    KahviValue k{};
    k.tag = KAHVI_NUMBER;
    k.number = static_cast<double>(i);
    return k;
  }

  std::string stringUtf8(KahviValue v) {
    size_t length = 0;
    char* bytes = kahvi_string_utf8(rt_, v, &length);
    if (!bytes) {
      throw jsi::JSINativeException("Value is not a string");
    }
    std::string s(bytes, length);
    kahvi_utf8_free(bytes, length);
    return s;
  }

  std::string ownedUtf8(KahviValue v) {
    std::string s = stringUtf8(v);
    kahvi_value_release(v);
    return s;
  }

  jsi::Value get(KahviValue object, KahviValue key) {
    KahviValue out{};
    check(kahvi_get(rt_, object, key, &out));
    return take(out);
  }

  bool has(KahviValue object, KahviValue key) {
    bool result = false;
    check(kahvi_has(rt_, object, key, &result));
    return result;
  }

  // A false return from the C ABI means JavaScript threw.
  void check(bool ok) {
    if (!ok) {
      throw jsi::JSError(*this, take(kahvi_take_exception(rt_)));
    }
  }

  // A new Error(message), or whatever constructing it threw.
  KahviValue makeError(const std::string& message) {
    KahviValue global = kahvi_global(rt_);
    KahviValue name = kahvi_string_new(rt_, "Error", 5);
    KahviValue text = kahvi_string_new(rt_, message.data(), message.size());
    KahviValue ctor{};
    KahviValue error{};
    if (!kahvi_get(rt_, global, name, &ctor) ||
        !kahvi_construct(rt_, ctor, &text, 1, &error)) {
      error = kahvi_take_exception(rt_);
    }
    kahvi_value_release(ctor);
    kahvi_value_release(text);
    kahvi_value_release(name);
    kahvi_value_release(global);
    return error;
  }

  // Runs a host callback, writing its result or the exception to *out.
  template <typename F>
  bool guard(KahviValue* out, F&& f) {
    try {
      *out = f();
      return true;
    } catch (const jsi::JSError& e) {
      *out = release(e.value());
    } catch (const std::exception& e) {
      *out = makeError(e.what());
    } catch (...) {
      *out = makeError("Unknown C++ exception in a host callback");
    }
    return false;
  }

  static bool hostFunctionCall(
      KahviRuntime*,
      void* data,
      KahviValue thisValue,
      const KahviValue* args,
      size_t argc,
      KahviValue* result) {
    auto* context = static_cast<HostFunctionContext*>(data);
    KahviRuntimeImpl& self = *context->runtime;
    return self.guard(result, [&] {
      jsi::Value receiver = take(kahvi_value_clone(thisValue));
      std::vector<jsi::Value> argv;
      argv.reserve(argc);
      for (size_t i = 0; i < argc; i++) {
        argv.push_back(take(kahvi_value_clone(args[i])));
      }
      return release(context->function(self, receiver, argv.data(), argc));
    });
  }

  static void hostFunctionFinalize(void* data) {
    delete static_cast<HostFunctionContext*>(data);
  }

  static bool hostObjectGet(
      KahviRuntime*,
      void* data,
      KahviValue name,
      KahviValue* result) {
    auto* context = static_cast<HostObjectContext*>(data);
    KahviRuntimeImpl& self = *context->runtime;
    return self.guard(result, [&] {
      auto prop = takeAs<jsi::PropNameID>(kahvi_value_clone(name));
      return release(context->hostObject->get(self, prop));
    });
  }

  static bool hostObjectSet(
      KahviRuntime*,
      void* data,
      KahviValue name,
      KahviValue value,
      KahviValue* exception) {
    auto* context = static_cast<HostObjectContext*>(data);
    KahviRuntimeImpl& self = *context->runtime;
    return self.guard(exception, [&] {
      auto prop = takeAs<jsi::PropNameID>(kahvi_value_clone(name));
      jsi::Value v = take(kahvi_value_clone(value));
      context->hostObject->set(self, prop, v);
      return KahviValue{};
    });
  }

  static bool hostObjectPropertyNames(
      KahviRuntime* rt,
      void* data,
      KahviValue* result) {
    auto* context = static_cast<HostObjectContext*>(data);
    KahviRuntimeImpl& self = *context->runtime;
    return self.guard(result, [&] {
      std::vector<jsi::PropNameID> names = context->hostObject->getPropertyNames(self);
      jsi::Array array = self.createArray(names.size());
      for (size_t i = 0; i < names.size(); i++) {
        self.check(kahvi_set(rt, borrow(array), index(i), borrow(names[i])));
      }
      return release(jsi::Value(std::move(array)));
    });
  }

  static void hostObjectFinalize(void* data) {
    delete static_cast<HostObjectContext*>(data);
  }

  static void nativeStateFinalize(void* data) {
    delete static_cast<NativeStateContext*>(data);
  }

  KahviRuntime* const rt_;
};

} // namespace

std::unique_ptr<jsi::Runtime> makeKahviRuntime() {
  return std::make_unique<KahviRuntimeImpl>();
}

} // namespace kahvi
