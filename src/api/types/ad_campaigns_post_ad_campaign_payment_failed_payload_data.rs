pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostAdCampaignPaymentFailedPayloadData {
    /// How delivery bids in the ad auction: `minimum_cost` gets the most results for the budget, `average_target` holds an average cost per result, and `maximum_target` never bids above a cap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<PostAdCampaignPaymentFailedPayloadDataBidType>,
    /// The campaign's budget, in the ad account's currency. `null` when each ad group sets its own budget instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_optimization: Option<PostAdCampaignPaymentFailedPayloadDataBudgetOptimization>,
    /// Whether `budget_amount` is spent per day (`daily`) or over the campaign's full run (`lifetime`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<PostAdCampaignPaymentFailedPayloadDataBudgetType>,
    /// When the campaign was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Whether the campaign's ads are delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
    pub delivery_status: PostAdCampaignPaymentFailedPayloadDataDeliveryStatus,
    /// Unique identifier for the ad campaign, prefixed `adcamp_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub issues: Vec<AdPlatformIssue>,
    /// The goal the campaign optimizes toward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective: Option<PostAdCampaignPaymentFailedPayloadDataObjective>,
    /// The event the campaign optimizes for when a single goal is set campaign-wide. `null` when each ad group sets its own optimization_goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_goal: Option<String>,
    /// The ad network the campaign runs on.
    pub platform: PostAdCampaignPaymentFailedPayloadDataPlatform,
    #[serde(default)]
    pub special_ad_categories: Vec<PostAdCampaignPaymentFailedPayloadDataSpecialAdCategoriesItem>,
    /// The lifecycle status of the ad campaign.
    pub status: PostAdCampaignPaymentFailedPayloadDataStatus,
    /// Display name of the ad campaign.
    #[serde(default)]
    pub title: String,
    /// When the campaign was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl PostAdCampaignPaymentFailedPayloadData {
    pub fn builder() -> PostAdCampaignPaymentFailedPayloadDataBuilder {
        <PostAdCampaignPaymentFailedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostAdCampaignPaymentFailedPayloadDataBuilder {
    bid_type: Option<PostAdCampaignPaymentFailedPayloadDataBidType>,
    budget_amount: Option<f64>,
    budget_optimization: Option<PostAdCampaignPaymentFailedPayloadDataBudgetOptimization>,
    budget_type: Option<PostAdCampaignPaymentFailedPayloadDataBudgetType>,
    created_at: Option<String>,
    delivery_status: Option<PostAdCampaignPaymentFailedPayloadDataDeliveryStatus>,
    id: Option<String>,
    issues: Option<Vec<AdPlatformIssue>>,
    objective: Option<PostAdCampaignPaymentFailedPayloadDataObjective>,
    optimization_goal: Option<String>,
    platform: Option<PostAdCampaignPaymentFailedPayloadDataPlatform>,
    special_ad_categories:
        Option<Vec<PostAdCampaignPaymentFailedPayloadDataSpecialAdCategoriesItem>>,
    status: Option<PostAdCampaignPaymentFailedPayloadDataStatus>,
    title: Option<String>,
    updated_at: Option<String>,
}

impl PostAdCampaignPaymentFailedPayloadDataBuilder {
    pub fn bid_type(mut self, value: PostAdCampaignPaymentFailedPayloadDataBidType) -> Self {
        self.bid_type = Some(value);
        self
    }

    pub fn budget_amount(mut self, value: f64) -> Self {
        self.budget_amount = Some(value);
        self
    }

    pub fn budget_optimization(
        mut self,
        value: PostAdCampaignPaymentFailedPayloadDataBudgetOptimization,
    ) -> Self {
        self.budget_optimization = Some(value);
        self
    }

    pub fn budget_type(mut self, value: PostAdCampaignPaymentFailedPayloadDataBudgetType) -> Self {
        self.budget_type = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn delivery_status(
        mut self,
        value: PostAdCampaignPaymentFailedPayloadDataDeliveryStatus,
    ) -> Self {
        self.delivery_status = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn issues(mut self, value: Vec<AdPlatformIssue>) -> Self {
        self.issues = Some(value);
        self
    }

    pub fn objective(mut self, value: PostAdCampaignPaymentFailedPayloadDataObjective) -> Self {
        self.objective = Some(value);
        self
    }

    pub fn optimization_goal(mut self, value: impl Into<String>) -> Self {
        self.optimization_goal = Some(value.into());
        self
    }

    pub fn platform(mut self, value: PostAdCampaignPaymentFailedPayloadDataPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn special_ad_categories(
        mut self,
        value: Vec<PostAdCampaignPaymentFailedPayloadDataSpecialAdCategoriesItem>,
    ) -> Self {
        self.special_ad_categories = Some(value);
        self
    }

    pub fn status(mut self, value: PostAdCampaignPaymentFailedPayloadDataStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostAdCampaignPaymentFailedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PostAdCampaignPaymentFailedPayloadDataBuilder::created_at)
    /// - [`delivery_status`](PostAdCampaignPaymentFailedPayloadDataBuilder::delivery_status)
    /// - [`id`](PostAdCampaignPaymentFailedPayloadDataBuilder::id)
    /// - [`issues`](PostAdCampaignPaymentFailedPayloadDataBuilder::issues)
    /// - [`platform`](PostAdCampaignPaymentFailedPayloadDataBuilder::platform)
    /// - [`special_ad_categories`](PostAdCampaignPaymentFailedPayloadDataBuilder::special_ad_categories)
    /// - [`status`](PostAdCampaignPaymentFailedPayloadDataBuilder::status)
    /// - [`title`](PostAdCampaignPaymentFailedPayloadDataBuilder::title)
    /// - [`updated_at`](PostAdCampaignPaymentFailedPayloadDataBuilder::updated_at)
    pub fn build(self) -> Result<PostAdCampaignPaymentFailedPayloadData, BuildError> {
        Ok(PostAdCampaignPaymentFailedPayloadData {
            bid_type: self.bid_type,
            budget_amount: self.budget_amount,
            budget_optimization: self.budget_optimization,
            budget_type: self.budget_type,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            delivery_status: self
                .delivery_status
                .ok_or_else(|| BuildError::missing_field("delivery_status"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            objective: self.objective,
            optimization_goal: self.optimization_goal,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            special_ad_categories: self
                .special_ad_categories
                .ok_or_else(|| BuildError::missing_field("special_ad_categories"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
