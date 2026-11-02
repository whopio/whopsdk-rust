pub use crate::prelude::*;

/// The payout account associated with the LedgerAccount, if any.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerAccountPayoutAccountDetails {
    /// The physical address associated with this payout account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<LedgerAccountPayoutAccountDetailsAddress>,
    /// The company's legal name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// The business representative for this payout account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_representative: Option<LedgerAccountPayoutAccountDetailsBusinessRepresentative>,
    /// The email address of the representative
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The unique identifier for the payout account.
    #[serde(default)]
    pub id: String,
    /// The latest verification for the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_verification: Option<LedgerAccountPayoutAccountDetailsLatestVerification>,
    /// The business representative's phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The granular calculated status of the payout account reflecting its current KYC and payout readiness state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PayoutAccountCalculatedStatuses>,
}

impl LedgerAccountPayoutAccountDetails {
    pub fn builder() -> LedgerAccountPayoutAccountDetailsBuilder {
        <LedgerAccountPayoutAccountDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerAccountPayoutAccountDetailsBuilder {
    address: Option<LedgerAccountPayoutAccountDetailsAddress>,
    business_name: Option<String>,
    business_representative: Option<LedgerAccountPayoutAccountDetailsBusinessRepresentative>,
    email: Option<String>,
    id: Option<String>,
    latest_verification: Option<LedgerAccountPayoutAccountDetailsLatestVerification>,
    phone: Option<String>,
    status: Option<PayoutAccountCalculatedStatuses>,
}

impl LedgerAccountPayoutAccountDetailsBuilder {
    pub fn address(mut self, value: LedgerAccountPayoutAccountDetailsAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_representative(
        mut self,
        value: LedgerAccountPayoutAccountDetailsBusinessRepresentative,
    ) -> Self {
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

    pub fn latest_verification(
        mut self,
        value: LedgerAccountPayoutAccountDetailsLatestVerification,
    ) -> Self {
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

    /// Consumes the builder and constructs a [`LedgerAccountPayoutAccountDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerAccountPayoutAccountDetailsBuilder::id)
    pub fn build(self) -> Result<LedgerAccountPayoutAccountDetails, BuildError> {
        Ok(LedgerAccountPayoutAccountDetails {
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
