//! Proxy [[OwnPropertyKeys]] (§10.5)
//!
//! Spec: https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-ownpropertykeys

#![allow(unused_imports)]

use super::ProxyData;
use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::{PropertyDescriptor, PropertyKey};
use crate::value::{Object, Value};

/// The [[OwnPropertyKeys]] internal method of the Proxy exotic object `o` with slots `p`.
pub(crate) fn own_property_keys(
    agent: &mut Agent,
    o: Object,
    p: &ProxyData,
) -> JsResult<Vec<PropertyKey>> {
    // for: a Proxy exotic object _obj_
    //
    // 1. Perform ? ValidateNonRevokedProxy(_obj_).
    // 2. Let _target_ be _obj_.[[ProxyTarget]].
    // 3. Let _handler_ be _obj_.[[ProxyHandler]].
    // 4. Assert: _handler_ is an Object.
    // 5. Let _trap_ be ? GetMethod(_handler_, *"ownKeys"*).
    // 6. If _trap_ is *undefined*, then
    //    a. Return ? _target_.[[OwnPropertyKeys]]().
    // 7. Let _trapResultArray_ be ? Call(_trap_, _handler_, « _target_ »).
    // 8. Let _trapResult_ be ? CreateListFromArrayLike(_trapResultArray_, ~property-key~).
    // 9. If _trapResult_ contains any duplicate entries, throw a *TypeError* exception.
    // 10. Let _extensibleTarget_ be ? IsExtensible(_target_).
    // 11. Let _targetKeys_ be ? _target_.[[OwnPropertyKeys]]().
    // 12. Assert: _targetKeys_ is a List of property keys.
    // 13. Assert: _targetKeys_ contains no duplicate entries.
    // 14. Let _targetConfigurableKeys_ be a new empty List.
    // 15. Let _targetNonconfigurableKeys_ be a new empty List.
    // 16. For each element _key_ of _targetKeys_, do
    //    a. Let _propertyDesc_ be ? _target_.[[GetOwnProperty]](_key_).
    //    b. If _propertyDesc_ is not *undefined* and _propertyDesc_.[[Configurable]] is *false*, then
    //       i. Append _key_ to _targetNonconfigurableKeys_.
    //    c. Else,
    //       i. Append _key_ to _targetConfigurableKeys_.
    // 17. If _extensibleTarget_ is *true* and _targetNonconfigurableKeys_ is empty, then
    //    a. Return _trapResult_.
    // 18. Let _uncheckedResultKeys_ be a List whose elements are the elements of _trapResult_.
    // 19. For each element _key_ of _targetNonconfigurableKeys_, do
    //    a. If _uncheckedResultKeys_ does not contain _key_, throw a *TypeError* exception.
    //    b. Remove _key_ from _uncheckedResultKeys_.
    // 20. If _extensibleTarget_ is *true*, return _trapResult_.
    // 21. For each element _key_ of _targetConfigurableKeys_, do
    //    a. If _uncheckedResultKeys_ does not contain _key_, throw a *TypeError* exception.
    //    b. Remove _key_ from _uncheckedResultKeys_.
    // 22. If _uncheckedResultKeys_ is not empty, throw a *TypeError* exception.
    // 23. Return _trapResult_.
    //
    // NOTE:
    // [[OwnPropertyKeys]] for Proxy objects enforces the following invariants:
    // -
    // The result of [[OwnPropertyKeys]] is a List.
    // -
    // The returned List contains no duplicate entries.
    // -
    // Each element of the returned List is a property key.
    // -
    // The result List must contain the keys of all non-configurable own properties of the target object.
    // -
    // If the target object is not extensible, then the result List must contain all the keys of the own properties of the target object and no other values.
    let _ = (o, p);
    Err(agent.not_implemented("Proxy [[OwnPropertyKeys]]"))
}
