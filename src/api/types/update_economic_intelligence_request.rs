pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEconomicIntelligenceRequest {
    /// No longer read: recommendations are written for the business as it changes. Up to 1000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// With `status: executed`, the ID of what the run produced or changed, and the recommendation's `result_url` becomes where to view it: an ad (`ad_`), ad group (`adgrp_`) or ad campaign (`adcamp_`), a website (`app_`), a product (`prod_`), a plan (`plan_`), a checkout link (`ch_`), a promo code (`promo_`), or an experience (`exp_`). Without `result_id` or `result_page`, `result_url` links to the resource the recommendation was about, when it names one. Send only one of `result_id`, `result_page` and `result_url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_id: Option<String>,
    /// With `status: executed`, the page where the run's result can be seen when it is not one resource, such as the checkout links list. The recommendation's `result_url` becomes that page on the account's dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_page: Option<UpdateEconomicIntelligenceRequestResultPage>,
    /// With `status: executed`, where to view what was produced when it is outside Whop. An http or https URL. Prefer `result_id` for anything on Whop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_url: Option<String>,
    /// A signed-in user can rate a recommendation as `positive` or `negative`. Can be sent alone or together with status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<UpdateEconomicIntelligenceRequestSentiment>,
    /// Use `running` to start a run of a ready recommendation, `executed` to record that it was carried out, `incomplete` to record that a running recommendation's run ended without carrying it out, `superseded` to reject a ready recommendation, or `acknowledged` to mark an executed run as seen; the recommendation stays `executed` and records `acknowledged_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateEconomicIntelligenceRequestStatus>,
    /// An optional explanation of the rating or rejection.
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
    input: Option<String>,
    result_id: Option<String>,
    result_page: Option<UpdateEconomicIntelligenceRequestResultPage>,
    result_url: Option<String>,
    sentiment: Option<UpdateEconomicIntelligenceRequestSentiment>,
    status: Option<UpdateEconomicIntelligenceRequestStatus>,
    user_feedback: Option<String>,
    account_id: Option<String>,
}

impl UpdateEconomicIntelligenceRequestBuilder {
    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input = Some(value.into());
        self
    }

    pub fn result_id(mut self, value: impl Into<String>) -> Self {
        self.result_id = Some(value.into());
        self
    }

    pub fn result_page(mut self, value: UpdateEconomicIntelligenceRequestResultPage) -> Self {
        self.result_page = Some(value);
        self
    }

    pub fn result_url(mut self, value: impl Into<String>) -> Self {
        self.result_url = Some(value.into());
        self
    }

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
            input: self.input,
            result_id: self.result_id,
            result_page: self.result_page,
            result_url: self.result_url,
            sentiment: self.sentiment,
            status: self.status,
            user_feedback: self.user_feedback,
            account_id: self.account_id,
        })
    }
}
