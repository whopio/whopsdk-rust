pub use crate::prelude::*;

/// The plan that this invoice charges for.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LegacyInvoiceListItemCurrentPlan {
    /// The currency used for all prices on this plan (e.g., 'usd', 'eur'). All monetary amounts on the plan are denominated in this currency.
    pub currency: Currencies,
    /// The formatted price (including currency) for the plan.
    #[serde(default)]
    pub formatted_price: String,
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
}

impl LegacyInvoiceListItemCurrentPlan {
    pub fn builder() -> LegacyInvoiceListItemCurrentPlanBuilder {
        <LegacyInvoiceListItemCurrentPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LegacyInvoiceListItemCurrentPlanBuilder {
    currency: Option<Currencies>,
    formatted_price: Option<String>,
    id: Option<String>,
}

impl LegacyInvoiceListItemCurrentPlanBuilder {
    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn formatted_price(mut self, value: impl Into<String>) -> Self {
        self.formatted_price = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LegacyInvoiceListItemCurrentPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](LegacyInvoiceListItemCurrentPlanBuilder::currency)
    /// - [`formatted_price`](LegacyInvoiceListItemCurrentPlanBuilder::formatted_price)
    /// - [`id`](LegacyInvoiceListItemCurrentPlanBuilder::id)
    pub fn build(self) -> Result<LegacyInvoiceListItemCurrentPlan, BuildError> {
        Ok(LegacyInvoiceListItemCurrentPlan {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            formatted_price: self
                .formatted_price
                .ok_or_else(|| BuildError::missing_field("formatted_price"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
