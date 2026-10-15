//! Intl.supportedValuesOf ( _key_ )
//!
//! Spec: https://tc39.es/ecma402/#sec-intl.supportedvaluesof

use kahvi_core::prelude::*;

/// Intl.supportedValuesOf
pub(crate) fn supported_values_of(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `supportedValuesOf` method is called with argument _key_ , the following steps are taken:
    //
    // 1. Set _key_ to ? ToString(_key_).
    // 2. If _key_ is *"calendar"*, then
    //    a. Let _list_ be a new empty List.
    //    b. For each element _identifier_ of AvailableCalendars(), do
    //       i. Let _canonical_ be CanonicalizeUValue(*"ca"*, _identifier_).
    //       ii. If _identifier_ is _canonical_, then
    //          1. Append _identifier_ to _list_.
    // 3. Else if _key_ is *"collation"*, then
    //    a. Let _list_ be AvailableCanonicalCollations( ).
    // 4. Else if _key_ is *"currency"*, then
    //    a. Let _list_ be AvailableCanonicalCurrencies( ).
    // 5. Else if _key_ is *"numberingSystem"*, then
    //    a. Let _list_ be AvailableCanonicalNumberingSystems( ).
    // 6. Else if _key_ is *"timeZone"*, then
    //    a. Let _list_ be AvailablePrimaryTimeZoneIdentifiers( ).
    // 7. Else if _key_ is *"unit"*, then
    //    a. Let _list_ be AvailableCanonicalUnits( ).
    // 8. Else,
    //    a. Throw a *RangeError* exception.
    // 9. Return CreateArrayFromList( _list_ ).
    let _ = (this, args, new_target);
    Err(agent.not_implemented("Intl.supportedValuesOf"))
}
