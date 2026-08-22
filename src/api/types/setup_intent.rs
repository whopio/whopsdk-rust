pub use crate::prelude::*;

/// A setup intent allows a user to save a payment method for future use without making an immediate purchase.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetupIntent {
    /// The checkout session configuration associated with this setup intent. Null if no checkout session was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_configuration: Option<SetupIntentCheckoutConfiguration>,
    /// The company that initiated this setup intent. Null if the company has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<SetupIntentCompany>,
    /// The datetime the setup intent was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// A human-readable error message explaining why the setup intent failed. Null if no error occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The unique identifier for the setup intent.
    #[serde(default)]
    pub id: String,
    /// The company member associated with this setup intent. Null if the user is not a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<SetupIntentMember>,
    /// Custom key-value pairs attached to this setup intent. Null if no metadata was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The saved payment method created by this setup intent. Null if the setup has not completed successfully.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<SetupIntentPaymentMethod>,
    /// The current status of the setup intent.
    pub status: SetupIntentStatuses,
    /// Whether 3D Secure authentication was completed when this payment method was set up.
    #[serde(default)]
    pub three_ds_verified: bool,
}

impl SetupIntent {
    pub fn builder() -> SetupIntentBuilder {
        <SetupIntentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentBuilder {
    checkout_configuration: Option<SetupIntentCheckoutConfiguration>,
    company: Option<SetupIntentCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    error_message: Option<String>,
    id: Option<String>,
    member: Option<SetupIntentMember>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    payment_method: Option<SetupIntentPaymentMethod>,
    status: Option<SetupIntentStatuses>,
    three_ds_verified: Option<bool>,
}

impl SetupIntentBuilder {
    pub fn checkout_configuration(mut self, value: SetupIntentCheckoutConfiguration) -> Self {
        self.checkout_configuration = Some(value);
        self
    }

    pub fn company(mut self, value: SetupIntentCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn member(mut self, value: SetupIntentMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn payment_method(mut self, value: SetupIntentPaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn status(mut self, value: SetupIntentStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn three_ds_verified(mut self, value: bool) -> Self {
        self.three_ds_verified = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetupIntent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SetupIntentBuilder::created_at)
    /// - [`id`](SetupIntentBuilder::id)
    /// - [`status`](SetupIntentBuilder::status)
    /// - [`three_ds_verified`](SetupIntentBuilder::three_ds_verified)
    pub fn build(self) -> Result<SetupIntent, BuildError> {
        Ok(SetupIntent {
            checkout_configuration: self.checkout_configuration,
            company: self.company,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            error_message: self.error_message,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            member: self.member,
            metadata: self.metadata,
            payment_method: self.payment_method,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            three_ds_verified: self
                .three_ds_verified
                .ok_or_else(|| BuildError::missing_field("three_ds_verified"))?,
        })
    }
}
