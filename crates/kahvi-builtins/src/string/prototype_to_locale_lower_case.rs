//! String.prototype.toLocaleLowerCase ( [ _reserved1_ [ , _reserved2_ ] ] )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.tolocalelowercase

use kahvi_core::prelude::*;

/// String.prototype.toLocaleLowerCase
pub(crate) fn prototype_to_locale_lower_case(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // An ECMAScript implementation that includes the ECMA-402 Internationalization API must implement this method as specified in ECMA-402. Otherwise, the following specification of this method is used:
    // This method interprets a String value as a sequence of UTF-16 encoded code points, as described in [sec-ecmascript-language-types-string-type].
    // It works exactly the same as `toLowerCase` except that it is intended to yield a locale-sensitive result corresponding with conventions of the host environment's current locale. There will only be a difference in the few cases (such as Turkish) where the rules for that language conflict with the regular Unicode case mappings.
    // The meaning of the optional parameters to this method are defined in ECMA-402; implementations that do not include ECMA-402 support must not use those parameter positions for anything else.
    // NOTE:
    // This method is intentionally generic; it does not require that its *this* value be a String object. Therefore, it can be transferred to other kinds of objects for use as a method.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("String.prototype.toLocaleLowerCase"))
}
