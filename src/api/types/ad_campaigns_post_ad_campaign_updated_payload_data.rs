pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostAdCampaignUpdatedPayloadData {
    /// The business running this campaign, with the name and logo its ads show.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<StorefrontAccount>,
    /// The campaign's budget in USD, which is what it is stored and billed in. `null` when each ad group sets its own budget instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// The same budget stated in `budget_currency` at today's exchange rate, for display in the account's ads reporting currency. `null` when `budget_amount` is.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount_local: Option<f64>,
    /// The ISO 4217 code `budget_amount_local` is in: the account's `ads_reporting_currency` preference. `usd` unless the account changed it.
    #[serde(default)]
    pub budget_currency: String,
    /// Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_optimization: Option<PostAdCampaignUpdatedPayloadDataBudgetOptimization>,
    /// Whether `budget_amount` is spent per day (`daily`) or over the campaign's full run (`lifetime`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<PostAdCampaignUpdatedPayloadDataBudgetType>,
    /// The kind of campaign, fixed at creation. `standard` runs image and video ads; `search` shows text ads on search results for the ad groups' keywords.
    pub campaign_type: PostAdCampaignUpdatedPayloadDataCampaignType,
    /// When the campaign was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Whether the campaign's ads are delivering right now, and if not, why. Content review is in_review; a campaign waiting to start is processing. Account billing failures set payment_failed without changing the configured status. Successful payment retry clears that block and recalculates delivery. When several states apply at once, the highest-precedence one is returned.
    pub delivery_status: PostAdCampaignUpdatedPayloadDataDeliveryStatus,
    /// The cost per result, in USD, the advertiser wants this campaign to achieve. A target only: it is not a bid, cost cap, or bid cap, and doesn't change how the campaign's ads bid. `null` when no target is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_cost_per_result: Option<f64>,
    /// When the campaign stops delivering, as an ISO 8601 timestamp. `null` when it runs until paused, or when its ad groups set their own schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Unique identifier for the ad campaign, prefixed `adcamp_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub issues: Vec<AdPlatformIssue>,
    /// The goal the campaign optimizes toward. `awareness` appears only on campaigns created before it was retired; those campaigns can't launch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective: Option<PostAdCampaignUpdatedPayloadDataObjective>,
    #[serde(default)]
    pub special_ad_categories: Vec<PostAdCampaignUpdatedPayloadDataSpecialAdCategoriesItem>,
    /// When the campaign starts delivering, as an ISO 8601 timestamp. `null` when it starts as soon as it launches, or when its ad groups set their own schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// The configured lifecycle status of the ad campaign. Billing failures preserve active or paused here and set delivery_status to payment_failed.
    pub status: PostAdCampaignUpdatedPayloadDataStatus,
    /// The desired cost per result for `target_result_event`, in USD. `null` when none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub target_cost_per_result: Option<f64>,
    /// The event `target_cost_per_result` is priced per, such as `purchase` or `lead`. `null` when the campaign has no single goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_result_event: Option<String>,
    /// Display name of the ad campaign.
    #[serde(default)]
    pub title: String,
    /// When the campaign was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl PostAdCampaignUpdatedPayloadData {
    pub fn builder() -> PostAdCampaignUpdatedPayloadDataBuilder {
        <PostAdCampaignUpdatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostAdCampaignUpdatedPayloadDataBuilder {
    account: Option<StorefrontAccount>,
    budget_amount: Option<f64>,
    budget_amount_local: Option<f64>,
    budget_currency: Option<String>,
    budget_optimization: Option<PostAdCampaignUpdatedPayloadDataBudgetOptimization>,
    budget_type: Option<PostAdCampaignUpdatedPayloadDataBudgetType>,
    campaign_type: Option<PostAdCampaignUpdatedPayloadDataCampaignType>,
    created_at: Option<String>,
    delivery_status: Option<PostAdCampaignUpdatedPayloadDataDeliveryStatus>,
    desired_cost_per_result: Option<f64>,
    ends_at: Option<String>,
    id: Option<String>,
    issues: Option<Vec<AdPlatformIssue>>,
    objective: Option<PostAdCampaignUpdatedPayloadDataObjective>,
    special_ad_categories: Option<Vec<PostAdCampaignUpdatedPayloadDataSpecialAdCategoriesItem>>,
    starts_at: Option<String>,
    status: Option<PostAdCampaignUpdatedPayloadDataStatus>,
    target_cost_per_result: Option<f64>,
    target_result_event: Option<String>,
    title: Option<String>,
    updated_at: Option<String>,
}

impl PostAdCampaignUpdatedPayloadDataBuilder {
    pub fn account(mut self, value: StorefrontAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn budget_amount(mut self, value: f64) -> Self {
        self.budget_amount = Some(value);
        self
    }

    pub fn budget_amount_local(mut self, value: f64) -> Self {
        self.budget_amount_local = Some(value);
        self
    }

    pub fn budget_currency(mut self, value: impl Into<String>) -> Self {
        self.budget_currency = Some(value.into());
        self
    }

    pub fn budget_optimization(
        mut self,
        value: PostAdCampaignUpdatedPayloadDataBudgetOptimization,
    ) -> Self {
        self.budget_optimization = Some(value);
        self
    }

    pub fn budget_type(mut self, value: PostAdCampaignUpdatedPayloadDataBudgetType) -> Self {
        self.budget_type = Some(value);
        self
    }

    pub fn campaign_type(mut self, value: PostAdCampaignUpdatedPayloadDataCampaignType) -> Self {
        self.campaign_type = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn delivery_status(
        mut self,
        value: PostAdCampaignUpdatedPayloadDataDeliveryStatus,
    ) -> Self {
        self.delivery_status = Some(value);
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn issues(mut self, value: Vec<AdPlatformIssue>) -> Self {
        self.issues = Some(value);
        self
    }

    pub fn objective(mut self, value: PostAdCampaignUpdatedPayloadDataObjective) -> Self {
        self.objective = Some(value);
        self
    }

    pub fn special_ad_categories(
        mut self,
        value: Vec<PostAdCampaignUpdatedPayloadDataSpecialAdCategoriesItem>,
    ) -> Self {
        self.special_ad_categories = Some(value);
        self
    }

    pub fn starts_at(mut self, value: impl Into<String>) -> Self {
        self.starts_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostAdCampaignUpdatedPayloadDataStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn target_cost_per_result(mut self, value: f64) -> Self {
        self.target_cost_per_result = Some(value);
        self
    }

    pub fn target_result_event(mut self, value: impl Into<String>) -> Self {
        self.target_result_event = Some(value.into());
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

    /// Consumes the builder and constructs a [`PostAdCampaignUpdatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`budget_currency`](PostAdCampaignUpdatedPayloadDataBuilder::budget_currency)
    /// - [`campaign_type`](PostAdCampaignUpdatedPayloadDataBuilder::campaign_type)
    /// - [`created_at`](PostAdCampaignUpdatedPayloadDataBuilder::created_at)
    /// - [`delivery_status`](PostAdCampaignUpdatedPayloadDataBuilder::delivery_status)
    /// - [`id`](PostAdCampaignUpdatedPayloadDataBuilder::id)
    /// - [`issues`](PostAdCampaignUpdatedPayloadDataBuilder::issues)
    /// - [`special_ad_categories`](PostAdCampaignUpdatedPayloadDataBuilder::special_ad_categories)
    /// - [`status`](PostAdCampaignUpdatedPayloadDataBuilder::status)
    /// - [`title`](PostAdCampaignUpdatedPayloadDataBuilder::title)
    /// - [`updated_at`](PostAdCampaignUpdatedPayloadDataBuilder::updated_at)
    pub fn build(self) -> Result<PostAdCampaignUpdatedPayloadData, BuildError> {
        Ok(PostAdCampaignUpdatedPayloadData {
            account: self.account,
            budget_amount: self.budget_amount,
            budget_amount_local: self.budget_amount_local,
            budget_currency: self
                .budget_currency
                .ok_or_else(|| BuildError::missing_field("budget_currency"))?,
            budget_optimization: self.budget_optimization,
            budget_type: self.budget_type,
            campaign_type: self
                .campaign_type
                .ok_or_else(|| BuildError::missing_field("campaign_type"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            delivery_status: self
                .delivery_status
                .ok_or_else(|| BuildError::missing_field("delivery_status"))?,
            desired_cost_per_result: self.desired_cost_per_result,
            ends_at: self.ends_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            objective: self.objective,
            special_ad_categories: self
                .special_ad_categories
                .ok_or_else(|| BuildError::missing_field("special_ad_categories"))?,
            starts_at: self.starts_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            target_cost_per_result: self.target_cost_per_result,
            target_result_event: self.target_result_event,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
