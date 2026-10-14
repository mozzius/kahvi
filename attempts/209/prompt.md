# Ticket B-set-prototype-is-subset-of: Set.prototype.isSubsetOf

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

- `crates/kahvi-builtins/src/set/prototype_is_subset_of.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`
- `crates/kahvi-builtins/src/support/collections.rs`
- `crates/kahvi-builtins/src/map/prototype_set.rs`
- `crates/kahvi-builtins/src/map_iterator_prototype/next.rs`

## Spec

Set.prototype.isSubsetOf ( _other_ )
  id: sec-set.prototype.issubsetof   type: built-in function

This method performs the following steps when called:

1. Let _set_ be the *this* value.
2. Perform ? RequireInternalSlot(_set_, [[SetData]]).
3. Let _otherRecord_ be ? GetSetRecord(_other_).
4. If SetDataSize(_set_.[[SetData]]) > _otherRecord_.[[Size]], return *false*.
5. Let _thisSize_ be the number of elements in _set_.[[SetData]].
6. Let _index_ be 0.
7. Repeat, while _index_ < _thisSize_,
   a. Let _entry_ be _set_.[[SetData]][_index_].
   b. Set _index_ to _index_ + 1.
   c. If _entry_ is not ~empty~, then
      i. Let _inOther_ be ToBoolean(? Call(_otherRecord_.[[Has]], _otherRecord_.[[SetObject]], « _entry_ »)).
      ii. If _inOther_ is *false*, return *false*.
      iii. NOTE: The number of elements in _set_.[[SetData]] may have increased during execution of _otherRecord_.[[Has]].
      iv. Set _thisSize_ to the number of elements in _set_.[[SetData]].
8. Return *true*.

--- referenced: RequireInternalSlot ( _obj_: an ECMAScript language value, _internalSlot_: an internal slot name, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-requireinternalslot

description: It throws an exception unless _obj_ is an Object and has the given internal slot.

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. If _obj_ does not have an _internalSlot_ internal slot, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: GetSetRecord ( _obj_: an ECMAScript language value, ): either a normal completion containing a Set Record or a throw completion
  id: sec-getsetrecord

1. If _obj_ is not an Object, throw a *TypeError* exception.
2. Let _rawSize_ be ? Get(_obj_, *"size"*).
3. Let _numberSize_ be ? ToNumber(_rawSize_).
4. NOTE: If _rawSize_ is *undefined*, then _numberSize_ will be *NaN*.
5. If _numberSize_ is *NaN*, throw a *TypeError* exception.
6. Let _intSize_ be ! ToIntegerOrInfinity(_numberSize_).
7. If _intSize_ < 0, throw a *RangeError* exception.
8. Let _has_ be ? Get(_obj_, *"has"*).
9. If IsCallable(_has_) is *false*, throw a *TypeError* exception.
10. Let _keys_ be ? Get(_obj_, *"keys"*).
11. If IsCallable(_keys_) is *false*, throw a *TypeError* exception.
12. Return a new Set Record { [[SetObject]]: _obj_, [[Size]]: _intSize_, [[Has]]: _has_, [[Keys]]: _keys_ }.

--- referenced: SetDataSize ( _setData_: a List of either ECMAScript language values or ~empty~, ): a non-negative integer
  id: sec-setdatasize

1. Let _count_ be 0.
2. For each element _element_ of _setData_, do
   a. If _element_ is not ~empty~, set _count_ to _count_ + 1.
3. Return _count_.

--- referenced: ToBoolean ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-toboolean

description: It converts _arg_ to a value of type Boolean.

1. If _arg_ is a Boolean, return _arg_.
2. If _arg_ is one of *undefined*, *null*, *+0*_𝔽, *-0*_𝔽, *NaN*, *0*_ℤ, or the empty String, return *false*.
3. If the host is a web browser or otherwise supports [sec-IsHTMLDDA-internal-slot], then
   a. If _arg_ is an Object and _arg_ has an [[IsHTMLDDA]] internal slot, return *false*.
4. Return *true*.

--- referenced: Call ( _func_: an ECMAScript language value, _thisValue_: an ECMAScript language value, optional _argList_: a List of ECMAScript language values, ): either a normal completion containing an ECMAScript language value or a throw completion
  id: sec-call

description: It is used to call the [[Call]] internal method of a function object. _func_ is the function object, _thisValue_ is an ECMAScript language value that is the *this* value of the [[Call]], and _argList_ is the value passed to the corresponding argument of the internal method. If _argList_ is not present, a new empty List is used as its value.

1. If _argList_ is not present, set _argList_ to a new empty List.
2. If IsCallable(_func_) is *false*, throw a *TypeError* exception.
3. Return ? _func_.[[Call]](_thisValue_, _argList_).


## Done when

- Every test262 run under built-ins/Set/prototype/isSubsetOf/ passes (check with `./t262 built-ins/Set/prototype/isSubsetOf/`). 10 pass on the main branch right now.

