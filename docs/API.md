# Writing a built-in

Every built-in function has its own file in `crates/kahvi-builtins/src/`,
e.g. `array/prototype_map.rs` is `Array.prototype.map`. The file starts as a
stub with the spec's algorithm as comments. A ticket asks you to implement
one stub; only that file may change.

Read a finished one first: `array/prototype_map.rs`, `array/prototype_join.rs`,
`object/define_property.rs`, `function/prototype_bind.rs`.

## The shape

```rust
use kahvi_core::prelude::*;

/// Array.prototype.map
pub(crate) fn prototype_map(agent: &mut Agent, this: Value, args: &[Value], new_target: Option<Object>) -> JsResult<Value> {
    // 1. Let O be ? ToObject(this value).
    let o = ops::to_object(agent, this)?;
    ...
}
```

- `this` is the this value; `args.get_or_undefined(i)` is the i-th argument
  (`undefined` when absent); `args.len()` is the argument count.
- `new_target` is `None` for a call and `Some(constructor)` for `new`.
- Every spec step stays a comment, directly above the code for it.
- `?` in the spec is `?` in Rust. `!` (can't fail) is still `?` (never `unwrap`).
- Rename unused parameters with a leading `_` (`_new_target`).

## Values

| Spec | Rust |
|---|---|
| undefined, null, true | `Value::Undefined`, `Value::Null`, `Value::Boolean(true)` |
| a Number | `Value::Number(f64)` |
| a String | `Value::String(JsString)` (a handle; contents are UTF-16) |
| a Symbol, a BigInt | `Value::Symbol(Symbol)`, `Value::BigInt(BigInt)` |
| an Object | `Value::Object(Object)` (a handle) |

- `agent.string("abc")` makes a String; `agent.str(s)` is its `&[u16]`;
  `agent.str_len(s)`; `agent.concat(a, b)`; `agent.substring(s, from, to)`;
  `agent.intern(&units)` makes a String from a `Vec<u16>` you built.
- Integers in spec loops (`k`, `len`) are `f64`, since they go up to 2^53 - 1.
- Strings, symbols and objects compare by handle: `a == b` is SameValue for
  them. For values in general use `ops::same_value` and friends.

## Property keys

- `agent.key("length")` for a fixed name; `PropertyKey::String(agent.names.length)`
  for the common names in `agent.names`.
- `agent.key_from_index(k)` is `! ToString(𝔽(k))` in a spec loop.
- `PropertyKey::Symbol(agent.symbols_wk.iterator)` is `%Symbol.iterator%`.
- `ops::to_property_key(agent, v)?` converts any value.
- `agent.key_to_value(key)` turns a key back into a String or Symbol value.

## Errors

- `return Err(agent.throw_type_error("message"));` throws a TypeError (also
  `throw_range_error`, `throw_reference_error`, `throw_syntax_error`).
- Anything not implemented: `return Err(agent.not_implemented("what is missing"));`.
  JavaScript cannot catch it, and the runner counts it. Never return a
  made-up value instead.
- Something only an engine bug could cause (your own internal record is
  missing, a closure has no slots): `unreachable!("what broke")` or
  `.expect("what broke")`. That is not "not implemented".

## Objects

- `agent.intrinsic("%Array.prototype%")` is an intrinsic of the current realm.
- `agent.ordinary_object_create(Some(proto))` is OrdinaryObjectCreate(proto).
- Internal methods: `o.internal_get(agent, key, receiver)`,
  `internal_set`, `internal_get_own_property`, `internal_define_own_property`,
  `internal_has_property`, `internal_delete`, `internal_own_property_keys`,
  `internal_get_prototype_of`, `internal_set_prototype_of`,
  `internal_is_extensible`, `internal_prevent_extensions`.
- `o.is_callable(agent)`, `o.is_constructor(agent)`.
- `PropertyDescriptor::data(value, writable, enumerable, configurable)` and
  `PropertyDescriptor::accessor(get, set, enumerable, configurable)`; for a
  partial descriptor set only the fields you need (all are `Option`).
- Objects with internal slots: `ObjectKind` (in `kahvi_core::object`) has
  the wrapper kinds (`BooleanObject`, `NumberObject`, `StringObject`, …);
  built-ins with their own slots store a struct in `ObjectKind::Native` and
  read it with `agent.native::<T>(o)` / `agent.native_mut::<T>(o)`.

## Abstract operations (`kahvi_core::ops`)

Named as in the spec, in snake_case. Most take `agent` first.

- Conversion: `to_primitive`, `to_boolean`, `to_number`, `to_numeric`,
  `to_integer_or_infinity`, `to_int32`, `to_uint32`, `to_uint16`, `to_string`,
  `to_object`, `to_property_key`, `to_length`, `to_index`,
  `canonical_numeric_index_string`, `string_to_number`, `number_to_js_string`.
- Testing: `require_object_coercible`, `is_array`, `is_callable`,
  `is_constructor`, `is_extensible`, `is_integral_number`, `is_regexp`,
  `same_value`, `same_value_zero`, `is_strictly_equal`, `is_loosely_equal`,
  `is_less_than`.
- Objects: `get`, `get_v`, `get_method`, `set`, `create_data_property`,
  `create_data_property_or_throw`, `create_non_enumerable_data_property_or_throw`,
  `define_property_or_throw`, `delete_property_or_throw`, `has_property`,
  `has_own_property`, `call`, `construct`, `invoke`, `set_integrity_level`,
  `test_integrity_level`, `create_array_from_list`, `length_of_array_like`,
  `create_list_from_array_like`, `ordinary_has_instance`,
  `species_constructor`, `array_species_create`, `enumerable_own_properties`,
  `get_prototype_from_constructor`, `ordinary_create_from_constructor`,
  `from_property_descriptor`, `to_property_descriptor`,
  `complete_property_descriptor`, `copy_data_properties`.
- Iteration: `get_iterator`, `get_iterator_from_method`, `iterator_next`,
  `iterator_step`, `iterator_step_value`, `iterator_complete`,
  `iterator_value`, `iterator_close`, `iterator_to_list`,
  `create_iter_result_object`.
- Arrays: `kahvi_core::object::array::array_create(agent, length, proto)`.
- Functions (`kahvi_core::function`): `create_builtin_function`,
  `set_function_name`, `set_function_length`, `bound_function_create`.

Helpers shared between built-ins live in `crates/kahvi-builtins/src/support/`
(`this_value.rs` has ThisNumberValue and friends, `error.rs` the error
constructors, `array_iterator.rs` CreateArrayIterator, `regexp.rs` the RegExp
internal slots (`RegExpData`, `require_regexp`) and RegExpCreate, RegExpExec,
RegExpBuiltinExec and AdvanceStringIndex, `typed_array.rs` ValidateTypedArray,
TypedArraySpeciesCreate, TypedArrayCreateSameType, AllocateArrayBuffer,
GetViewValue/SetViewValue and friends). TypedArray and ArrayBuffer internal
slots and buffer operations (GetValueFromBuffer, IsValidIntegerIndex,
TypedArrayLength, DetachArrayBuffer, ...) are in
`kahvi_core::object::typed_array`. `iterator_helper.rs` is CreateIteratorFromClosure for the
Iterator helpers: implement `HelperClosure` (see Iterator.prototype.map).

## Checking your work

- `cargo build -p kahvi-cli` builds the engine at `target/debug/kahvi`.
- `./t262 built-ins/Array/prototype/map/` runs the ticket's tests.
- `./spec sec-array.prototype.map` prints the algorithm; `./spec --find map`
  searches clause titles.
