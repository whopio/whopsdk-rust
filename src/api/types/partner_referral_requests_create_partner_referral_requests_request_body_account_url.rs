pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct CreatePartnerReferralRequestsRequestBodyAccountUrl {
    /// HTTP or HTTPS Whop business or product link.
    pub account_url: String,
}

impl CreatePartnerReferralRequestsRequestBodyAccountUrl {
    pub fn builder() -> CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder {
        <CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder {
    account_url: Option<String>,
}

impl CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder {
    pub fn account_url(mut self, value: impl Into<String>) -> Self {
        self.account_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePartnerReferralRequestsRequestBodyAccountUrl`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_url`](CreatePartnerReferralRequestsRequestBodyAccountUrlBuilder::account_url)
    pub fn build(self) -> Result<CreatePartnerReferralRequestsRequestBodyAccountUrl, BuildError> {
        Ok(CreatePartnerReferralRequestsRequestBodyAccountUrl {
            account_url: self
                .account_url
                .ok_or_else(|| BuildError::missing_field("account_url"))?,
        })
    }
}
