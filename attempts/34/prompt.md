# Ticket B-array-prototype-sort: Array.prototype.sort

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

- `crates/kahvi-builtins/src/array/prototype_sort.rs`

## Read first

- `docs/API.md`
- `crates/kahvi-builtins/src/support/mod.rs`
- `crates/kahvi-builtins/src/array/prototype_map.rs`
- `crates/kahvi-builtins/src/object/define_property.rs`

## Spec

Array.prototype.sort ( _comparator_ )
  id: sec-array.prototype.sort   type: built-in function

This method sorts the elements of this array. If _comparator_ is not *undefined*, it should be a function that accepts two arguments _x_ and _y_ and returns a negative Number if _x_ < _y_, a positive Number if _x_ > _y_, or a zero otherwise.
It performs the following steps when called:

1. If _comparator_ is not *undefined* and IsCallable(_comparator_) is *false*, throw a *TypeError* exception.
2. Let _obj_ be ? ToObject(*this* value).
3. Let _length_ be ? LengthOfArrayLike(_obj_).
4. Let _sortCompare_ be a new Abstract Closure with parameters (_x_, _y_) that captures _comparator_ and performs the following steps when called:
   a. Return ? CompareArrayElements(_x_, _y_, _comparator_).
5. Let _sortedList_ be ? SortIndexedProperties(_obj_, _length_, _sortCompare_, ~skip-holes~).
6. Let _itemCount_ be the number of elements in _sortedList_.
7. Let _j_ be 0.
8. Repeat, while _j_ < _itemCount_,
   a. Perform ? Set(_obj_, ! ToString(𝔽(_j_)), _sortedList_[_j_], *true*).
   b. Set _j_ to _j_ + 1.
9. NOTE: The call to SortIndexedProperties in step [step-array-sortindexedproperties] uses ~skip-holes~. The remaining indices are deleted to preserve the number of holes that were detected and excluded from the sort.
10. Repeat, while _j_ < _length_,
   a. Perform ? DeletePropertyOrThrow(_obj_, ! ToString(𝔽(_j_))).
   b. Set _j_ to _j_ + 1.
11. Return _obj_.

NOTE:
Because non-existent property values always compare greater than *undefined* property values, and *undefined* always compares greater than any other value (see CompareArrayElements), *undefined* property values always sort to the end of the result, followed by non-existent property values.
NOTE:
Method calls performed by the ToString abstract operations in steps [step-sortcompare-tostring-x] and [step-sortcompare-tostring-y] have the potential to cause _sortCompare_ to not behave as a consistent comparator.
NOTE:
This method is intentionally generic; it does not require that its *this* value be an Array. Therefore, it can be transferred to other kinds of objects for use as a method.
[subclause sec-sortindexedproperties: SortIndexedProperties (
_obj_: an Object,
_length_: a non-negative integer,
_sortCompare_: an Abstract Closure with two parameters,
_holes_: ~skip-holes~ or ~read-through-holes~,
): either a normal completion containing a List of ECMAScript language values or a throw completion]
[subclause sec-comparearrayelements: CompareArrayElements (
_x_: an ECMAScript language value,
_y_: an ECMAScript language value,
_comparator_: a function object or *undefined*,
): either a normal completion containing a Number or an abrupt completion]

--- referenced: IsCallable ( _arg_: an ECMAScript language value, ): a Boolean
  id: sec-iscallable

description: It determines if _arg_ is a callable function with a [[Call]] internal method.

1. If _arg_ is not an Object, return *false*.
2. If _arg_ has a [[Call]] internal method, return *true*.
3. Return *false*.

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

--- referenced: LengthOfArrayLike ( _obj_: an Object, ): either a normal completion containing a non-negative integer or a throw completion
  id: sec-lengthofarraylike

description: It returns the value of the *"length"* property of an array-like object.

1. Return ℝ(? ToLength(? Get(_obj_, *"length"*))).

An array-like object is any object for which this operation returns a normal completion.
NOTE:
Typically, an array-like object would also have some properties with integer index names. However, that is not a requirement of this definition.
NOTE:
Arrays and String objects are examples of array-like objects.

--- referenced: CompareArrayElements ( _x_: an ECMAScript language value, _y_: an ECMAScript language value, _comparator_: a function object or *undefined*, ): either a normal completion containing a Number or an abrupt completion
  id: sec-comparearrayelements

1. If _x_ is *undefined* and _y_ is *undefined*, return *+0*_𝔽.
2. If _x_ is *undefined*, return *1*_𝔽.
3. If _y_ is *undefined*, return *-1*_𝔽.
4. If _comparator_ is not *undefined*, then
   a. Let _result_ be ? ToNumber(? Call(_comparator_, *undefined*, « _x_, _y_ »)).
   b. If _result_ is *NaN*, return *+0*_𝔽.
   c. Return _result_.
5. Let _xString_ be ? ToString(_x_).
6. Let _yString_ be ? ToString(_y_).
7. Let _xSmaller_ be ! IsLessThan(_xString_, _yString_, *true*).
8. If _xSmaller_ is *true*, return *-1*_𝔽.
9. Let _ySmaller_ be ! IsLessThan(_yString_, _xString_, *true*).
10. If _ySmaller_ is *true*, return *1*_𝔽.
11. Return *+0*_𝔽.

--- referenced: Set ( _obj_: an Object, _propertyKey_: a property key, _value_: an ECMAScript language value, _throw_: a Boolean, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-set-o-p-v-throw

description: It is used to set the value of a specific property of an object. _value_ is the new value for the property.

1. Let _success_ be ? _obj_.[[Set]](_propertyKey_, _value_, _obj_).
2. If _success_ is *false* and _throw_ is *true*, throw a *TypeError* exception.
3. Return ~unused~.

--- referenced: ToString ( _arg_: an ECMAScript language value, ): either a normal completion containing a String or a throw completion
  id: sec-tostring

description: It converts _arg_ to a value of type String.

1. If _arg_ is a String, return _arg_.
2. If _arg_ is a Symbol, throw a *TypeError* exception.
3. If _arg_ is *undefined*, return *"undefined"*.
4. If _arg_ is *null*, return *"null"*.
5. If _arg_ is *true*, return *"true"*.
6. If _arg_ is *false*, return *"false"*.
7. If _arg_ is a Number, return Number::toString(_arg_, 10).
8. If _arg_ is a BigInt, return BigInt::toString(_arg_, 10).
9. Assert: _arg_ is an Object.
10. Let _primitiveValue_ be ? ToPrimitive(_arg_, ~string~).
11. Assert: _primitiveValue_ is not an Object.
12. Return ? ToString(_primitiveValue_).

--- referenced: DeletePropertyOrThrow ( _obj_: an Object, _propertyKey_: a property key, ): either a normal completion containing ~unused~ or a throw completion
  id: sec-deletepropertyorthrow

description: It is used to remove a specific own property of an object. It throws an exception if the property is not configurable.

1. Let _success_ be ? _obj_.[[Delete]](_propertyKey_).
2. If _success_ is *false*, throw a *TypeError* exception.
3. Return ~unused~.


## Done when

- Every test262 run under built-ins/Array/prototype/sort/ passes (check with `./t262 built-ins/Array/prototype/sort/`). 8 pass on the main branch right now.

