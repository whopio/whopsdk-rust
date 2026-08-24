pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CalculateTaxPlansResponse {
    /// Three-letter ISO 4217 currency code for the returned amounts.
    #[serde(default)]
    pub currency: String,
    /// Whether Whop calculated tax for this preview. `not_calculated` means no tax could be determined, so `tax_amount` is 0 and `total` equals `subtotal`.
    pub status: CalculateTaxPlansResponseStatus,
    /// Plan price in the currency's smallest unit, for example cents. For exclusive tax, this is the pre-tax amount; for inclusive tax, it already includes tax and equals the total.
    #[serde(default)]
    pub subtotal: i64,
    /// Calculated tax amount in the currency's smallest unit. For exclusive tax, this is added on top of the subtotal; for inclusive tax, it is the portion of the subtotal that is tax.
    #[serde(default)]
    pub tax_amount: i64,
    /// Whether tax is added on top of the plan price or already included in it for this buyer's location.
    pub tax_behavior: CalculateTaxPlansResponseTaxBehavior,
    /// Amount the buyer would pay in the currency's smallest unit.
    #[serde(default)]
    pub total: i64,
}

impl CalculateTaxPlansResponse {
    pub fn builder() -> CalculateTaxPlansResponseBuilder {
        <CalculateTaxPlansResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CalculateTaxPlansResponseBuilder {
    currency: Option<String>,
    status: Option<CalculateTaxPlansResponseStatus>,
    subtotal: Option<i64>,
    tax_amount: Option<i64>,
    tax_behavior: Option<CalculateTaxPlansResponseTaxBehavior>,
    total: Option<i64>,
}

impl CalculateTaxPlansResponseBuilder {
    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn status(mut self, value: CalculateTaxPlansResponseStatus) -> Self {
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

    pub fn tax_behavior(mut self, value: CalculateTaxPlansResponseTaxBehavior) -> Self {
        self.tax_behavior = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CalculateTaxPlansResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CalculateTaxPlansResponseBuilder::currency)
    /// - [`status`](CalculateTaxPlansResponseBuilder::status)
    /// - [`subtotal`](CalculateTaxPlansResponseBuilder::subtotal)
    /// - [`tax_amount`](CalculateTaxPlansResponseBuilder::tax_amount)
    /// - [`tax_behavior`](CalculateTaxPlansResponseBuilder::tax_behavior)
    /// - [`total`](CalculateTaxPlansResponseBuilder::total)
    pub fn build(self) -> Result<CalculateTaxPlansResponse, BuildError> {
        Ok(CalculateTaxPlansResponse {
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
