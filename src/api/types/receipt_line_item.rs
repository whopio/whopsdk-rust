pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceiptLineItem {
    /// Line item ID, prefixed `li_`. Null when the payment predates item snapshots and the item is read from the payment's plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item's name as shown at checkout — the product title, else the plan title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The plan bought, prefixed `plan_`. Null when the plan has since been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// The plan's current title, or `null` when the plan has been deleted or has no title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_title: Option<String>,
    /// The product the plan belongs to, prefixed `prod_`. On a payment that predates item snapshots this falls back to the plan's product, so it can be set where the parent's own `product_id` is null. Null for a plan with no product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The product's current title, or `null` when the item has no product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_title: Option<String>,
    /// How many units were bought.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
    /// The recorded amount for this item's full quantity, before discounts, tax, and fees, in its purchase currency. This is not the amount being contested. Returns `null` when no item amount was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<Money>,
}

impl ReceiptLineItem {
    pub fn builder() -> ReceiptLineItemBuilder {
        <ReceiptLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptLineItemBuilder {
    id: Option<String>,
    label: Option<String>,
    plan_id: Option<String>,
    plan_title: Option<String>,
    product_id: Option<String>,
    product_title: Option<String>,
    quantity: Option<f64>,
    subtotal: Option<Money>,
}

impl ReceiptLineItemBuilder {
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

    pub fn plan_title(mut self, value: impl Into<String>) -> Self {
        self.plan_title = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn product_title(mut self, value: impl Into<String>) -> Self {
        self.product_title = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn subtotal(mut self, value: Money) -> Self {
        self.subtotal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptLineItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](ReceiptLineItemBuilder::quantity)
    pub fn build(self) -> Result<ReceiptLineItem, BuildError> {
        Ok(ReceiptLineItem {
            id: self.id,
            label: self.label,
            plan_id: self.plan_id,
            plan_title: self.plan_title,
            product_id: self.product_id,
            product_title: self.product_title,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            subtotal: self.subtotal,
        })
    }
}
