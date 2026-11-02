pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct CreatePartnerReferralRequestsRequestBodyAccountId {
    /// Business to request attribution for, prefixed `biz_`.
    pub account_id: String,
}

impl CreatePartnerReferralRequestsRequestBodyAccountId {
    pub fn builder() -> CreatePartnerReferralRequestsRequestBodyAccountIdBuilder {
        <CreatePartnerReferralRequestsRequestBodyAccountIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnerReferralRequestsRequestBodyAccountIdBuilder {
    account_id: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyAccountIdBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePartnerReferralRequestsRequestBodyAccountId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreatePartnerReferralRequestsRequestBodyAccountIdBuilder::account_id)
    pub fn build(self) -> Result<CreatePartnerReferralRequestsRequestBodyAccountId, BuildError> {
        Ok(CreatePartnerReferralRequestsRequestBodyAccountId {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
