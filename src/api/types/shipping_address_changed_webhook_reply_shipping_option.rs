pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ShippingAddressChangedWebhookReplyShippingOption {
    /// What this option costs at this address, in the currency of `data.amount`. Zero for free shipping.
    #[serde(default)]
    pub amount: Money,
    /// What the buyer sees for this option in PayPal, up to 127 characters.
    #[serde(default)]
    pub description: String,
    /// Your identifier for this option, unique within `shipping_options` and up to 127 characters. PayPal returns it to Whop when the buyer picks this option.
    #[serde(default)]
    pub external_identifier: String,
}

impl ShippingAddressChangedWebhookReplyShippingOption {
    pub fn builder() -> ShippingAddressChangedWebhookReplyShippingOptionBuilder {
        <ShippingAddressChangedWebhookReplyShippingOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShippingAddressChangedWebhookReplyShippingOptionBuilder {
    amount: Option<Money>,
    description: Option<String>,
    external_identifier: Option<String>,
}

impl ShippingAddressChangedWebhookReplyShippingOptionBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn external_identifier(mut self, value: impl Into<String>) -> Self {
        self.external_identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ShippingAddressChangedWebhookReplyShippingOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ShippingAddressChangedWebhookReplyShippingOptionBuilder::amount)
    /// - [`description`](ShippingAddressChangedWebhookReplyShippingOptionBuilder::description)
    /// - [`external_identifier`](ShippingAddressChangedWebhookReplyShippingOptionBuilder::external_identifier)
    pub fn build(self) -> Result<ShippingAddressChangedWebhookReplyShippingOption, BuildError> {
        Ok(ShippingAddressChangedWebhookReplyShippingOption {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            external_identifier: self
                .external_identifier
                .ok_or_else(|| BuildError::missing_field("external_identifier"))?,
        })
    }
}
