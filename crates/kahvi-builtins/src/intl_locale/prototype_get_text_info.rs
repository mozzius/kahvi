//! Intl.Locale.prototype.getTextInfo ( )
//!
//! Spec: https://tc39.es/ecma402/#sec-Intl.Locale.prototype.getTextInfo

use kahvi_core::prelude::*;

/// Intl.Locale.prototype.getTextInfo
pub(crate) fn prototype_get_text_info(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // When the `getTextInfo` method is called, the following steps are taken:
    //
    // 1. Let _loc_ be the *this* value.
    // 2. Perform ? RequireInternalSlot(_loc_, [[InitializedLocale]]).
    // 3. Let _info_ be OrdinaryObjectCreate(%Object.prototype%).
    // 4. Let _dir_ be TextDirectionOfLocale(_loc_).
    // 5. Perform ! CreateDataPropertyOrThrow(_info_, *"direction"*, _dir_).
    // 6. Return _info_.
    let _ = (args, new_target);
    let loc = crate::support::intl_locale::require_locale(agent, this)?;
    let proto = agent.intrinsic("%Object.prototype%");
    let info = agent.ordinary_object_create(Some(proto));
    // TextDirectionOfLocale ( _loc_ ): "ltr", "rtl", or *undefined* when unknown.
    let dir = match icu::locale::Locale::try_from_str(&loc.locale) {
        Ok(parsed) => {
            let directionality = icu::locale::LocaleDirectionality::new_extended();
            if directionality.is_right_to_left(&parsed.id) {
                Some("rtl")
            } else if directionality.is_left_to_right(&parsed.id) {
                Some("ltr")
            } else {
                None
            }
        }
        Err(_) => {
            return Err(agent.not_implemented(
                "Intl.Locale.prototype.getTextInfo: locale identifier not parsed by ICU4X",
            ));
        }
    };
    let dir = crate::support::intl_locale::optional_string(agent, dir);
    let key = agent.key("direction");
    ops::create_data_property_or_throw(agent, info, key, dir)?;
    Ok(Value::Object(info))
}
