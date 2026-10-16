//! Intl.NumberFormat: the [[InitializedNumberFormat]] internal slots and the
//! abstract operations shared by its built-ins.
#![allow(dead_code)] // helpers for the Intl tickets

use kahvi_core::gc::{Trace, Tracer};
use kahvi_core::prelude::*;

use crate::support::intl::{default_number_option, get_number_option, get_string_option};

/// The [[RoundingType]] values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RoundingType {
    MorePrecision,
    LessPrecision,
    SignificantDigits,
    FractionDigits,
}

/// The internal slots of an Intl.NumberFormat (`None` is *undefined*).
#[derive(Debug, Clone)]
pub(crate) struct NumberFormatSlots {
    /// [[Locale]]
    pub locale: String,
    /// [[NumberingSystem]]
    pub numbering_system: String,
    /// [[Style]]
    pub style: String,
    /// [[Unit]]
    pub unit: Option<String>,
    /// [[UnitDisplay]]
    pub unit_display: Option<String>,
    /// [[Currency]]
    pub currency: Option<String>,
    /// [[CurrencyDisplay]]
    pub currency_display: Option<String>,
    /// [[CurrencySign]]
    pub currency_sign: Option<String>,
    /// [[MinimumIntegerDigits]]
    pub minimum_integer_digits: f64,
    /// [[MinimumFractionDigits]]
    pub minimum_fraction_digits: Option<f64>,
    /// [[MaximumFractionDigits]]
    pub maximum_fraction_digits: Option<f64>,
    /// [[MinimumSignificantDigits]]
    pub minimum_significant_digits: Option<f64>,
    /// [[MaximumSignificantDigits]]
    pub maximum_significant_digits: Option<f64>,
    /// [[RoundingType]]
    pub rounding_type: RoundingType,
    /// [[Notation]]
    pub notation: String,
    /// [[CompactDisplay]]
    pub compact_display: Option<String>,
    /// [[UseGrouping]]: a String, or *false*.
    pub use_grouping: Option<String>,
    /// [[SignDisplay]]
    pub sign_display: String,
    /// [[RoundingIncrement]]
    pub rounding_increment: f64,
    /// [[RoundingMode]]
    pub rounding_mode: String,
    /// [[ComputedRoundingPriority]]
    pub computed_rounding_priority: String,
    /// [[TrailingZeroDisplay]]
    pub trailing_zero_display: String,
    /// [[BoundFormat]]
    pub bound_format: Option<Object>,
}

impl Trace for NumberFormatSlots {
    fn trace(&self, t: &mut Tracer) {
        self.bound_format.trace(t);
    }
}

impl NumberFormatSlots {
    /// The slots before the constructor has filled them in.
    pub(crate) fn new(locale: String, numbering_system: String) -> Self {
        NumberFormatSlots {
            locale,
            numbering_system,
            style: String::new(),
            unit: None,
            unit_display: None,
            currency: None,
            currency_display: None,
            currency_sign: None,
            minimum_integer_digits: 1.0,
            minimum_fraction_digits: None,
            maximum_fraction_digits: None,
            minimum_significant_digits: None,
            maximum_significant_digits: None,
            rounding_type: RoundingType::FractionDigits,
            notation: String::new(),
            compact_display: None,
            use_grouping: None,
            sign_display: String::new(),
            rounding_increment: 1.0,
            rounding_mode: String::new(),
            computed_rounding_priority: String::new(),
            trailing_zero_display: String::new(),
            bound_format: None,
        }
    }
}

const SANCTIONED_SINGLE_UNITS: &[&str] = &[
    "acre",
    "bit",
    "byte",
    "celsius",
    "centimeter",
    "day",
    "degree",
    "fahrenheit",
    "fluid-ounce",
    "foot",
    "gallon",
    "gigabit",
    "gigabyte",
    "gram",
    "hectare",
    "hour",
    "inch",
    "kilobit",
    "kilobyte",
    "kilogram",
    "kilometer",
    "liter",
    "megabit",
    "megabyte",
    "meter",
    "microsecond",
    "mile",
    "mile-scandinavian",
    "milliliter",
    "millimeter",
    "millisecond",
    "minute",
    "month",
    "nanosecond",
    "ounce",
    "percent",
    "petabyte",
    "pound",
    "second",
    "stone",
    "terabit",
    "terabyte",
    "week",
    "yard",
    "year",
];

/// IsSanctionedSingleUnitIdentifier ( _unitIdentifier_ )
fn is_sanctioned_single_unit_identifier(unit_identifier: &str) -> bool {
    // 1. If _unitIdentifier_ is listed in [table-sanctioned-single-unit-identifiers] below, return *true*.
    // 2. Else, return *false*.
    SANCTIONED_SINGLE_UNITS.contains(&unit_identifier)
}

/// IsWellFormedUnitIdentifier ( _unitIdentifier_ )
pub(crate) fn is_well_formed_unit_identifier(unit_identifier: &str) -> bool {
    // 1. If IsSanctionedSingleUnitIdentifier(_unitIdentifier_) is *true*, then
    if is_sanctioned_single_unit_identifier(unit_identifier) {
        // a. Return *true*.
        return true;
    }
    // 2. Let _numeratorAndDenominator_ be StringSplitToList(_unitIdentifier_, *"-per-"*).
    let parts: Vec<&str> = unit_identifier.split("-per-").collect();
    // 3. If _numeratorAndDenominator_ does not have exactly 2 elements, return *false*.
    if parts.len() != 2 {
        return false;
    }
    // 4. If IsSanctionedSingleUnitIdentifier(_numeratorAndDenominator_[0]) is *false*, return *false*.
    // 5. If IsSanctionedSingleUnitIdentifier(_numeratorAndDenominator_[1]) is *false*, return *false*.
    // 6. Return *true*.
    is_sanctioned_single_unit_identifier(parts[0]) && is_sanctioned_single_unit_identifier(parts[1])
}

/// IsWellFormedCurrencyCode ( _currency_ )
pub(crate) fn is_well_formed_currency_code(currency: &str) -> bool {
    // 1. If the length of _currency_ is not 3, return *false*.
    // 2. Let _normalized_ be the ASCII-uppercase of _currency_.
    // 3. If _normalized_ contains any code unit outside of 0x0041 through 0x005A, return *false*.
    // 4. Return *true*.
    currency.len() == 3 && currency.bytes().all(|b| b.is_ascii_alphabetic())
}

/// SetNumberFormatUnitOptions ( _intlObj_, _options_ )
pub(crate) fn set_number_format_unit_options(
    agent: &mut Agent,
    intl_obj: &mut NumberFormatSlots,
    options: Object,
) -> JsResult<()> {
    // 1. Let _style_ be ? GetOption(_options_, *"style"*, ~string~, « *"decimal"*, *"percent"*, *"currency"*, *"unit"* », *"decimal"*).
    let style = get_string_option(
        agent,
        options,
        "style",
        &["decimal", "percent", "currency", "unit"],
        Some("decimal"),
    )?
    .unwrap_or_default();
    // 2. Set _intlObj_.[[Style]] to _style_.
    intl_obj.style = style.clone();
    // 3. Let _currency_ be ? GetOption(_options_, *"currency"*, ~string~, ~empty~, *undefined*).
    let currency = get_string_option(agent, options, "currency", &[], None)?;
    match &currency {
        // 4. If _currency_ is *undefined*, then
        None => {
            // a. If _style_ is *"currency"*, throw a *TypeError* exception.
            if style == "currency" {
                return Err(agent.throw_type_error("currency is required with style currency"));
            }
        }
        // 5. Else,
        Some(c) => {
            // a. If IsWellFormedCurrencyCode(_currency_) is *false*, throw a *RangeError* exception.
            if !is_well_formed_currency_code(c) {
                return Err(agent.throw_range_error("invalid currency code"));
            }
        }
    }
    // 6. Let _currencyDisplay_ be ? GetOption(_options_, *"currencyDisplay"*, ~string~, « *"code"*, *"symbol"*, *"narrowSymbol"*, *"name"* », *"symbol"*).
    let currency_display = get_string_option(
        agent,
        options,
        "currencyDisplay",
        &["code", "symbol", "narrowSymbol", "name"],
        Some("symbol"),
    )?;
    // 7. Let _currencySign_ be ? GetOption(_options_, *"currencySign"*, ~string~, « *"standard"*, *"accounting"* », *"standard"*).
    let currency_sign = get_string_option(
        agent,
        options,
        "currencySign",
        &["standard", "accounting"],
        Some("standard"),
    )?;
    // 8. Let _unit_ be ? GetOption(_options_, *"unit"*, ~string~, ~empty~, *undefined*).
    let unit = get_string_option(agent, options, "unit", &[], None)?;
    match &unit {
        // 9. If _unit_ is *undefined*, then
        None => {
            // a. If _style_ is *"unit"*, throw a *TypeError* exception.
            if style == "unit" {
                return Err(agent.throw_type_error("unit is required with style unit"));
            }
        }
        // 10. Else,
        Some(u) => {
            // a. If IsWellFormedUnitIdentifier(_unit_) is *false*, throw a *RangeError* exception.
            if !is_well_formed_unit_identifier(u) {
                return Err(agent.throw_range_error("invalid unit identifier"));
            }
        }
    }
    // 11. Let _unitDisplay_ be ? GetOption(_options_, *"unitDisplay"*, ~string~, « *"short"*, *"narrow"*, *"long"* », *"short"*).
    let unit_display = get_string_option(
        agent,
        options,
        "unitDisplay",
        &["short", "narrow", "long"],
        Some("short"),
    )?;
    // 12. If _style_ is *"currency"*, then
    if style == "currency" {
        // a. Set _intlObj_.[[Currency]] to the ASCII-uppercase of _currency_.
        intl_obj.currency = currency.map(|c| c.to_ascii_uppercase());
        // b. Set _intlObj_.[[CurrencyDisplay]] to _currencyDisplay_.
        intl_obj.currency_display = currency_display;
        // c. Set _intlObj_.[[CurrencySign]] to _currencySign_.
        intl_obj.currency_sign = currency_sign;
    }
    // 13. If _style_ is *"unit"*, then
    if style == "unit" {
        // a. Set _intlObj_.[[Unit]] to _unit_.
        intl_obj.unit = unit;
        // b. Set _intlObj_.[[UnitDisplay]] to _unitDisplay_.
        intl_obj.unit_display = unit_display;
    }
    // 14. Return ~unused~.
    Ok(())
}

/// SetNumberFormatDigitOptions ( _intlObj_, _options_, _mnfdDefault_, _mxfdDefault_, _notation_ )
///
/// Shared with Intl.PluralRules.
pub(crate) fn set_number_format_digit_options(
    agent: &mut Agent,
    intl_obj: &mut NumberFormatSlots,
    options: Object,
    mnfd_default: f64,
    mxfd_default: f64,
    notation: &str,
) -> JsResult<()> {
    let mut mxfd_default = mxfd_default;
    // 1. Let _mnid_ be ? GetNumberOption(_options_, *"minimumIntegerDigits"*, 1, 21, 1).
    let mnid = get_number_option(agent, options, "minimumIntegerDigits", 1.0, 21.0, Some(1.0))?;
    // 2. Let _mnfd_ be ? Get(_options_, *"minimumFractionDigits"*).
    let mnfd = get_value(agent, options, "minimumFractionDigits")?;
    // 3. Let _mxfd_ be ? Get(_options_, *"maximumFractionDigits"*).
    let mxfd = get_value(agent, options, "maximumFractionDigits")?;
    // 4. Let _mnsd_ be ? Get(_options_, *"minimumSignificantDigits"*).
    let mnsd = get_value(agent, options, "minimumSignificantDigits")?;
    // 5. Let _mxsd_ be ? Get(_options_, *"maximumSignificantDigits"*).
    let mxsd = get_value(agent, options, "maximumSignificantDigits")?;
    // 6. Set _intlObj_.[[MinimumIntegerDigits]] to _mnid_.
    intl_obj.minimum_integer_digits = mnid.unwrap_or(1.0);
    // 7. Let _roundingIncrement_ be ? GetNumberOption(_options_, *"roundingIncrement"*, 1, 5000, 1).
    let rounding_increment =
        get_number_option(agent, options, "roundingIncrement", 1.0, 5000.0, Some(1.0))?
            .unwrap_or(1.0);
    // 8. If _roundingIncrement_ is not in « 1, 2, 5, 10, 20, 25, 50, 100, 200, 250, 500, 1000, 2000, 2500, 5000 », throw a *RangeError* exception.
    const INCREMENTS: [f64; 15] = [
        1.0, 2.0, 5.0, 10.0, 20.0, 25.0, 50.0, 100.0, 200.0, 250.0, 500.0, 1000.0, 2000.0, 2500.0,
        5000.0,
    ];
    if !INCREMENTS.contains(&rounding_increment) {
        return Err(agent.throw_range_error("invalid roundingIncrement"));
    }
    // 9. Let _roundingMode_ be ? GetOption(_options_, *"roundingMode"*, ~string~, « ... », *"halfExpand"*).
    let rounding_mode = get_string_option(
        agent,
        options,
        "roundingMode",
        &[
            "ceil",
            "floor",
            "expand",
            "trunc",
            "halfCeil",
            "halfFloor",
            "halfExpand",
            "halfTrunc",
            "halfEven",
        ],
        Some("halfExpand"),
    )?
    .unwrap_or_default();
    // 10. Let _roundingPriority_ be ? GetOption(_options_, *"roundingPriority"*, ~string~, « *"auto"*, *"morePrecision"*, *"lessPrecision"* », *"auto"*).
    let rounding_priority = get_string_option(
        agent,
        options,
        "roundingPriority",
        &["auto", "morePrecision", "lessPrecision"],
        Some("auto"),
    )?
    .unwrap_or_default();
    // 11. Let _trailingZeroDisplay_ be ? GetOption(_options_, *"trailingZeroDisplay"*, ~string~, « *"auto"*, *"stripIfInteger"* », *"auto"*).
    let trailing_zero_display = get_string_option(
        agent,
        options,
        "trailingZeroDisplay",
        &["auto", "stripIfInteger"],
        Some("auto"),
    )?
    .unwrap_or_default();
    // 12. NOTE: All fields required by SetNumberFormatDigitOptions have now been read from _options_.
    // 13. If _roundingIncrement_ is not 1, set _mxfdDefault_ to _mnfdDefault_.
    if rounding_increment != 1.0 {
        mxfd_default = mnfd_default;
    }
    // 14. Set _intlObj_.[[RoundingIncrement]] to _roundingIncrement_.
    intl_obj.rounding_increment = rounding_increment;
    // 15. Set _intlObj_.[[RoundingMode]] to _roundingMode_.
    intl_obj.rounding_mode = rounding_mode;
    // 16. Set _intlObj_.[[TrailingZeroDisplay]] to _trailingZeroDisplay_.
    intl_obj.trailing_zero_display = trailing_zero_display;
    // 17. If _mnsd_ is *undefined* and _mxsd_ is *undefined*, let _hasSd_ be *false*. Otherwise, let _hasSd_ be *true*.
    let has_sd = !(matches!(mnsd, Value::Undefined) && matches!(mxsd, Value::Undefined));
    // 18. If _mnfd_ is *undefined* and _mxfd_ is *undefined*, let _hasFd_ be *false*. Otherwise, let _hasFd_ be *true*.
    let has_fd = !(matches!(mnfd, Value::Undefined) && matches!(mxfd, Value::Undefined));
    // 19. Let _needSd_ be *true*.
    let mut need_sd = true;
    // 20. Let _needFd_ be *true*.
    let mut need_fd = true;
    // 21. If _roundingPriority_ is *"auto"*, then
    if rounding_priority == "auto" {
        // a. Set _needSd_ to _hasSd_.
        need_sd = has_sd;
        // b. If _needSd_ is *true*, or _hasFd_ is *false* and _notation_ is *"compact"*, then
        if need_sd || (!has_fd && notation == "compact") {
            // i. Set _needFd_ to *false*.
            need_fd = false;
        }
    }
    // 22. If _needSd_ is *true*, then
    if need_sd {
        // a. If _hasSd_ is *true*, then
        if has_sd {
            // i. Set _intlObj_.[[MinimumSignificantDigits]] to ? DefaultNumberOption(_mnsd_, 1, 21, 1).
            let min = default_number_option(agent, mnsd, 1.0, 21.0, Some(1.0))?;
            intl_obj.minimum_significant_digits = min;
            // ii. Set _intlObj_.[[MaximumSignificantDigits]] to ? DefaultNumberOption(_mxsd_, _intlObj_.[[MinimumSignificantDigits]], 21, 21).
            intl_obj.maximum_significant_digits =
                default_number_option(agent, mxsd, min.unwrap_or(1.0), 21.0, Some(21.0))?;
        } else {
            // b. Else,
            // i. Set _intlObj_.[[MinimumSignificantDigits]] to 1.
            intl_obj.minimum_significant_digits = Some(1.0);
            // ii. Set _intlObj_.[[MaximumSignificantDigits]] to 21.
            intl_obj.maximum_significant_digits = Some(21.0);
        }
    }
    // 23. If _needFd_ is *true*, then
    if need_fd {
        // a. If _hasFd_ is *true*, then
        if has_fd {
            // i. Set _mnfd_ to ? DefaultNumberOption(_mnfd_, 0, 100, *undefined*).
            let mnfd = default_number_option(agent, mnfd, 0.0, 100.0, None)?;
            // ii. Set _mxfd_ to ? DefaultNumberOption(_mxfd_, 0, 100, *undefined*).
            let mxfd = default_number_option(agent, mxfd, 0.0, 100.0, None)?;
            let (mnfd, mxfd) = match (mnfd, mxfd) {
                // iii. If _mnfd_ is *undefined*, set _mnfd_ to min(_mnfdDefault_, _mxfd_).
                (None, Some(mxfd)) => (mnfd_default.min(mxfd), mxfd),
                // iv. Else if _mxfd_ is *undefined*, set _mxfd_ to max(_mxfdDefault_, _mnfd_).
                (Some(mnfd), None) => (mnfd, mxfd_default.max(mnfd)),
                // v. Else if _mnfd_ is greater than _mxfd_, throw a *RangeError* exception.
                (Some(mnfd), Some(mxfd)) => {
                    if mnfd > mxfd {
                        return Err(agent.throw_range_error(
                            "minimumFractionDigits is greater than maximumFractionDigits",
                        ));
                    }
                    (mnfd, mxfd)
                }
                // hasFd guarantees at least one of them was present.
                (None, None) => (mnfd_default, mxfd_default),
            };
            // vi. Set _intlObj_.[[MinimumFractionDigits]] to _mnfd_.
            intl_obj.minimum_fraction_digits = Some(mnfd);
            // vii. Set _intlObj_.[[MaximumFractionDigits]] to _mxfd_.
            intl_obj.maximum_fraction_digits = Some(mxfd);
        } else {
            // b. Else,
            // i. Set _intlObj_.[[MinimumFractionDigits]] to _mnfdDefault_.
            intl_obj.minimum_fraction_digits = Some(mnfd_default);
            // ii. Set _intlObj_.[[MaximumFractionDigits]] to _mxfdDefault_.
            intl_obj.maximum_fraction_digits = Some(mxfd_default);
        }
    }
    // 24. If _needSd_ is *false* and _needFd_ is *false*, then
    if !need_sd && !need_fd {
        // a. Set _intlObj_.[[MinimumFractionDigits]] to 0.
        intl_obj.minimum_fraction_digits = Some(0.0);
        // b. Set _intlObj_.[[MaximumFractionDigits]] to 0.
        intl_obj.maximum_fraction_digits = Some(0.0);
        // c. Set _intlObj_.[[MinimumSignificantDigits]] to 1.
        intl_obj.minimum_significant_digits = Some(1.0);
        // d. Set _intlObj_.[[MaximumSignificantDigits]] to 2.
        intl_obj.maximum_significant_digits = Some(2.0);
        // e. Set _intlObj_.[[RoundingType]] to ~more-precision~.
        intl_obj.rounding_type = RoundingType::MorePrecision;
        // f. Set _intlObj_.[[ComputedRoundingPriority]] to *"morePrecision"*.
        intl_obj.computed_rounding_priority = "morePrecision".to_string();
    } else if rounding_priority == "morePrecision" {
        // 25. Else if _roundingPriority_ is *"morePrecision"*, then
        intl_obj.rounding_type = RoundingType::MorePrecision;
        intl_obj.computed_rounding_priority = "morePrecision".to_string();
    } else if rounding_priority == "lessPrecision" {
        // 26. Else if _roundingPriority_ is *"lessPrecision"*, then
        intl_obj.rounding_type = RoundingType::LessPrecision;
        intl_obj.computed_rounding_priority = "lessPrecision".to_string();
    } else if has_sd {
        // 27. Else if _hasSd_ is *true*, then
        intl_obj.rounding_type = RoundingType::SignificantDigits;
        intl_obj.computed_rounding_priority = "auto".to_string();
    } else {
        // 28. Else,
        intl_obj.rounding_type = RoundingType::FractionDigits;
        intl_obj.computed_rounding_priority = "auto".to_string();
    }
    // 29. If _roundingIncrement_ is not 1, then
    if rounding_increment != 1.0 {
        // a. If _intlObj_.[[RoundingType]] is not ~fraction-digits~, throw a *TypeError* exception.
        if intl_obj.rounding_type != RoundingType::FractionDigits {
            return Err(
                agent.throw_type_error("roundingIncrement requires fraction digits rounding")
            );
        }
        // b. If _intlObj_.[[MaximumFractionDigits]] is not _intlObj_.[[MinimumFractionDigits]], throw a *RangeError* exception.
        if intl_obj.maximum_fraction_digits != intl_obj.minimum_fraction_digits {
            return Err(agent.throw_range_error(
                "roundingIncrement requires equal minimum and maximum fraction digits",
            ));
        }
    }
    // 30. Return ~unused~.
    Ok(())
}

/// Get ( _options_, _propertyKey_ )
fn get_value(agent: &mut Agent, options: Object, name: &str) -> JsResult<Value> {
    let key = agent.key(name);
    ops::get(agent, options, key)
}

/// UnwrapNumberFormat ( _nf_ ), as the object holding the slots.
pub(crate) fn unwrap_number_format(agent: &mut Agent, nf: Value) -> JsResult<Object> {
    // 1. If _nf_ is not an Object, throw a *TypeError* exception.
    let Value::Object(nf_obj) = nf else {
        return Err(agent.throw_type_error("not an Intl.NumberFormat"));
    };
    // 2. If _nf_ does not have an [[InitializedNumberFormat]] internal slot and ? OrdinaryHasInstance(%Intl.NumberFormat%, _nf_) is *true*, then
    if agent.native::<NumberFormatSlots>(nf_obj).is_none() {
        let ctor = Value::Object(agent.intrinsic("%Intl.NumberFormat%"));
        if ops::ordinary_has_instance(agent, ctor, nf)? {
            // a. Return ? Get(_nf_, %Intl%.[[FallbackSymbol]]).
            let key = PropertyKey::Symbol(crate::support::intl::fallback_symbol(agent));
            let v = ops::get(agent, nf_obj, key)?;
            return match v {
                Value::Object(o) => Ok(o),
                _ => Err(agent.throw_type_error("not an Intl.NumberFormat")),
            };
        }
    }
    // 3. Return _nf_.
    Ok(nf_obj)
}

/// %Intl.NumberFormat%.[[LocaleData]].[[<_locale_>]].[[nu]]: the locale's
/// default numbering system from ICU4X's decimal symbols (found through
/// locale fallback), then every other numbering system ICU4X has digits for.
/// *None* when ICU4X has no symbols for _locale_ or its ancestors.
pub(crate) fn numbering_systems_of(locale: &str) -> Option<Vec<Option<String>>> {
    use icu::decimal::provider::{Baked, DecimalSymbolsV1};
    use icu::locale::fallback::LocaleFallbacker;
    use icu_provider::prelude::*;

    let mut parsed = icu::locale::Locale::try_from_str(locale).ok()?;
    parsed.extensions = Default::default();
    let fallbacker = LocaleFallbacker::new();
    let mut iter = fallbacker
        .for_config(DecimalSymbolsV1::INFO.fallback_config)
        .fallback_for(DataLocale::from(&parsed));
    let default = loop {
        let mut metadata = DataRequestMetadata::default();
        metadata.silent = true;
        let request = DataRequest {
            id: DataIdentifierBorrowed::for_locale(iter.get()),
            metadata,
        };
        if let Ok(response) = DataProvider::<DecimalSymbolsV1>::load(&Baked, request) {
            break response.payload.get().numsys().to_string();
        }
        if iter.get().is_unknown() {
            return None;
        }
        iter.step();
    };
    let mut list = vec![Some(default.clone())];
    list.extend(
        crate::generated::intl::supported_values_of::numbering_systems_with_digits()?
            .into_iter()
            .filter(|n| *n != default)
            .map(Some),
    );
    Some(list)
}

/// CurrencyDigits ( _currency_ ): the fraction digits of CLDR's currency
/// data (ICU4X), whose default for currencies it does not list is 2.
pub(crate) fn currency_digits(currency: &str) -> Option<u8> {
    use icu::experimental::dimension::provider::currency::fractions::CurrencyFractionsV1;
    use icu_provider::prelude::*;

    // 1. Assert: IsWellFormedCurrencyCode(_currency_) is *true*.
    // 2. Return a non-negative integer indicating the number of fractional digits used when formatting quantities of the currency corresponding to _currency_. If there is no available information on the number of digits to be used, return 2.
    let code = icu::experimental::dimension::currency::CurrencyType::try_from_str(
        &currency.to_ascii_lowercase(),
    )
    .ok()?;
    let response = DataProvider::<CurrencyFractionsV1>::load(
        &icu::experimental::provider::Baked,
        DataRequest::default(),
    )
    .ok()?;
    Some(response.payload.get().resolve(code).digits)
}
