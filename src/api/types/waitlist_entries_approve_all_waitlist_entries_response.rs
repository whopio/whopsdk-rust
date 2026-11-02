pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApproveAllWaitlistEntriesResponse {
    /// The seller account whose signups were queued, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// The plan the request was narrowed to, prefixed `plan_`, or `null` when every waitlist plan on the account was included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Whether any pending signups were queued for approval. `false` when there were none.
    #[serde(default)]
    pub queued: bool,
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
    plan_id: Option<String>,
    queued: Option<bool>,
}

impl ApproveAllWaitlistEntriesResponseBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn queued(mut self, value: bool) -> Self {
        self.queued = Some(value);
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
            plan_id: self.plan_id,
            queued: self
                .queued
                .ok_or_else(|| BuildError::missing_field("queued"))?,
        })
    }
}
