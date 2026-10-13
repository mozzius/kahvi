//! parseInt ( _string_, _radix_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-parseint-string-radix

use kahvi_core::prelude::*;

/// parseInt
pub(crate) fn parse_int(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function produces an integral Number dictated by interpretation of the contents of _string_ according to the specified _radix_. Leading white space in _string_ is ignored. If _radix_ coerces to 0 (such as when it is *undefined*), it is assumed to be 10 except when the number representation begins with *"0x"* or *"0X"*, in which case it is assumed to be 16. If _radix_ is 16, the number representation may optionally begin with *"0x"* or *"0X"*.
    // It is the %parseInt% intrinsic object.
    // It performs the following steps when called:
    //
    // 1. Let _inputString_ be ? ToString(_string_).
    // 2. Let _radixMV_ be ℝ(? ToInt32(_radix_)).
    // 3. If _radixMV_ ≠ 0 and _radixMV_ is not in the inclusive interval from 2 to 36, return *NaN*.
    // 4. Let _trimmedString_ be ! TrimString(_inputString_, ~start~).
    // 5. If _trimmedString_ is the empty String, return *NaN*.
    // 6. Let _sign_ be 1.
    // 7. If the first code unit of _trimmedString_ is the code unit 0x002D (HYPHEN-MINUS), then
    //    a. Set _sign_ to -1.
    //    b. Set _trimmedString_ to the substring of _trimmedString_ from 1.
    // 8. Else if the first code unit of _trimmedString_ is the code unit 0x002B (PLUS SIGN), then
    //    a. Set _trimmedString_ to the substring of _trimmedString_ from 1.
    // 9. If _radixMV_ = 0 or _radixMV_ = 16, then
    //    a. If the length of _trimmedString_ ≥ 2 and the substring of _trimmedString_ from 0 to 2 is either *"0x"* or *"0X"*, then
    //       i. Set _trimmedString_ to the substring of _trimmedString_ from 2.
    //       ii. Set _radixMV_ to 16.
    // 10. If _radixMV_ = 0, set _radixMV_ to 10.
    // 11. If _trimmedString_ contains a code unit that is not a radix-_radixMV_ digit, let _end_ be the index within _trimmedString_ of the first such code unit; else let _end_ be the length of _trimmedString_.
    // 12. Let _numberString_ be the substring of _trimmedString_ from 0 to _end_.
    // 13. If _numberString_ is empty, return *NaN*.
    // 14. Let _mathInt_ be the integer value that is represented by _numberString_ in radix-_radixMV_ notation, using the letters A through Z and a through z for digits with values 10 through 35. (However, if _radixMV_ = 10 and _numberString_ contains more than 20 significant digits, every significant digit after the 20th may be replaced by a 0 digit, at the option of the implementation; and if _radixMV_ is not one of 2, 4, 8, 10, 16, or 32, then _mathInt_ may be an implementation-approximated integer representing the integer value denoted by _numberString_ in radix-_radixMV_ notation.)
    // 15. If _sign_ = -1 and _mathInt_ = 0, return *-0*_𝔽.
    // 16. Return 𝔽(_sign_ × _mathInt_).
    //
    // NOTE:
    // This function may interpret only a leading portion of _string_ as an integer value; it ignores any code units that cannot be interpreted as part of the notation of an integer, and no indication is given that any such code units were ignored.
    let _ = (this, args, new_target);
    Err(agent.not_implemented("parseInt"))
}
