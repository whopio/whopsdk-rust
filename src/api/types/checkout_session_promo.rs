pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckoutSessionPromo {
    /// What each covered charge is discounted by. Percentage discounts are represented as a decimal fraction; fixed-amount discounts are in `currency`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount_off: f64,
    /// The promo code, exactly as `promo_code` states it.
    #[serde(default)]
    pub code: String,
    /// Currency of a fixed-amount discount; `null` for a percentage one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CheckoutSessionPromoCurrency>,
    /// Which charges the discount covers: `forever` discounts every charge; `once` covers only the charge at purchase — the code is spent then even when it made that charge free, except on a free trial with nothing due today, where it holds until the trial's first real charge; `repeating` covers every charge landing within `number_of_intervals` calendar months of purchase.
    pub duration: CheckoutSessionPromoDuration,
    /// How many calendar months of charges a `repeating` promo covers, counted from purchase — a renewal scheduled past that window bills full price however few charges came before it. `null` for the other durations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_intervals: Option<i64>,
    /// Whether the discount is percentage-based or a fixed amount.
    pub promo_type: CheckoutSessionPromoPromoType,
}

impl CheckoutSessionPromo {
    pub fn builder() -> CheckoutSessionPromoBuilder {
        <CheckoutSessionPromoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionPromoBuilder {
    amount_off: Option<f64>,
    code: Option<String>,
    currency: Option<CheckoutSessionPromoCurrency>,
    duration: Option<CheckoutSessionPromoDuration>,
    number_of_intervals: Option<i64>,
    promo_type: Option<CheckoutSessionPromoPromoType>,
}

impl CheckoutSessionPromoBuilder {
    pub fn amount_off(mut self, value: f64) -> Self {
        self.amount_off = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn currency(mut self, value: CheckoutSessionPromoCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn duration(mut self, value: CheckoutSessionPromoDuration) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn number_of_intervals(mut self, value: i64) -> Self {
        self.number_of_intervals = Some(value);
        self
    }

    pub fn promo_type(mut self, value: CheckoutSessionPromoPromoType) -> Self {
        self.promo_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionPromo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_off`](CheckoutSessionPromoBuilder::amount_off)
    /// - [`code`](CheckoutSessionPromoBuilder::code)
    /// - [`duration`](CheckoutSessionPromoBuilder::duration)
    /// - [`promo_type`](CheckoutSessionPromoBuilder::promo_type)
    pub fn build(self) -> Result<CheckoutSessionPromo, BuildError> {
        Ok(CheckoutSessionPromo {
            amount_off: self
                .amount_off
                .ok_or_else(|| BuildError::missing_field("amount_off"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            currency: self.currency,
            duration: self
                .duration
                .ok_or_else(|| BuildError::missing_field("duration"))?,
            number_of_intervals: self.number_of_intervals,
            promo_type: self
                .promo_type
                .ok_or_else(|| BuildError::missing_field("promo_type"))?,
        })
    }
}
