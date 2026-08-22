pub use crate::prelude::*;

/// The plan that this invoice charges for.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceCurrentPlan {
    /// The currency used for all prices on this plan (e.g., 'usd', 'eur'). All monetary amounts on the plan are denominated in this currency.
    pub currency: Currencies,
    /// A text description of the plan visible to customers. Maximum 1000 characters. Null if no description is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The formatted price (including currency) for the plan.
    #[serde(default)]
    pub formatted_price: String,
    /// The unique identifier for the plan.
    #[serde(default)]
    pub id: String,
}

impl InvoiceCurrentPlan {
    pub fn builder() -> InvoiceCurrentPlanBuilder {
        <InvoiceCurrentPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceCurrentPlanBuilder {
    currency: Option<Currencies>,
    description: Option<String>,
    formatted_price: Option<String>,
    id: Option<String>,
}

impl InvoiceCurrentPlanBuilder {
    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
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

    /// Consumes the builder and constructs a [`InvoiceCurrentPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](InvoiceCurrentPlanBuilder::currency)
    /// - [`formatted_price`](InvoiceCurrentPlanBuilder::formatted_price)
    /// - [`id`](InvoiceCurrentPlanBuilder::id)
    pub fn build(self) -> Result<InvoiceCurrentPlan, BuildError> {
        Ok(InvoiceCurrentPlan {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self.description,
            formatted_price: self
                .formatted_price
                .ok_or_else(|| BuildError::missing_field("formatted_price"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
