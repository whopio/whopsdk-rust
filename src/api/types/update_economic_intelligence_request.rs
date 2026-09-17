pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEconomicIntelligenceRequest {
    /// A signed-in user can rate a recommendation as `positive` or `negative`. Can be sent alone or together with status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<UpdateEconomicIntelligenceRequestSentiment>,
    /// Use `executed` to record approval, or `superseded` to reject the recommendation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateEconomicIntelligenceRequestStatus>,
    /// An optional explanation of the rating or rejection. Negative feedback informs replacement recommendations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_feedback: Option<String>,
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
    sentiment: Option<UpdateEconomicIntelligenceRequestSentiment>,
    status: Option<UpdateEconomicIntelligenceRequestStatus>,
    user_feedback: Option<String>,
    account_id: Option<String>,
}

impl UpdateEconomicIntelligenceRequestBuilder {
    pub fn sentiment(mut self, value: UpdateEconomicIntelligenceRequestSentiment) -> Self {
        self.sentiment = Some(value);
        self
    }

    pub fn status(mut self, value: UpdateEconomicIntelligenceRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user_feedback(mut self, value: impl Into<String>) -> Self {
        self.user_feedback = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEconomicIntelligenceRequest`].
    pub fn build(self) -> Result<UpdateEconomicIntelligenceRequest, BuildError> {
        Ok(UpdateEconomicIntelligenceRequest {
            sentiment: self.sentiment,
            status: self.status,
            user_feedback: self.user_feedback,
            account_id: self.account_id,
        })
    }
}
