/* kahvi's C ABI. See crates/kahvi-ffi/src/lib.rs.
 *
 * Ownership: every KahviValue the API returns (directly or through an out
 * parameter) is owned by the caller, who releases it with
 * kahvi_value_release. Values passed in are borrowed. Strings, symbols,
 * bigints and objects carry a handle that keeps them alive until released.
 *
 * Errors: functions returning bool return false when JavaScript threw; the
 * exception is then taken with kahvi_take_exception.
 *
 * Threads: a runtime may move between threads but is used by one at a time.
 * Host callbacks run on the calling thread and may call back into the API
 * with the KahviRuntime* they are given.
 */
#ifndef KAHVI_H
#define KAHVI_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct KahviRuntime KahviRuntime;
typedef struct KahviHandle KahviHandle;

typedef enum {
  KAHVI_UNDEFINED = 0,
  KAHVI_NULL = 1,
  KAHVI_BOOLEAN = 2,
  KAHVI_NUMBER = 3,
  KAHVI_STRING = 4,
  KAHVI_SYMBOL = 5,
  KAHVI_BIGINT = 6,
  KAHVI_OBJECT = 7,
} KahviTag;

typedef struct {
  KahviTag tag;
  bool boolean;
  double number;
  KahviHandle *handle; /* strings, symbols, bigints and objects */
} KahviValue;

/* A host function: write the result (or, returning false, the exception)
 * to *result. */
typedef bool (*KahviHostFunction)(KahviRuntime *rt, void *context, KahviValue this_value,
                                  const KahviValue *args, size_t argc, KahviValue *result);
typedef void (*KahviFinalizer)(void *context);

/* A host object's properties (JSI's HostObject). get and property_names
 * write their result to *result (or the exception, returning false);
 * property_names returns an array of strings or symbols. finalize runs when
 * the object is collected and must not call into the runtime. */
typedef struct {
  bool (*get)(KahviRuntime *rt, void *context, KahviValue name, KahviValue *result);
  bool (*set)(KahviRuntime *rt, void *context, KahviValue name, KahviValue value,
              KahviValue *exception);
  bool (*property_names)(KahviRuntime *rt, void *context, KahviValue *result);
  KahviFinalizer finalize;
} KahviHostObjectClass;

/* Runtime */
KahviRuntime *kahvi_runtime_new(void);
void kahvi_runtime_free(KahviRuntime *rt);
bool kahvi_eval(KahviRuntime *rt, const char *source, size_t len, const char *name, size_t name_len,
                KahviValue *result);
bool kahvi_run_jobs(KahviRuntime *rt);
void kahvi_queue_microtask(KahviRuntime *rt, KahviValue function);
KahviValue kahvi_take_exception(KahviRuntime *rt);
void kahvi_collect_garbage(KahviRuntime *rt);
KahviValue kahvi_global(KahviRuntime *rt);

/* Values */
void kahvi_value_release(KahviValue v);
KahviValue kahvi_value_clone(KahviValue v);
bool kahvi_strict_equals(KahviRuntime *rt, KahviValue a, KahviValue b);
KahviValue kahvi_string_new(KahviRuntime *rt, const char *utf8, size_t len);
/* NULL unless v is a string; free with kahvi_utf8_free. */
char *kahvi_string_utf8(KahviRuntime *rt, KahviValue v, size_t *len);
void kahvi_utf8_free(char *utf8, size_t len);
bool kahvi_to_string(KahviRuntime *rt, KahviValue v, KahviValue *result);
KahviValue kahvi_symbol_to_string(KahviRuntime *rt, KahviValue symbol);
KahviValue kahvi_bigint_from_i64(KahviRuntime *rt, int64_t v);
KahviValue kahvi_bigint_from_u64(KahviRuntime *rt, uint64_t v);
uint64_t kahvi_bigint_truncate(KahviRuntime *rt, KahviValue v, bool *fits_i64, bool *fits_u64);
KahviValue kahvi_bigint_to_string(KahviRuntime *rt, KahviValue v, uint32_t radix);

/* Objects */
KahviValue kahvi_object_new(KahviRuntime *rt);
bool kahvi_array_new(KahviRuntime *rt, size_t length, KahviValue *result);
bool kahvi_get(KahviRuntime *rt, KahviValue object, KahviValue key, KahviValue *result);
bool kahvi_set(KahviRuntime *rt, KahviValue object, KahviValue key, KahviValue value);
bool kahvi_has(KahviRuntime *rt, KahviValue object, KahviValue key, bool *result);
bool kahvi_property_names(KahviRuntime *rt, KahviValue object, KahviValue *result);
bool kahvi_call(KahviRuntime *rt, KahviValue function, KahviValue this_value,
                const KahviValue *args, size_t argc, KahviValue *result);
bool kahvi_construct(KahviRuntime *rt, KahviValue function, const KahviValue *args, size_t argc,
                     KahviValue *result);
bool kahvi_instance_of(KahviRuntime *rt, KahviValue v, KahviValue constructor, bool *result);
bool kahvi_is_array(KahviRuntime *rt, KahviValue v);
bool kahvi_is_function(KahviRuntime *rt, KahviValue v);
bool kahvi_is_array_buffer(KahviRuntime *rt, KahviValue v);
bool kahvi_length(KahviRuntime *rt, KahviValue v, size_t *result);
bool kahvi_array_buffer_new(KahviRuntime *rt, size_t length, KahviValue *result);
uint8_t *kahvi_array_buffer_data(KahviRuntime *rt, KahviValue buffer, size_t *length);
bool kahvi_weak_new(KahviRuntime *rt, KahviValue object, KahviValue *result);
bool kahvi_weak_lock(KahviRuntime *rt, KahviValue weak, KahviValue *result);

/* Host functions and host objects */
KahviValue kahvi_function_new(KahviRuntime *rt, const char *name, size_t name_len, uint32_t length,
                              KahviHostFunction function, void *context, KahviFinalizer finalize);
void *kahvi_host_function_context(KahviRuntime *rt, KahviValue function);
KahviValue kahvi_host_object_new(KahviRuntime *rt, const KahviHostObjectClass *cls, void *context);
void *kahvi_host_object_context(KahviRuntime *rt, KahviValue object);
/* JSI's NativeState: finalize(context) runs when replaced or collected. */
bool kahvi_set_native_state(KahviRuntime *rt, KahviValue object, void *context,
                            KahviFinalizer finalize);
void *kahvi_get_native_state(KahviRuntime *rt, KahviValue object);

#ifdef __cplusplus
}
#endif

#endif
