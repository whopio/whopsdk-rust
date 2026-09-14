pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateEconomicIntelligenceRequest {
    /// The status to move the recommendation to. Only `superseded` is accepted.
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
    status: Option<UpdateEconomicIntelligenceRequestStatus>,
    account_id: Option<String>,
}

impl UpdateEconomicIntelligenceRequestBuilder {
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
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            account_id: self.account_id,
        })
    }
}
