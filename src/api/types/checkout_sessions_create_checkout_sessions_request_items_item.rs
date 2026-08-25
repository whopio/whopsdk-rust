pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCheckoutSessionsRequestItemsItem {
    /// The plan being purchased (`plan_…`). The plan is the price.
    #[serde(default)]
    pub plan: String,
    /// How many of the plan. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

impl CreateCheckoutSessionsRequestItemsItem {
    pub fn builder() -> CreateCheckoutSessionsRequestItemsItemBuilder {
        <CreateCheckoutSessionsRequestItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckoutSessionsRequestItemsItemBuilder {
    plan: Option<String>,
    quantity: Option<i64>,
}

impl CreateCheckoutSessionsRequestItemsItemBuilder {
    pub fn plan(mut self, value: impl Into<String>) -> Self {
        self.plan = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCheckoutSessionsRequestItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan`](CreateCheckoutSessionsRequestItemsItemBuilder::plan)
    pub fn build(self) -> Result<CreateCheckoutSessionsRequestItemsItem, BuildError> {
        Ok(CreateCheckoutSessionsRequestItemsItem {
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            quantity: self.quantity,
        })
    }
}
