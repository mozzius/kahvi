# Ticket B-object-prototype-define-getter: Object.prototype.__defineGetter__

Implement this built-in in its file, following the spec steps already in
the stub: keep each step as a comment directly above the code for it.

- Abstract operations are in `kahvi_core::ops` (see docs/API.md); use them
  rather than re-implementing. If a helper you need is missing and other
  built-ins would need it too, write it in your file and say so in your
  final report; do not edit other files.
- Anything you cannot finish must still return
  `Err(agent.not_implemented("..."))` naming what is missing. Never return
  a made-up value.
- No new Rust tests: the test262 target is the measure.

## Files you may change

- `crates/kahvi-builtins/src/object/prototype_define_getter.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Object.prototype.__defineGetter__ ( _key_, _getter_ )
  id: sec-object.prototype.__defineGetter__   type: built-in function

This method performs the following steps when called:

1. Let _obj_ be ? ToObject(*this* value).
2. If IsCallable(_getter_) is *false*, throw a *TypeError* exception.
3. Let _propertyDesc_ be PropertyDescriptor { [[Getter]]: _getter_, [[Enumerable]]: *true*, [[Configurable]]: *true* }.
4. Let _propertyKey_ be ? ToPropertyKey(_key_).
5. Perform ? DefinePropertyOrThrow(_obj_, _propertyKey_, _propertyDesc_).
6. Return *undefined*.

--- referenced: ToObject ( _arg_: an ECMAScript language value, ): either a normal completion containing an Object or a throw completion
  id: sec-toobject

description: It converts _arg_ to a value of type Object.

1. If _arg_ is either *undefined* or *null*, throw a *TypeError* exception.
2. If _arg_ is a Boolean, return a new Boolean object whose [[BooleanData]] internal slot is set to _arg_. See [sec-boolean-objects] for a description of Boolean objects.
3. If _arg_ is a Number, return a new Number object whose [[NumberData]] internal slot is set to _arg_. See [sec-number-objects] for a description of Number objects.
4. If _arg_ is a String, return a new String object whose [[StringData]] internal slot is set to _arg_. See [sec-string-objects] for a description of String objects.
5. If _arg_ is a Symbol, return a new Symbol object whose [[SymbolData]] internal slot is set to _arg_. See [sec-symbol-objects] for a description of Symbol objects.
6. If _arg_ is a BigInt, return a new BigInt object whose [[BigIntData]] internal slot is set to _arg_. See [sec-bigint-objects] for a description of BigInt objects.
7. Assert: _arg_ is an Object.
8. Return _arg_.

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

--- referenced: ToPropertyKey ( _arg_: an ECMAScript language value, ): either a normal completion containing a property key or a throw completion
  id: sec-topropertykey

description: It converts _arg_ to a value that can be used as a property key.

1. Let _key_ be ? ToPrimitive(_arg_, ~string~).
2. If _key_ is a Symbol, then
   a. Return _key_.
3. Return ! ToString(_key_).

--- referenced: DefinePropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, _propertyDesc_: a Property Descriptor, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-definepropertyorthrow

description: It is used to call the [[DefineOwnProperty]] internal method of an object in a manner that will throw a *TypeError* exception if the requested property update cannot be performed.

1. Let _success_ be ? _obj_.[[DefineOwnProperty]](_propertyKey_, _propertyDesc_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.


## Done when

- Every test262 run under built-ins/Object/prototype/__defineGetter__/ passes (check with `./t262 built-ins/Object/prototype/__defineGetter__/`). 6 pass on the main branch right now.

