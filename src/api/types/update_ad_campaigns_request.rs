pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAdCampaignsRequest {
    /// The campaign budget in USD, which is what it is stored and billed in. Interpreted as daily or lifetime per the campaign's budget type, including a budget_type sent in the same request.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// The campaign budget stated in the account's ads reporting currency (`budget_currency` on the response) instead of USD. Converted to USD at the current exchange rate and stored as budget_amount; an amount equal to the current budget_amount_local keeps the stored USD budget as is. Provide this or budget_amount, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount_local: Option<f64>,
    /// Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`). Only changeable before the campaign launches; switching to `ad_campaign` requires budget_amount in the same request, and switching to `ad_group` clears the campaign budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_optimization: Option<UpdateAdCampaignsRequestBudgetOptimization>,
    /// Whether `budget_amount` is spent per day (`daily`) or over the campaign's full run (`lifetime`). Only changeable while the campaign is a draft; send budget_amount in the same request so the amount lands on the new type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<UpdateAdCampaignsRequestBudgetType>,
    /// Accepted only when it matches the campaign's current type, so a read can be sent back unchanged. The type is fixed at creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_type: Option<UpdateAdCampaignsRequestCampaignType>,
    /// The cost per result, in USD, you want this campaign to achieve. A target only: it is not a bid, cost cap, or bid cap, and doesn't change how the campaign's ads bid. Ad groups set cost and bid caps with their own `desired_cost_per_result`. `null` clears it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_cost_per_result: Option<f64>,
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
    budget_amount: Option<f64>,
    budget_amount_local: Option<f64>,
    budget_optimization: Option<UpdateAdCampaignsRequestBudgetOptimization>,
    budget_type: Option<UpdateAdCampaignsRequestBudgetType>,
    campaign_type: Option<UpdateAdCampaignsRequestCampaignType>,
    desired_cost_per_result: Option<f64>,
    ends_at: Option<String>,
    special_ad_categories: Option<Vec<UpdateAdCampaignsRequestSpecialAdCategoriesItem>>,
    starts_at: Option<String>,
    status: Option<UpdateAdCampaignsRequestStatus>,
    title: Option<String>,
}

impl UpdateAdCampaignsRequestBuilder {
    pub fn budget_amount(mut self, value: f64) -> Self {
        self.budget_amount = Some(value);
        self
    }

    pub fn budget_amount_local(mut self, value: f64) -> Self {
        self.budget_amount_local = Some(value);
        self
    }

    pub fn budget_optimization(
        mut self,
        value: UpdateAdCampaignsRequestBudgetOptimization,
    ) -> Self {
        self.budget_optimization = Some(value);
        self
    }

    pub fn budget_type(mut self, value: UpdateAdCampaignsRequestBudgetType) -> Self {
        self.budget_type = Some(value);
        self
    }

    pub fn campaign_type(mut self, value: UpdateAdCampaignsRequestCampaignType) -> Self {
        self.campaign_type = Some(value);
        self
    }

    pub fn desired_cost_per_result(mut self, value: f64) -> Self {
        self.desired_cost_per_result = Some(value);
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
            budget_amount: self.budget_amount,
            budget_amount_local: self.budget_amount_local,
            budget_optimization: self.budget_optimization,
            budget_type: self.budget_type,
            campaign_type: self.campaign_type,
            desired_cost_per_result: self.desired_cost_per_result,
            ends_at: self.ends_at,
            special_ad_categories: self.special_ad_categories,
            starts_at: self.starts_at,
            status: self.status,
            title: self.title,
        })
    }
}
