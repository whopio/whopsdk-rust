pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaymentsRequestLineItemsItem {
    /// An existing plan to charge for, prefixed `plan_`. Each plan may appear once — use `quantity` for multiple units.
    #[serde(default)]
    pub plan_id: String,
    /// How many units of the plan to purchase. Defaults to 1; more than 1 requires the plan to allow multiple quantities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

impl CreatePaymentsRequestLineItemsItem {
    pub fn builder() -> CreatePaymentsRequestLineItemsItemBuilder {
        <CreatePaymentsRequestLineItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestLineItemsItemBuilder {
    plan_id: Option<String>,
    quantity: Option<i64>,
}

impl CreatePaymentsRequestLineItemsItemBuilder {
    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestLineItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan_id`](CreatePaymentsRequestLineItemsItemBuilder::plan_id)
    pub fn build(self) -> Result<CreatePaymentsRequestLineItemsItem, BuildError> {
        Ok(CreatePaymentsRequestLineItemsItem {
            plan_id: self
                .plan_id
                .ok_or_else(|| BuildError::missing_field("plan_id"))?,
            quantity: self.quantity,
        })
    }
}
