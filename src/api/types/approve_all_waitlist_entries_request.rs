pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApproveAllWaitlistEntriesRequest {
    /// The seller account whose pending signups to approve, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Only approve signups for this plan, prefixed `plan_`. Omit to include every waitlist plan on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
}

impl ApproveAllWaitlistEntriesRequest {
    pub fn builder() -> ApproveAllWaitlistEntriesRequestBuilder {
        <ApproveAllWaitlistEntriesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApproveAllWaitlistEntriesRequestBuilder {
    account_id: Option<String>,
    plan_id: Option<String>,
}

impl ApproveAllWaitlistEntriesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApproveAllWaitlistEntriesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ApproveAllWaitlistEntriesRequestBuilder::account_id)
    pub fn build(self) -> Result<ApproveAllWaitlistEntriesRequest, BuildError> {
        Ok(ApproveAllWaitlistEntriesRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            plan_id: self.plan_id,
        })
    }
}
