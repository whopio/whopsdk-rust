pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfirmationToken {
    /// Enough of the billing details to raise a customer record and recognise the method — email, name, country and postal code. The street address is collected for the charge but never returned; this endpoint is a display-safe preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<PaymentBillingDetailsPreview>,
    /// When the token was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the token expires, as an ISO 8601 timestamp. Tokens are single-use and short-lived.
    #[serde(default)]
    pub expires_at: String,
    #[serde(default)]
    pub id: String,
    /// Always `confirmation_token`.
    #[serde(default)]
    pub object: String,
    /// Display-only preview of the collected method — never the underlying token.
    pub payment_method_preview: PaymentMethodDisplay,
    /// Save-consent state the element displayed at collection: `off_session`, `on_session`, or `null`. Confirm may vault only if attested here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<String>,
    /// `pending` until it is used, then `consumed`; `expired` once its short lifetime elapses. Only a `pending` token can be charged.
    pub status: ConfirmationTokenStatus,
}

impl ConfirmationToken {
    pub fn builder() -> ConfirmationTokenBuilder {
        <ConfirmationTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfirmationTokenBuilder {
    billing_details: Option<PaymentBillingDetailsPreview>,
    created_at: Option<String>,
    expires_at: Option<String>,
    id: Option<String>,
    object: Option<String>,
    payment_method_preview: Option<PaymentMethodDisplay>,
    setup_future_usage: Option<String>,
    status: Option<ConfirmationTokenStatus>,
}

impl ConfirmationTokenBuilder {
    pub fn billing_details(mut self, value: PaymentBillingDetailsPreview) -> Self {
        self.billing_details = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn payment_method_preview(mut self, value: PaymentMethodDisplay) -> Self {
        self.payment_method_preview = Some(value);
        self
    }

    pub fn setup_future_usage(mut self, value: impl Into<String>) -> Self {
        self.setup_future_usage = Some(value.into());
        self
    }

    pub fn status(mut self, value: ConfirmationTokenStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfirmationToken`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ConfirmationTokenBuilder::created_at)
    /// - [`expires_at`](ConfirmationTokenBuilder::expires_at)
    /// - [`id`](ConfirmationTokenBuilder::id)
    /// - [`object`](ConfirmationTokenBuilder::object)
    /// - [`payment_method_preview`](ConfirmationTokenBuilder::payment_method_preview)
    /// - [`status`](ConfirmationTokenBuilder::status)
    pub fn build(self) -> Result<ConfirmationToken, BuildError> {
        Ok(ConfirmationToken {
            billing_details: self.billing_details,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payment_method_preview: self
                .payment_method_preview
                .ok_or_else(|| BuildError::missing_field("payment_method_preview"))?,
            setup_future_usage: self.setup_future_usage,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
