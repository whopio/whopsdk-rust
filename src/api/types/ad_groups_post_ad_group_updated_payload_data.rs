pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostAdGroupUpdatedPayloadData {
    /// The ad campaign this ad group belongs to.
    #[serde(default)]
    pub ad_campaign: AdEntityReference,
    /// Saved audiences this ad group delivers to or excludes.
    #[serde(default)]
    pub audiences: AdGroupAudiences,
    /// How delivery bids are set in the ad auction. Target-based strategies use `desired_cost_per_result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<PostAdGroupUpdatedPayloadDataBidType>,
    /// This ad group's budget in USD, which is what it is stored and billed in. `null` when the budget is set on the campaign instead.
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
    /// Whether `budget_amount` is spent per day (`daily`) or over the ad group's full run (`lifetime`). A `lifetime` ad group also needs `ends_at`, at least 24 hours after it starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<PostAdGroupUpdatedPayloadDataBudgetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_event: Option<ConversionEvent>,
    /// When the ad group was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Hours the ad group delivers in each week, keyed by day. Days it doesn't deliver are omitted. `null` when it delivers at every hour. Some platforms need a lifetime `budget_type` for a schedule, on the ad group or on its campaign when the campaign holds the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_schedule: Option<AdGroupDeliverySchedule>,
    /// Whether ads in this ad group are delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
    pub delivery_status: PostAdGroupUpdatedPayloadDataDeliveryStatus,
    /// Age, gender, and automatic-audience targeting.
    pub demographics: AdGroupDemographics,
    /// Cost per result to aim for (`average_target`) or never exceed (`maximum_target`). `null` for `minimum_cost` bidding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub desired_cost_per_result: Option<f64>,
    /// Device platforms and operating systems targeted.
    #[serde(default)]
    pub devices: AdGroupDevices,
    /// When the ad group stops delivering, as an ISO 8601 timestamp. `null` when it runs until paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Unique identifier for the ad group, prefixed `adgrp_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub issues: Vec<AdPlatformIssue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<AdGroupKeyword>>,
    #[serde(default)]
    pub languages: Vec<String>,
    /// Progress toward the conversions this ad group's learning phase needs. Null unless it is learning or learning limited and valid counts are available. Reaching the threshold does not determine delivery status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learning_progress: Option<AdGroupLearningProgress>,
    /// Locations targeted and excluded.
    #[serde(default)]
    pub regions: AdGroupRegions,
    /// When the ad group starts delivering, as an ISO 8601 timestamp. `null` when it starts as soon as it's active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Whether the ad group is enabled. `active` and `paused` are set by you; `rejected` means it failed ad review; `duplicating` is a copy still being filled in.
    pub status: PostAdGroupUpdatedPayloadDataStatus,
    /// Display name of the ad group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// When the ad group was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl PostAdGroupUpdatedPayloadData {
    pub fn builder() -> PostAdGroupUpdatedPayloadDataBuilder {
        <PostAdGroupUpdatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostAdGroupUpdatedPayloadDataBuilder {
    ad_campaign: Option<AdEntityReference>,
    audiences: Option<AdGroupAudiences>,
    bid_type: Option<PostAdGroupUpdatedPayloadDataBidType>,
    budget_amount: Option<f64>,
    budget_amount_local: Option<f64>,
    budget_currency: Option<String>,
    budget_type: Option<PostAdGroupUpdatedPayloadDataBudgetType>,
    conversion_event: Option<ConversionEvent>,
    created_at: Option<String>,
    delivery_schedule: Option<AdGroupDeliverySchedule>,
    delivery_status: Option<PostAdGroupUpdatedPayloadDataDeliveryStatus>,
    demographics: Option<AdGroupDemographics>,
    desired_cost_per_result: Option<f64>,
    devices: Option<AdGroupDevices>,
    ends_at: Option<String>,
    id: Option<String>,
    issues: Option<Vec<AdPlatformIssue>>,
    keywords: Option<Vec<AdGroupKeyword>>,
    languages: Option<Vec<String>>,
    learning_progress: Option<AdGroupLearningProgress>,
    regions: Option<AdGroupRegions>,
    starts_at: Option<String>,
    status: Option<PostAdGroupUpdatedPayloadDataStatus>,
    title: Option<String>,
    updated_at: Option<String>,
}

impl PostAdGroupUpdatedPayloadDataBuilder {
    pub fn ad_campaign(mut self, value: AdEntityReference) -> Self {
        self.ad_campaign = Some(value);
        self
    }

    pub fn audiences(mut self, value: AdGroupAudiences) -> Self {
        self.audiences = Some(value);
        self
    }

    pub fn bid_type(mut self, value: PostAdGroupUpdatedPayloadDataBidType) -> Self {
        self.bid_type = Some(value);
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

    pub fn budget_type(mut self, value: PostAdGroupUpdatedPayloadDataBudgetType) -> Self {
        self.budget_type = Some(value);
        self
    }

    pub fn conversion_event(mut self, value: ConversionEvent) -> Self {
        self.conversion_event = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn delivery_schedule(mut self, value: AdGroupDeliverySchedule) -> Self {
        self.delivery_schedule = Some(value);
        self
    }

    pub fn delivery_status(mut self, value: PostAdGroupUpdatedPayloadDataDeliveryStatus) -> Self {
        self.delivery_status = Some(value);
        self
    }

    pub fn demographics(mut self, value: AdGroupDemographics) -> Self {
        self.demographics = Some(value);
        self
    }

    pub fn desired_cost_per_result(mut self, value: f64) -> Self {
        self.desired_cost_per_result = Some(value);
        self
    }

    pub fn devices(mut self, value: AdGroupDevices) -> Self {
        self.devices = Some(value);
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

    pub fn keywords(mut self, value: Vec<AdGroupKeyword>) -> Self {
        self.keywords = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn learning_progress(mut self, value: AdGroupLearningProgress) -> Self {
        self.learning_progress = Some(value);
        self
    }

    pub fn regions(mut self, value: AdGroupRegions) -> Self {
        self.regions = Some(value);
        self
    }

    pub fn starts_at(mut self, value: impl Into<String>) -> Self {
        self.starts_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostAdGroupUpdatedPayloadDataStatus) -> Self {
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

    /// Consumes the builder and constructs a [`PostAdGroupUpdatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_campaign`](PostAdGroupUpdatedPayloadDataBuilder::ad_campaign)
    /// - [`audiences`](PostAdGroupUpdatedPayloadDataBuilder::audiences)
    /// - [`budget_currency`](PostAdGroupUpdatedPayloadDataBuilder::budget_currency)
    /// - [`created_at`](PostAdGroupUpdatedPayloadDataBuilder::created_at)
    /// - [`delivery_status`](PostAdGroupUpdatedPayloadDataBuilder::delivery_status)
    /// - [`demographics`](PostAdGroupUpdatedPayloadDataBuilder::demographics)
    /// - [`devices`](PostAdGroupUpdatedPayloadDataBuilder::devices)
    /// - [`id`](PostAdGroupUpdatedPayloadDataBuilder::id)
    /// - [`issues`](PostAdGroupUpdatedPayloadDataBuilder::issues)
    /// - [`languages`](PostAdGroupUpdatedPayloadDataBuilder::languages)
    /// - [`regions`](PostAdGroupUpdatedPayloadDataBuilder::regions)
    /// - [`status`](PostAdGroupUpdatedPayloadDataBuilder::status)
    /// - [`updated_at`](PostAdGroupUpdatedPayloadDataBuilder::updated_at)
    pub fn build(self) -> Result<PostAdGroupUpdatedPayloadData, BuildError> {
        Ok(PostAdGroupUpdatedPayloadData {
            ad_campaign: self
                .ad_campaign
                .ok_or_else(|| BuildError::missing_field("ad_campaign"))?,
            audiences: self
                .audiences
                .ok_or_else(|| BuildError::missing_field("audiences"))?,
            bid_type: self.bid_type,
            budget_amount: self.budget_amount,
            budget_amount_local: self.budget_amount_local,
            budget_currency: self
                .budget_currency
                .ok_or_else(|| BuildError::missing_field("budget_currency"))?,
            budget_type: self.budget_type,
            conversion_event: self.conversion_event,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            delivery_schedule: self.delivery_schedule,
            delivery_status: self
                .delivery_status
                .ok_or_else(|| BuildError::missing_field("delivery_status"))?,
            demographics: self
                .demographics
                .ok_or_else(|| BuildError::missing_field("demographics"))?,
            desired_cost_per_result: self.desired_cost_per_result,
            devices: self
                .devices
                .ok_or_else(|| BuildError::missing_field("devices"))?,
            ends_at: self.ends_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            keywords: self.keywords,
            languages: self
                .languages
                .ok_or_else(|| BuildError::missing_field("languages"))?,
            learning_progress: self.learning_progress,
            regions: self
                .regions
                .ok_or_else(|| BuildError::missing_field("regions"))?,
            starts_at: self.starts_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self.title,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
