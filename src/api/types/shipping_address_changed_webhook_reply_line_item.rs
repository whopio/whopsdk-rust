pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShippingAddressChangedWebhookReplyLineItem {
    /// The line's total at this address, in the currency of `data.amount`. It includes tax unless you send `tax`, and shipping unless you send `shipping_options`.
    #[serde(default)]
    pub amount: Money,
    /// The line's `id` from `data.line_items`, unchanged.
    #[serde(default)]
    pub id: String,
}

impl ShippingAddressChangedWebhookReplyLineItem {
    pub fn builder() -> ShippingAddressChangedWebhookReplyLineItemBuilder {
        <ShippingAddressChangedWebhookReplyLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShippingAddressChangedWebhookReplyLineItemBuilder {
    amount: Option<Money>,
    id: Option<String>,
}

impl ShippingAddressChangedWebhookReplyLineItemBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShippingAddressChangedWebhookReplyLineItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ShippingAddressChangedWebhookReplyLineItemBuilder::amount)
    /// - [`id`](ShippingAddressChangedWebhookReplyLineItemBuilder::id)
    pub fn build(self) -> Result<ShippingAddressChangedWebhookReplyLineItem, BuildError> {
        Ok(ShippingAddressChangedWebhookReplyLineItem {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
