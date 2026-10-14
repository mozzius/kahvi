# Ticket B-disposable-stack-prototype-move: DisposableStack.prototype.move

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

- `crates/kahvi-builtins/src/disposable_stack/prototype_move.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/disposable_stack.rs`
- `crates/kahvi-core/src/ops/dispose.rs`
- `crates/kahvi-builtins/src/disposable_stack/prototype_use.rs`

## Spec

DisposableStack.prototype.move ( )
  id: sec-disposablestack.prototype.move   type: built-in function

This method performs the following steps when called:

1. Let _disposableStack_ be the *this* value.
2. Perform ? RequireInternalSlot(_disposableStack_, [[DisposableState]]).
3. If _disposableStack_.[[DisposableState]] is ~disposed~, throw a *ReferenceError* exception.
4. Let _newDisposableStack_ be ? OrdinaryCreateFromConstructor(%DisposableStack%, *"%DisposableStack.prototype%"*, « [[DisposableState]], [[DisposableResourceStack]] »).
5. Set _newDisposableStack_.[[DisposableState]] to ~pending~.
6. Set _newDisposableStack_.[[DisposableResourceStack]] to _disposableStack_.[[DisposableResourceStack]].
7. Set _disposableStack_.[[DisposableResourceStack]] to a new empty List.
8. Set _disposableStack_.[[DisposableState]] to ~disposed~.
9. Return _newDisposableStack_.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: OrdinaryCreateFromConstructor ( _ctor_: a function object, _intrinsicDefaultProto_: a String, optional _internalSlotsList_: a List of names of internal slots, ): either a normal completion containing an Object or a throw completion
  id: sec-ordinarycreatefromconstructor

description: It creates an ordinary object whose [[Prototype]] value is retrieved from a constructor's *"prototype"* property, if it exists. Otherwise the intrinsic named by _intrinsicDefaultProto_ is used for [[Prototype]]. _internalSlotsList_ contains the names of additional internal slots that must be defined as part of the object. If _internalSlotsList_ is not provided, a new empty List is used.

1. Assert: _intrinsicDefaultProto_ is this specification's name of an intrinsic object. The corresponding object must be an intrinsic that is intended to be used as the [[Prototype]] value of an object.
2. Let _proto_ be ? GetPrototypeFromConstructor(_ctor_, _intrinsicDefaultProto_).
3. If _internalSlotsList_ is present, let _slots_ be _internalSlotsList_.
4. Else, let _slots_ be a new empty List.
5. Return OrdinaryObjectCreate(_proto_, _slots_).


## Done when

- Every test262 run under built-ins/DisposableStack/prototype/move/ passes (check with `./t262 built-ins/DisposableStack/prototype/move/`). 8 pass on the main branch right now.

