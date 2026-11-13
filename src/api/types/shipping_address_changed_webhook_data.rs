pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ShippingAddressChangedWebhookData {
    /// The original total, before your price for this address.
    #[serde(default)]
    pub amount: Money,
    /// The checkout session being priced, prefixed `chs_`, when the buyer is paying through one. `null` for a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_session_id: Option<String>,
    #[serde(default)]
    pub line_items: Vec<ShippingAddressChangedWebhookLineItem>,
    /// The payment being priced, prefixed `pay_`. `null` when the buyer is paying through a checkout session: the payment is created only after they approve, at your price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// The address the buyer picked. PayPal shares only these fields until the buyer approves.
    #[serde(default)]
    pub shipping_address: ShippingAddressChangedWebhookAddress,
}

impl ShippingAddressChangedWebhookData {
    pub fn builder() -> ShippingAddressChangedWebhookDataBuilder {
        <ShippingAddressChangedWebhookDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ShippingAddressChangedWebhookDataBuilder {
    amount: Option<Money>,
    checkout_session_id: Option<String>,
    line_items: Option<Vec<ShippingAddressChangedWebhookLineItem>>,
    payment_id: Option<String>,
    shipping_address: Option<ShippingAddressChangedWebhookAddress>,
}

impl ShippingAddressChangedWebhookDataBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn checkout_session_id(mut self, value: impl Into<String>) -> Self {
        self.checkout_session_id = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<ShippingAddressChangedWebhookLineItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: ShippingAddressChangedWebhookAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ShippingAddressChangedWebhookData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ShippingAddressChangedWebhookDataBuilder::amount)
    /// - [`line_items`](ShippingAddressChangedWebhookDataBuilder::line_items)
    /// - [`shipping_address`](ShippingAddressChangedWebhookDataBuilder::shipping_address)
    pub fn build(self) -> Result<ShippingAddressChangedWebhookData, BuildError> {
        Ok(ShippingAddressChangedWebhookData {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            checkout_session_id: self.checkout_session_id,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            payment_id: self.payment_id,
            shipping_address: self
                .shipping_address
                .ok_or_else(|| BuildError::missing_field("shipping_address"))?,
        })
    }
}
