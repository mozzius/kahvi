//! Object.prototype.toString ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-object.prototype.tostring

use kahvi_core::prelude::*;

/// Object.prototype.toString
pub(crate) fn prototype_to_string(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This method performs the following steps when called:
    //
    // 1. If the *this* value is *undefined*, return *"[object Undefined]"*.
    // 2. If the *this* value is *null*, return *"[object Null]"*.
    // 3. Let _obj_ be ! ToObject(*this* value).
    // 4. Let _isArray_ be ? IsArray(_obj_).
    // 5. If _isArray_ is *true*, let _builtinTag_ be *"Array"*.
    // 6. Else if _obj_ has a [[ParameterMap]] internal slot, let _builtinTag_ be *"Arguments"*.
    // 7. Else if _obj_ has a [[Call]] internal method, let _builtinTag_ be *"Function"*.
    // 8. Else if _obj_ has an [[ErrorData]] internal slot, let _builtinTag_ be *"Error"*.
    // 9. Else if _obj_ has a [[BooleanData]] internal slot, let _builtinTag_ be *"Boolean"*.
    // 10. Else if _obj_ has a [[NumberData]] internal slot, let _builtinTag_ be *"Number"*.
    // 11. Else if _obj_ has a [[StringData]] internal slot, let _builtinTag_ be *"String"*.
    // 12. Else if _obj_ has a [[DateValue]] internal slot, let _builtinTag_ be *"Date"*.
    // 13. Else if _obj_ has a [[RegExpMatcher]] internal slot, let _builtinTag_ be *"RegExp"*.
    // 14. Else, let _builtinTag_ be *"Object"*.
    // 15. Let _tag_ be ? Get(_obj_, %Symbol.toStringTag%).
    // 16. If _tag_ is not a String, set _tag_ to _builtinTag_.
    // 17. Return the string-concatenation of *"[object "*, _tag_, and *"]"*.
    //
    // NOTE:
    // Historically, this method was occasionally used to access the String value of the [[Class]] internal slot that was used in previous editions of this specification as a nominal type tag for various built-in objects. The above definition of `toString` preserves compatibility for legacy code that uses `toString` as a test for those specific kinds of built-in objects. It does not provide a reliable type testing mechanism for other kinds of built-in or program defined objects. In addition, programs can use %Symbol.toStringTag% in ways that will invalidate the reliability of such legacy type tests.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Object.prototype.toString"))
}
