pub use crate::prelude::*;

/// An object representing an account used for payouts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutAccount {
    /// The physical address associated with this payout account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PayoutAccountAddress>,
    /// The company's legal name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// The business representative for this payout account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_representative: Option<PayoutAccountBusinessRepresentative>,
    /// The email address of the representative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The unique identifier for the payout account.
    #[serde(default)]
    pub id: String,
    /// The latest verification for the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_verification: Option<PayoutAccountLatestVerification>,
    /// The business representative's phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The granular calculated status of the payout account reflecting its current KYC and payout readiness state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PayoutAccountCalculatedStatuses>,
}

impl PayoutAccount {
    pub fn builder() -> PayoutAccountBuilder {
        <PayoutAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutAccountBuilder {
    address: Option<PayoutAccountAddress>,
    business_name: Option<String>,
    business_representative: Option<PayoutAccountBusinessRepresentative>,
    email: Option<String>,
    id: Option<String>,
    latest_verification: Option<PayoutAccountLatestVerification>,
    phone: Option<String>,
    status: Option<PayoutAccountCalculatedStatuses>,
}

impl PayoutAccountBuilder {
    pub fn address(mut self, value: PayoutAccountAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_representative(mut self, value: PayoutAccountBusinessRepresentative) -> Self {
        self.business_representative = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn latest_verification(mut self, value: PayoutAccountLatestVerification) -> Self {
        self.latest_verification = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn status(mut self, value: PayoutAccountCalculatedStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PayoutAccountBuilder::id)
    pub fn build(self) -> Result<PayoutAccount, BuildError> {
        Ok(PayoutAccount {
            address: self.address,
            business_name: self.business_name,
            business_representative: self.business_representative,
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            latest_verification: self.latest_verification,
            phone: self.phone,
            status: self.status,
        })
    }
}
