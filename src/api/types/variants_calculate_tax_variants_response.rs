pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CalculateTaxVariantsResponse {
    /// Three-letter ISO 4217 currency code for the returned amounts.
    #[serde(default)]
    pub currency: String,
    /// Whether Whop calculated tax for this preview. `not_calculated` means no tax could be determined, so `tax_amount` is 0 and `total` equals `subtotal`.
    pub status: CalculateTaxVariantsResponseStatus,
    /// Variant price in the currency's smallest unit, for example cents. For exclusive tax, this is the pre-tax amount; for inclusive tax, it already includes tax and equals the total.
    #[serde(default)]
    pub subtotal: i64,
    /// Calculated tax amount in the currency's smallest unit. For exclusive tax, this is added on top of the subtotal; for inclusive tax, it is the portion of the subtotal that is tax.
    #[serde(default)]
    pub tax_amount: i64,
    /// Whether tax is added on top of the variant price or already included in it for this buyer's location.
    pub tax_behavior: CalculateTaxVariantsResponseTaxBehavior,
    /// Amount the buyer would pay in the currency's smallest unit.
    #[serde(default)]
    pub total: i64,
}

impl CalculateTaxVariantsResponse {
    pub fn builder() -> CalculateTaxVariantsResponseBuilder {
        <CalculateTaxVariantsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalculateTaxVariantsResponseBuilder {
    currency: Option<String>,
    status: Option<CalculateTaxVariantsResponseStatus>,
    subtotal: Option<i64>,
    tax_amount: Option<i64>,
    tax_behavior: Option<CalculateTaxVariantsResponseTaxBehavior>,
    total: Option<i64>,
}

impl CalculateTaxVariantsResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn status(mut self, value: CalculateTaxVariantsResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn subtotal(mut self, value: i64) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn tax_amount(mut self, value: i64) -> Self {
        self.tax_amount = Some(value);
        self
    }

    pub fn tax_behavior(mut self, value: CalculateTaxVariantsResponseTaxBehavior) -> Self {
        self.tax_behavior = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalculateTaxVariantsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CalculateTaxVariantsResponseBuilder::currency)
    /// - [`status`](CalculateTaxVariantsResponseBuilder::status)
    /// - [`subtotal`](CalculateTaxVariantsResponseBuilder::subtotal)
    /// - [`tax_amount`](CalculateTaxVariantsResponseBuilder::tax_amount)
    /// - [`tax_behavior`](CalculateTaxVariantsResponseBuilder::tax_behavior)
    /// - [`total`](CalculateTaxVariantsResponseBuilder::total)
    pub fn build(self) -> Result<CalculateTaxVariantsResponse, BuildError> {
        Ok(CalculateTaxVariantsResponse {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            subtotal: self
                .subtotal
                .ok_or_else(|| BuildError::missing_field("subtotal"))?,
            tax_amount: self
                .tax_amount
                .ok_or_else(|| BuildError::missing_field("tax_amount"))?,
            tax_behavior: self
                .tax_behavior
                .ok_or_else(|| BuildError::missing_field("tax_behavior"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
