//! String.prototype.anchor ( _name_ )
//!
//! Spec: https://tc39.es/ecma262/#sec-string.prototype.anchor

use kahvi_core::prelude::*;

/// CreateHTML ( _contents_, _tag_, _attr_, _attrValue_ ), which is not yet in `ops`.
///
/// Spec: https://tc39.es/ecma262/#sec-createhtml
pub(crate) fn create_html(
    agent: &mut Agent,
    contents: Value,
    tag: &str,
    attr: &str,
    attr_value: Value,
) -> JsResult<Value> {
    // 1. Perform ? RequireObjectCoercible(_contents_).
    ops::require_object_coercible(agent, contents)?;
    // 2. Let _contentsString_ be ? ToString(_contents_).
    let contents_string = ops::to_string(agent, contents)?;
    // The conversions below may collect garbage: contents_string and attr_value stay rooted.
    agent.with_roots(&[Value::String(contents_string), attr_value], |agent| {
        // 3. Let _part1_ be the string-concatenation of *"<"* and _tag_.
        let mut part1: Vec<u16> = "<".encode_utf16().chain(tag.encode_utf16()).collect();
        // 4. If _attr_ is not the empty String, then
        if !attr.is_empty() {
            // a. Let _attrValueString_ be ? ToString(_attrValue_).
            let attr_value_string = ops::to_string(agent, attr_value)?;
            // b. Let _escapedAttrValue_ be the String value that is the same as _attrValueString_
            //    except that each occurrence of the code unit 0x0022 (QUOTATION MARK) in
            //    _attrValueString_ has been replaced with the six code unit sequence *"&quot;"*.
            let mut escaped_attr_value: Vec<u16> = Vec::new();
            for &unit in agent.str(attr_value_string) {
                if unit == 0x0022 {
                    escaped_attr_value.extend("&quot;".encode_utf16());
                } else {
                    escaped_attr_value.push(unit);
                }
            }
            // c. Set _part1_ to the string-concatenation of:
            //    _part1_, the code unit 0x0020 (SPACE), _attr_, the code unit 0x003D (EQUALS SIGN),
            //    the code unit 0x0022 (QUOTATION MARK), _escapedAttrValue_, and
            //    the code unit 0x0022 (QUOTATION MARK).
            part1.push(0x0020);
            part1.extend(attr.encode_utf16());
            part1.push(0x003D);
            part1.push(0x0022);
            part1.extend(escaped_attr_value);
            part1.push(0x0022);
        }
        // 5. Let _part2_ be the string-concatenation of _part1_ and *">"*.
        let mut part2 = part1;
        part2.push(0x003E);
        // 6. Let _part3_ be the string-concatenation of _part2_ and _contentsString_.
        let mut part3 = part2;
        part3.extend_from_slice(agent.str(contents_string));
        // 7. Let _part4_ be the string-concatenation of _part3_, *"</"*, _tag_, and *">"*.
        let mut part4 = part3;
        part4.extend("</".encode_utf16());
        part4.extend(tag.encode_utf16());
        part4.push(0x003E);
        // 8. Return _part4_.
        Ok(Value::String(agent.intern(&part4)))
    })
}

/// String.prototype.anchor
pub(crate) fn prototype_anchor(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _new_target: Option<Object>,
) -> JsResult<Value> {
    let name = args.get_or_undefined(0);
    // This method performs the following steps when called:
    //
    // 1. Let _string_ be the *this* value.
    let string = this;
    // 2. Return ? CreateHTML(_string_, *"a"*, *"name"*, _name_).
    create_html(agent, string, "a", "name", name)
}
