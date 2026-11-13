pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApproveAllWaitlistEntriesResponse {
    /// The seller account whose signups were queued, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Whether any pending signups were queued for approval. `false` when there were none.
    #[serde(default)]
    pub queued: bool,
    /// The variant the request was narrowed to, prefixed `plan_`, or `null` when every waitlist variant on the account was included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl ApproveAllWaitlistEntriesResponse {
    pub fn builder() -> ApproveAllWaitlistEntriesResponseBuilder {
        <ApproveAllWaitlistEntriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApproveAllWaitlistEntriesResponseBuilder {
    account_id: Option<String>,
    queued: Option<bool>,
    variant_id: Option<String>,
}

impl ApproveAllWaitlistEntriesResponseBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn queued(mut self, value: bool) -> Self {
        self.queued = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApproveAllWaitlistEntriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ApproveAllWaitlistEntriesResponseBuilder::account_id)
    /// - [`queued`](ApproveAllWaitlistEntriesResponseBuilder::queued)
    pub fn build(self) -> Result<ApproveAllWaitlistEntriesResponse, BuildError> {
        Ok(ApproveAllWaitlistEntriesResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            queued: self
                .queued
                .ok_or_else(|| BuildError::missing_field("queued"))?,
            variant_id: self.variant_id,
        })
    }
}
