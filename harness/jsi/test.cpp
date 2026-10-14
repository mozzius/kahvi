// The JSI shim end to end: what React Native asks of a jsi::Runtime.
// test262 cannot reach this layer (it is C++ over the C ABI).
#include <cstdio>
#include <cstdlib>
#include <stdexcept>
#include <string>

#include <jsi/jsi.h>

#include "KahviJSI.h"

using namespace facebook::jsi;

static int failures = 0;

#define EXPECT(cond)                                                   \
  do {                                                                 \
    if (!(cond)) {                                                     \
      std::fprintf(stderr, "FAIL %s:%d: %s\n", __FILE__, __LINE__, #cond); \
      failures++;                                                      \
    }                                                                  \
  } while (0)

static Value eval(Runtime& rt, const std::string& src) {
  return rt.evaluateJavaScript(std::make_shared<StringBuffer>(src), "test.js");
}

struct Counter : HostObject {
  double count = 0;
  Value get(Runtime& rt, const PropNameID& name) override {
    std::string n = name.utf8(rt);
    if (n == "count") return Value(count);
    if (n == "boom") throw JSINativeException("host get failed");
    return Value::undefined();
  }
  void set(Runtime& rt, const PropNameID& name, const Value& v) override {
    if (name.utf8(rt) == "count") count = v.asNumber();
  }
  std::vector<PropNameID> getPropertyNames(Runtime& rt) override {
    std::vector<PropNameID> names;
    names.push_back(PropNameID::forAscii(rt, "count"));
    return names;
  }
};

struct Tag : NativeState {
  explicit Tag(int v) : value(v) {}
  int value;
};

int main() {
  {
    auto runtime = kahvi::makeKahviRuntime();
    Runtime& rt = *runtime;

    // Evaluation and primitive values.
    EXPECT(eval(rt, "1 + 2").getNumber() == 3);
    EXPECT(eval(rt, "'k' + 'ahvi'").getString(rt).utf8(rt) == "kahvi");
    EXPECT(eval(rt, "null").isNull());
    EXPECT(eval(rt, "undefined").isUndefined());
    EXPECT(eval(rt, "1 < 2").getBool());
    EXPECT(String::createFromUtf8(rt, "h\xc3\xa9").utf8(rt) == "h\xc3\xa9");

    // Objects and properties.
    Object o(rt);
    o.setProperty(rt, "a", 1);
    o.setProperty(rt, "b", "two");
    EXPECT(o.getProperty(rt, "a").getNumber() == 1);
    EXPECT(o.hasProperty(rt, "b"));
    EXPECT(!o.hasProperty(rt, "c"));
    EXPECT(o.getPropertyNames(rt).size(rt) == 2);
    rt.global().setProperty(rt, "o", o);
    EXPECT(eval(rt, "o.a + o.b").getString(rt).utf8(rt) == "1two");
    EXPECT(Value::strictEquals(rt, rt.global().getProperty(rt, "o"), Value(rt, o)));

    // Arrays.
    Array arr = Array::createWithElements(rt, 1, "x", true);
    EXPECT(arr.size(rt) == 3);
    EXPECT(arr.getValueAtIndex(rt, 1).getString(rt).utf8(rt) == "x");
    EXPECT(eval(rt, "[1,2,3]").getObject(rt).isArray(rt));
    EXPECT(eval(rt, "[1,2,3]").getObject(rt).getArray(rt).size(rt) == 3);

    // Functions both ways.
    Function add = eval(rt, "(function (a, b) { return a + b; })").getObject(rt).getFunction(rt);
    EXPECT(add.call(rt, 2, 3).getNumber() == 5);
    Function point = eval(rt, "(class P { constructor(x) { this.x = x; } })").getObject(rt).getFunction(rt);
    Object p = point.callAsConstructor(rt, 7).getObject(rt);
    EXPECT(p.getProperty(rt, "x").getNumber() == 7);
    EXPECT(p.instanceOf(rt, point));

    int calls = 0;
    Function twice = Function::createFromHostFunction(
        rt, PropNameID::forAscii(rt, "twice"), 1,
        [&calls](Runtime& rt, const Value&, const Value* args, size_t n) -> Value {
          calls++;
          if (n < 1) throw JSError(rt, "twice needs an argument");
          return Value(args[0].asNumber() * 2);
        });
    rt.global().setProperty(rt, "twice", twice);
    EXPECT(eval(rt, "twice(21)").getNumber() == 42);
    EXPECT(eval(rt, "twice.name").getString(rt).utf8(rt) == "twice");
    EXPECT(eval(rt, "twice.length").getNumber() == 1);
    EXPECT(twice.isHostFunction(rt));
    EXPECT(!add.isHostFunction(rt));
    EXPECT(eval(rt, "try { twice() } catch (e) { e.message }").getString(rt).utf8(rt) ==
           "twice needs an argument");
    EXPECT(calls == 2);
    Function nativeThrow = Function::createFromHostFunction(
        rt, PropNameID::forAscii(rt, "f"), 0,
        [](Runtime&, const Value&, const Value*, size_t) -> Value {
          throw std::runtime_error("native");
          return Value::undefined();
        });
    rt.global().setProperty(rt, "nativeThrow", nativeThrow);
    EXPECT(eval(rt, "try { nativeThrow() } catch (e) { e instanceof Error && e.message }")
               .getString(rt).utf8(rt) == "native");

    // Exceptions from JS reach C++ as JSError.
    bool threw = false;
    try {
      eval(rt, "throw new TypeError('bad')");
    } catch (const JSError& e) {
      threw = true;
      EXPECT(e.getMessage() == "bad");
    }
    EXPECT(threw);
    threw = false;
    try {
      eval(rt, "(");
    } catch (const JSError&) {
      threw = true;
    }
    EXPECT(threw);

    // Host objects.
    auto counter = std::make_shared<Counter>();
    Object ho = Object::createFromHostObject(rt, counter);
    rt.global().setProperty(rt, "counter", ho);
    eval(rt, "counter.count = 5; counter.count += 1");
    EXPECT(counter->count == 6);
    EXPECT(eval(rt, "Object.keys(counter).join()").getString(rt).utf8(rt) == "count");
    EXPECT(eval(rt, "try { counter.boom } catch (e) { e.message }").getString(rt).utf8(rt) ==
           "host get failed");
    EXPECT(ho.isHostObject(rt));
    EXPECT(ho.getHostObject<Counter>(rt) == counter);

    // Native state.
    Object holder(rt);
    EXPECT(!holder.hasNativeState(rt));
    holder.setNativeState(rt, std::make_shared<Tag>(9));
    EXPECT(holder.hasNativeState(rt));
    EXPECT(holder.getNativeState<Tag>(rt)->value == 9);

    // Symbols and BigInts.
    Symbol sym = eval(rt, "Symbol('s')").getSymbol(rt);
    EXPECT(sym.toString(rt) == "Symbol(s)");
    BigInt big = BigInt::fromInt64(rt, -5);
    EXPECT(big.isInt64(rt));
    EXPECT(!big.isUint64(rt));
    EXPECT(big.toString(rt, 10).utf8(rt) == "-5");
    EXPECT(eval(rt, "2n ** 64n").getBigInt(rt).getUint64(rt) == 0);
    EXPECT(BigInt::fromUint64(rt, 255).toString(rt, 16).utf8(rt) == "ff");

    // Microtasks and promises.
    eval(rt, "globalThis.log = []; Promise.resolve().then(() => log.push('then'))");
    Function task = Function::createFromHostFunction(
        rt, PropNameID::forAscii(rt, "task"), 0,
        [](Runtime& rt, const Value&, const Value*, size_t) -> Value {
          eval(rt, "log.push('task')");
          return Value::undefined();
        });
    rt.queueMicrotask(task);
    EXPECT(rt.drainMicrotasks());
    EXPECT(eval(rt, "log.join()").getString(rt).utf8(rt) == "then,task");

    // JSON and weak objects.
    Value parsed = Value::createFromJsonUtf8(rt, reinterpret_cast<const uint8_t*>("{\"n\":[1]}"), 9);
    EXPECT(parsed.getObject(rt).getProperty(rt, "n").getObject(rt).isArray(rt));
    WeakObject weak(rt, o);
    EXPECT(weak.lock(rt).isObject());

    // ArrayBuffers created by JS.
    Object ab = eval(rt, "new Uint8Array([1, 2, 3]).buffer").getObject(rt);
    EXPECT(ab.isArrayBuffer(rt));
    EXPECT(ab.getArrayBuffer(rt).size(rt) == 3);
    EXPECT(ab.getArrayBuffer(rt).data(rt)[2] == 3);

    // Prepared scripts.
    auto prepared = rt.prepareJavaScript(std::make_shared<StringBuffer>("6 * 7"), "p.js");
    EXPECT(rt.evaluatePreparedJavaScript(prepared).getNumber() == 42);
  }
  if (failures) {
    std::fprintf(stderr, "%d failures\n", failures);
    return 1;
  }
  std::printf("jsi: all checks passed\n");
  return 0;
}
