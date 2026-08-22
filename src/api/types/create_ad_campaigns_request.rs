pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAdCampaignsRequest {
    /// The account to create the campaign under. Defaults to the account-scoped key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// How delivery bids in the ad auction: `minimum_cost` gets the most results for the budget, `average_target` holds an average cost per result, `maximum_target` never bids above a cap. Only for campaigns that own the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<CreateAdCampaignsRequestBidType>,
    /// The campaign's budget, in the ad account's currency. Required when budget_optimization is `ad_campaign`; omit when each ad group sets its own budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`). Defaults to `ad_group`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_optimization: Option<CreateAdCampaignsRequestBudgetOptimization>,
    /// Whether the budget is spent per day (`daily`) or over the campaign's full run (`lifetime`). Defaults to `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<CreateAdCampaignsRequestBudgetType>,
    /// Cost per result to aim for (`average_target`) or never exceed (`maximum_target`). Only for campaigns that own the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub desired_cost_per_result: Option<f64>,
    /// When the campaign stops delivering, as an ISO 8601 timestamp. Only for campaigns that own the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// The goal the campaign optimizes toward.
    pub objective: CreateAdCampaignsRequestObjective,
    /// The ad network the campaign runs on.
    pub platform: CreateAdCampaignsRequestPlatform,
    /// Regulated categories the campaign falls under. Ads in these categories are subject to extra targeting restrictions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_ad_categories: Option<Vec<CreateAdCampaignsRequestSpecialAdCategoriesItem>>,
    /// When the campaign starts delivering, as an ISO 8601 timestamp. Only for campaigns that own the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// The title of the campaign.
    #[serde(default)]
    pub title: String,
}

impl CreateAdCampaignsRequest {
    pub fn builder() -> CreateAdCampaignsRequestBuilder {
        <CreateAdCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdCampaignsRequestBuilder {
    account_id: Option<String>,
    bid_type: Option<CreateAdCampaignsRequestBidType>,
    budget_amount: Option<f64>,
    budget_optimization: Option<CreateAdCampaignsRequestBudgetOptimization>,
    budget_type: Option<CreateAdCampaignsRequestBudgetType>,
    desired_cost_per_result: Option<f64>,
    ends_at: Option<String>,
    objective: Option<CreateAdCampaignsRequestObjective>,
    platform: Option<CreateAdCampaignsRequestPlatform>,
    special_ad_categories: Option<Vec<CreateAdCampaignsRequestSpecialAdCategoriesItem>>,
    starts_at: Option<String>,
    title: Option<String>,
}

impl CreateAdCampaignsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn bid_type(mut self, value: CreateAdCampaignsRequestBidType) -> Self {
        self.bid_type = Some(value);
        self
    }

    pub fn budget_amount(mut self, value: f64) -> Self {
        self.budget_amount = Some(value);
        self
    }

    pub fn budget_optimization(
        mut self,
        value: CreateAdCampaignsRequestBudgetOptimization,
    ) -> Self {
        self.budget_optimization = Some(value);
        self
    }

    pub fn budget_type(mut self, value: CreateAdCampaignsRequestBudgetType) -> Self {
        self.budget_type = Some(value);
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

    pub fn objective(mut self, value: CreateAdCampaignsRequestObjective) -> Self {
        self.objective = Some(value);
        self
    }

    pub fn platform(mut self, value: CreateAdCampaignsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn special_ad_categories(
        mut self,
        value: Vec<CreateAdCampaignsRequestSpecialAdCategoriesItem>,
    ) -> Self {
        self.special_ad_categories = Some(value);
        self
    }

    pub fn starts_at(mut self, value: impl Into<String>) -> Self {
        self.starts_at = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdCampaignsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`objective`](CreateAdCampaignsRequestBuilder::objective)
    /// - [`platform`](CreateAdCampaignsRequestBuilder::platform)
    /// - [`title`](CreateAdCampaignsRequestBuilder::title)
    pub fn build(self) -> Result<CreateAdCampaignsRequest, BuildError> {
        Ok(CreateAdCampaignsRequest {
            account_id: self.account_id,
            bid_type: self.bid_type,
            budget_amount: self.budget_amount,
            budget_optimization: self.budget_optimization,
            budget_type: self.budget_type,
            desired_cost_per_result: self.desired_cost_per_result,
            ends_at: self.ends_at,
            objective: self
                .objective
                .ok_or_else(|| BuildError::missing_field("objective"))?,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            special_ad_categories: self.special_ad_categories,
            starts_at: self.starts_at,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
