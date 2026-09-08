pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResolutionLineItem {
    /// Line item ID, prefixed `li_`. Null when the payment predates item snapshots and the item is read from the payment's plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item's name as shown at checkout — the product title, else the plan title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The plan bought, prefixed `plan_`. Null when the plan has since been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// The product the plan belongs to, prefixed `prod_`. On a payment that predates item snapshots this falls back to the plan's product, so it can be set where the case's own `product_id` is null. Null for a plan with no product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// How many units were bought.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
}

impl ResolutionLineItem {
    pub fn builder() -> ResolutionLineItemBuilder {
        <ResolutionLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionLineItemBuilder {
    id: Option<String>,
    label: Option<String>,
    plan_id: Option<String>,
    product_id: Option<String>,
    quantity: Option<f64>,
}

impl ResolutionLineItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResolutionLineItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](ResolutionLineItemBuilder::quantity)
    pub fn build(self) -> Result<ResolutionLineItem, BuildError> {
        Ok(ResolutionLineItem {
            id: self.id,
            label: self.label,
            plan_id: self.plan_id,
            product_id: self.product_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
