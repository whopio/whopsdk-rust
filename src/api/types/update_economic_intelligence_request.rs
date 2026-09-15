pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateEconomicIntelligenceRequest {
    /// Why the recommendation was rejected. Used as feedback when replenishing recommendations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Use `executed` after approval to start the action, or `superseded` to reject it.
    pub status: UpdateEconomicIntelligenceRequestStatus,
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account.
    #[serde(skip)]
    pub account_id: Option<String>,
}

impl UpdateEconomicIntelligenceRequest {
    pub fn builder() -> UpdateEconomicIntelligenceRequestBuilder {
        <UpdateEconomicIntelligenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEconomicIntelligenceRequestBuilder {
    reason: Option<String>,
    status: Option<UpdateEconomicIntelligenceRequestStatus>,
    account_id: Option<String>,
}

impl UpdateEconomicIntelligenceRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn status(mut self, value: UpdateEconomicIntelligenceRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEconomicIntelligenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](UpdateEconomicIntelligenceRequestBuilder::status)
    pub fn build(self) -> Result<UpdateEconomicIntelligenceRequest, BuildError> {
        Ok(UpdateEconomicIntelligenceRequest {
            reason: self.reason,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            account_id: self.account_id,
        })
    }
}
