pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShippingAddressChangedWebhookReply {
    #[serde(default)]
    pub line_items: Vec<ShippingAddressChangedWebhookReplyLineItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingAddressChangedWebhookReplyShippingOption>>,
    /// The tax you charge at this address, in the currency of `data.amount`. PayPal shows it on its own Tax line, and the buyer pays it on top of the line totals. Leave it out to keep tax inside the line totals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Money>,
}

impl ShippingAddressChangedWebhookReply {
    pub fn builder() -> ShippingAddressChangedWebhookReplyBuilder {
        <ShippingAddressChangedWebhookReplyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShippingAddressChangedWebhookReplyBuilder {
    line_items: Option<Vec<ShippingAddressChangedWebhookReplyLineItem>>,
    shipping_options: Option<Vec<ShippingAddressChangedWebhookReplyShippingOption>>,
    tax: Option<Money>,
}

impl ShippingAddressChangedWebhookReplyBuilder {
    pub fn line_items(mut self, value: Vec<ShippingAddressChangedWebhookReplyLineItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn shipping_options(
        mut self,
        value: Vec<ShippingAddressChangedWebhookReplyShippingOption>,
    ) -> Self {
        self.shipping_options = Some(value);
        self
    }

    pub fn tax(mut self, value: Money) -> Self {
        self.tax = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShippingAddressChangedWebhookReply`].
    /// This method will fail if any of the following fields are not set:
    /// - [`line_items`](ShippingAddressChangedWebhookReplyBuilder::line_items)
    pub fn build(self) -> Result<ShippingAddressChangedWebhookReply, BuildError> {
        Ok(ShippingAddressChangedWebhookReply {
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            shipping_options: self.shipping_options,
            tax: self.tax,
        })
    }
}
