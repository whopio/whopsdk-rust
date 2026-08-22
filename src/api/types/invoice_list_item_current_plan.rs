pub use crate::prelude::*;

/// The plan that this invoice charges for.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceListItemCurrentPlan {
    /// The currency used for all prices on this plan (e.g., 'usd', 'eur'). All monetary amounts on the plan are denominated in this currency.
    pub currency: Currencies,
    /// The formatted price (including currency) for the plan.
    #[serde(default)]
    pub formatted_price: String,
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
}

impl InvoiceListItemCurrentPlan {
    pub fn builder() -> InvoiceListItemCurrentPlanBuilder {
        <InvoiceListItemCurrentPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceListItemCurrentPlanBuilder {
    currency: Option<Currencies>,
    formatted_price: Option<String>,
    id: Option<String>,
}

impl InvoiceListItemCurrentPlanBuilder {
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

    /// Consumes the builder and constructs a [`InvoiceListItemCurrentPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](InvoiceListItemCurrentPlanBuilder::currency)
    /// - [`formatted_price`](InvoiceListItemCurrentPlanBuilder::formatted_price)
    /// - [`id`](InvoiceListItemCurrentPlanBuilder::id)
    pub fn build(self) -> Result<InvoiceListItemCurrentPlan, BuildError> {
        Ok(InvoiceListItemCurrentPlan {
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
