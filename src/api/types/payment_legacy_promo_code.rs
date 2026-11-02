pub use crate::prelude::*;

/// The promo code used for this payment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentLegacyPromoCode {
    /// The discount amount. Interpretation depends on promo_type: if 'percentage', this is the percentage (e.g., 20 means 20% off); if 'flat_amount', this is dollars off (e.g., 10.00 means $10.00 off).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount_off: f64,
    /// The monetary currency of the promo code.
    pub base_currency: Currencies,
    /// The specific code used to apply the promo at checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The unique identifier for the promo code.
    #[serde(default)]
    pub id: String,
    /// The number of months the promo is applied for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_intervals: Option<i64>,
    /// The type (% or flat amount) of the promo.
    pub promo_type: PromoTypes,
}

impl PaymentLegacyPromoCode {
    pub fn builder() -> PaymentLegacyPromoCodeBuilder {
        <PaymentLegacyPromoCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyPromoCodeBuilder {
    amount_off: Option<f64>,
    base_currency: Option<Currencies>,
    code: Option<String>,
    id: Option<String>,
    number_of_intervals: Option<i64>,
    promo_type: Option<PromoTypes>,
}

impl PaymentLegacyPromoCodeBuilder {
    pub fn amount_off(mut self, value: f64) -> Self {
        self.amount_off = Some(value);
        self
    }

    pub fn base_currency(mut self, value: Currencies) -> Self {
        self.base_currency = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn number_of_intervals(mut self, value: i64) -> Self {
        self.number_of_intervals = Some(value);
        self
    }

    pub fn promo_type(mut self, value: PromoTypes) -> Self {
        self.promo_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentLegacyPromoCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_off`](PaymentLegacyPromoCodeBuilder::amount_off)
    /// - [`base_currency`](PaymentLegacyPromoCodeBuilder::base_currency)
    /// - [`id`](PaymentLegacyPromoCodeBuilder::id)
    /// - [`promo_type`](PaymentLegacyPromoCodeBuilder::promo_type)
    pub fn build(self) -> Result<PaymentLegacyPromoCode, BuildError> {
        Ok(PaymentLegacyPromoCode {
            amount_off: self
                .amount_off
                .ok_or_else(|| BuildError::missing_field("amount_off"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            code: self.code,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            number_of_intervals: self.number_of_intervals,
            promo_type: self
                .promo_type
                .ok_or_else(|| BuildError::missing_field("promo_type"))?,
        })
    }
}
