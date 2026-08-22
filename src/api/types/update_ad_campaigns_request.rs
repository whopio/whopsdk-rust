pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAdCampaignsRequest {
    /// How delivery bids in the ad auction: `minimum_cost` gets the most results for the budget, `average_target` holds an average cost per result, `maximum_target` never bids above a cap. Switching to `minimum_cost` clears the cap amounts stored on the campaign's ad groups. Only for campaigns that own the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<UpdateAdCampaignsRequestBidType>,
    /// The campaign budget, in the account's currency. Interpreted as daily or lifetime per the campaign's existing budget type.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`). Only changeable before the campaign is live on the ad network; switching to `ad_campaign` requires budget_amount in the same request, and switching to `ad_group` clears the campaign budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_optimization: Option<UpdateAdCampaignsRequestBudgetOptimization>,
    /// When the campaign stops delivering, as an ISO 8601 timestamp. Only for campaigns that own the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Regulated categories the campaign falls under. Editable on any campaign, draft or launched; pass an empty array to clear.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_ad_categories: Option<Vec<UpdateAdCampaignsRequestSpecialAdCategoriesItem>>,
    /// When the campaign starts delivering, as an ISO 8601 timestamp. Only for campaigns that own the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Set to active to launch a draft campaign (moderates and pushes it live). Live-campaign pause and resume use the pause and unpause actions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateAdCampaignsRequestStatus>,
    /// The name of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateAdCampaignsRequest {
    pub fn builder() -> UpdateAdCampaignsRequestBuilder {
        <UpdateAdCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdCampaignsRequestBuilder {
    bid_type: Option<UpdateAdCampaignsRequestBidType>,
    budget_amount: Option<f64>,
    budget_optimization: Option<UpdateAdCampaignsRequestBudgetOptimization>,
    ends_at: Option<String>,
    special_ad_categories: Option<Vec<UpdateAdCampaignsRequestSpecialAdCategoriesItem>>,
    starts_at: Option<String>,
    status: Option<UpdateAdCampaignsRequestStatus>,
    title: Option<String>,
}

impl UpdateAdCampaignsRequestBuilder {
    pub fn bid_type(mut self, value: UpdateAdCampaignsRequestBidType) -> Self {
        self.bid_type = Some(value);
        self
    }

    pub fn budget_amount(mut self, value: f64) -> Self {
        self.budget_amount = Some(value);
        self
    }

    pub fn budget_optimization(
        mut self,
        value: UpdateAdCampaignsRequestBudgetOptimization,
    ) -> Self {
        self.budget_optimization = Some(value);
        self
    }

    pub fn ends_at(mut self, value: impl Into<String>) -> Self {
        self.ends_at = Some(value.into());
        self
    }

    pub fn special_ad_categories(
        mut self,
        value: Vec<UpdateAdCampaignsRequestSpecialAdCategoriesItem>,
    ) -> Self {
        self.special_ad_categories = Some(value);
        self
    }

    pub fn starts_at(mut self, value: impl Into<String>) -> Self {
        self.starts_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: UpdateAdCampaignsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdCampaignsRequest`].
    pub fn build(self) -> Result<UpdateAdCampaignsRequest, BuildError> {
        Ok(UpdateAdCampaignsRequest {
            bid_type: self.bid_type,
            budget_amount: self.budget_amount,
            budget_optimization: self.budget_optimization,
            ends_at: self.ends_at,
            special_ad_categories: self.special_ad_categories,
            starts_at: self.starts_at,
            status: self.status,
            title: self.title,
        })
    }
}
