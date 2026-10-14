//! String.raw ( _template_, ..._substitutions_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.raw

use kahvi_core::prelude::*;

/// String.raw
pub(crate) fn raw(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function may be called with a variable number of arguments. The first argument is _template_ and the remainder of the arguments form the List _substitutions_.
    // It performs the following steps when called:
    //
    // 1. Let _substitutionCount_ be the number of elements in _substitutions_.
    // 2. Let _cooked_ be ? ToObject(_template_).
    // 3. Let _literals_ be ? ToObject(? Get(_cooked_, *"raw"*)).
    // 4. Let _literalCount_ be ? LengthOfArrayLike(_literals_).
    // 5. If _literalCount_ ≤ 0, return the empty String.
    // 6. Let _result_ be the empty String.
    // 7. Let _nextIndex_ be 0.
    // 8. Repeat,
    //    a. Let _nextLiteralValue_ be ? Get(_literals_, ! ToString(𝔽(_nextIndex_))).
    //    b. Let _nextLiteral_ be ? ToString(_nextLiteralValue_).
    //    c. Set _result_ to the string-concatenation of _result_ and _nextLiteral_.
    //    d. If _nextIndex_ + 1 = _literalCount_, return _result_.
    //    e. If _nextIndex_ < _substitutionCount_, then
    //       i. Let _nextSubValue_ be _substitutions_[_nextIndex_].
    //       ii. Let _nextSub_ be ? ToString(_nextSubValue_).
    //       iii. Set _result_ to the string-concatenation of _result_ and _nextSub_.
    //    f. Set _nextIndex_ to _nextIndex_ + 1.
    //
    // NOTE:
    // This function is intended for use as a tag function of a Tagged Template ([sec-tagged-templates]). When called as such, the first argument will be a well formed template object and the rest parameter will contain the substitution values.
    let _ = (this, new_target);

    // 1. Let _substitutionCount_ be the number of elements in _substitutions_.
    let substitution_count = args.len().saturating_sub(1) as f64;
    // 2. Let _cooked_ be ? ToObject(_template_).
    let cooked = ops::to_object(agent, args.get_or_undefined(0))?;
    // 3. Let _literals_ be ? ToObject(? Get(_cooked_, *"raw"*)).
    let raw_key = agent.key("raw");
    let raw_value = ops::get(agent, cooked, raw_key)?;
    let literals = ops::to_object(agent, raw_value)?;
    // 4. Let _literalCount_ be ? LengthOfArrayLike(_literals_).
    let literal_count = ops::length_of_array_like(agent, literals)?;
    // 5. If _literalCount_ ≤ 0, return the empty String.
    if literal_count <= 0.0 {
        return Ok(Value::String(agent.string("")));
    }
    // The loop calls back into user code (getters, toString): the template, its
    // raw object and the substitutions stay rooted while it runs.
    let mut roots = args.to_vec();
    roots.push(Value::Object(literals));
    agent.with_roots(&roots, |agent| {
        // 6. Let _result_ be the empty String.
        // The result is accumulated as UTF-16 code units and interned at the end.
        let mut result: Vec<u16> = Vec::new();
        // 7. Let _nextIndex_ be 0.
        let mut next_index = 0.0;
        // 8. Repeat,
        loop {
            // a. Let _nextLiteralValue_ be ? Get(_literals_, ! ToString(𝔽(_nextIndex_))).
            let key = agent.key_from_index(next_index);
            let next_literal_value = ops::get(agent, literals, key)?;
            // b. Let _nextLiteral_ be ? ToString(_nextLiteralValue_).
            let next_literal = ops::to_string(agent, next_literal_value)?;
            // c. Set _result_ to the string-concatenation of _result_ and _nextLiteral_.
            result.extend_from_slice(agent.str(next_literal));
            // d. If _nextIndex_ + 1 = _literalCount_, return _result_.
            if next_index + 1.0 == literal_count {
                return Ok(Value::String(agent.intern(&result)));
            }
            // e. If _nextIndex_ < _substitutionCount_, then
            if next_index < substitution_count {
                // i. Let _nextSubValue_ be _substitutions_[_nextIndex_].
                let next_sub_value = args.get_or_undefined(next_index as usize + 1);
                // ii. Let _nextSub_ be ? ToString(_nextSubValue_).
                let next_sub = ops::to_string(agent, next_sub_value)?;
                // iii. Set _result_ to the string-concatenation of _result_ and _nextSub_.
                result.extend_from_slice(agent.str(next_sub));
            }
            // f. Set _nextIndex_ to _nextIndex_ + 1.
            next_index += 1.0;
        }
    })
}
