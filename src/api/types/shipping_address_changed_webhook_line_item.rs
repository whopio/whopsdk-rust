pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ShippingAddressChangedWebhookLineItem {
    /// The line's original total, the same on every call for this purchase.
    #[serde(default)]
    pub amount: Money,
    /// Identifies the line. Your reply names each line by this id, so send it back unchanged.
    #[serde(default)]
    pub id: String,
    /// How many of the variant this line holds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
    /// The variant bought on this line, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl ShippingAddressChangedWebhookLineItem {
    pub fn builder() -> ShippingAddressChangedWebhookLineItemBuilder {
        <ShippingAddressChangedWebhookLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShippingAddressChangedWebhookLineItemBuilder {
    amount: Option<Money>,
    id: Option<String>,
    quantity: Option<f64>,
    variant_id: Option<String>,
}

impl ShippingAddressChangedWebhookLineItemBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShippingAddressChangedWebhookLineItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ShippingAddressChangedWebhookLineItemBuilder::amount)
    /// - [`id`](ShippingAddressChangedWebhookLineItemBuilder::id)
    /// - [`quantity`](ShippingAddressChangedWebhookLineItemBuilder::quantity)
    pub fn build(self) -> Result<ShippingAddressChangedWebhookLineItem, BuildError> {
        Ok(ShippingAddressChangedWebhookLineItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            variant_id: self.variant_id,
        })
    }
}
