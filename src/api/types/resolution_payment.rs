pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionPayment {
    /// Card brand, when the customer paid by card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brand: Option<String>,
    /// Last four digits of the card, when the customer paid by card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_last4: Option<String>,
    /// When the payment was made, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Payment ID, prefixed `pay_`.
    #[serde(default)]
    pub id: String,
    /// How the customer paid, such as `card` or `paypal`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<String>,
}

impl ResolutionPayment {
    pub fn builder() -> ResolutionPaymentBuilder {
        <ResolutionPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionPaymentBuilder {
    card_brand: Option<String>,
    card_last4: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    payment_method_type: Option<String>,
}

impl ResolutionPaymentBuilder {
    pub fn card_brand(mut self, value: impl Into<String>) -> Self {
        self.card_brand = Some(value.into());
        self
    }

    pub fn card_last4(mut self, value: impl Into<String>) -> Self {
        self.card_last4 = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolutionPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ResolutionPaymentBuilder::created_at)
    /// - [`id`](ResolutionPaymentBuilder::id)
    pub fn build(self) -> Result<ResolutionPayment, BuildError> {
        Ok(ResolutionPayment {
            card_brand: self.card_brand,
            card_last4: self.card_last4,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payment_method_type: self.payment_method_type,
        })
    }
}
