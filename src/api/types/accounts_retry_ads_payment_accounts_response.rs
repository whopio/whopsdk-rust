pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetryAdsPaymentAccountsResponse {
    /// The account whose ads payments will be retried.
    #[serde(default)]
    pub account_id: String,
    /// Whether the retry was accepted for background processing.
    #[serde(default)]
    pub queued: bool,
}

impl RetryAdsPaymentAccountsResponse {
    pub fn builder() -> RetryAdsPaymentAccountsResponseBuilder {
        <RetryAdsPaymentAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetryAdsPaymentAccountsResponseBuilder {
    account_id: Option<String>,
    queued: Option<bool>,
}

impl RetryAdsPaymentAccountsResponseBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn queued(mut self, value: bool) -> Self {
        self.queued = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetryAdsPaymentAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](RetryAdsPaymentAccountsResponseBuilder::account_id)
    /// - [`queued`](RetryAdsPaymentAccountsResponseBuilder::queued)
    pub fn build(self) -> Result<RetryAdsPaymentAccountsResponse, BuildError> {
        Ok(RetryAdsPaymentAccountsResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            queued: self
                .queued
                .ok_or_else(|| BuildError::missing_field("queued"))?,
        })
    }
}
