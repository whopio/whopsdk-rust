pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAdGroupsRequest {
    /// The ad campaign to create the ad group in, prefixed `adcamp_`.
    #[serde(default)]
    pub ad_campaign_id: String,
    /// Saved audiences to deliver to or exclude. Can't be combined with demographics.automatic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<AdGroupAudiencesBody>,
    /// How delivery bids are set in the ad auction. Target-based strategies use `desired_cost_per_result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<CreateAdGroupsRequestBidType>,
    /// This ad group's budget in USD, which is what it is stored and billed in. Omit when the budget is set on the campaign instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// This ad group's budget stated in the account's ads reporting currency (`budget_currency` on the response) instead of USD. Converted to USD at the current exchange rate and stored as budget_amount; on an update, an amount equal to the current budget_amount_local keeps the stored USD budget as is. Provide this or budget_amount, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount_local: Option<f64>,
    /// Whether budget_amount is spent per day (`daily`) or over the ad group's full run (`lifetime`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<CreateAdGroupsRequestBudgetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_event: Option<ConversionEvent>,
    /// Hours the ad group delivers in each week, keyed by day. Each window runs from `start` to `end` on the hour, with `24:00` for midnight, and a day's windows can't overlap or touch. A day that's empty or left out doesn't deliver. Replaces the whole schedule; `null` delivers at every hour. Some platforms need a lifetime `budget_type` for a schedule, on the ad group or on its campaign when the campaign holds the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_schedule: Option<CreateAdGroupsRequestDeliverySchedule>,
    /// Age, gender, and automatic-audience targeting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographics: Option<AdGroupDemographicsBody>,
    /// Cost per result to aim for (`average_target`) or never exceed (`maximum_target`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub desired_cost_per_result: Option<f64>,
    /// Device platforms and operating systems to target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<AdGroupDevicesBody>,
    /// When the ad group stops delivering, as an ISO 8601 timestamp. Omit to run until paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Search terms the ad group's ads can show for, and terms they never show for. Only search campaigns take keywords, and each of their ad groups needs at least one before launch. Replaces the stored list; omit to keep it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<CreateAdGroupsRequestKeywordsItem>>,
    /// Languages to target, as ISO 639 codes such as `en` or `es`. Empty or omitted targets all languages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// Locations to target and exclude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<AdGroupRegionsBody>,
    /// When the ad group starts delivering, as an ISO 8601 timestamp. Omit to start as soon as it's active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Initial status (default: `active`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateAdGroupsRequestStatus>,
    /// The display name of the ad group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateAdGroupsRequest {
    pub fn builder() -> CreateAdGroupsRequestBuilder {
        <CreateAdGroupsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdGroupsRequestBuilder {
    ad_campaign_id: Option<String>,
    audiences: Option<AdGroupAudiencesBody>,
    bid_type: Option<CreateAdGroupsRequestBidType>,
    budget_amount: Option<f64>,
    budget_amount_local: Option<f64>,
    budget_type: Option<CreateAdGroupsRequestBudgetType>,
    conversion_event: Option<ConversionEvent>,
    delivery_schedule: Option<CreateAdGroupsRequestDeliverySchedule>,
    demographics: Option<AdGroupDemographicsBody>,
    desired_cost_per_result: Option<f64>,
    devices: Option<AdGroupDevicesBody>,
    ends_at: Option<String>,
    keywords: Option<Vec<CreateAdGroupsRequestKeywordsItem>>,
    languages: Option<Vec<String>>,
    regions: Option<AdGroupRegionsBody>,
    starts_at: Option<String>,
    status: Option<CreateAdGroupsRequestStatus>,
    title: Option<String>,
}

impl CreateAdGroupsRequestBuilder {
    pub fn ad_campaign_id(mut self, value: impl Into<String>) -> Self {
        self.ad_campaign_id = Some(value.into());
        self
    }

    pub fn audiences(mut self, value: AdGroupAudiencesBody) -> Self {
        self.audiences = Some(value);
        self
    }

    pub fn bid_type(mut self, value: CreateAdGroupsRequestBidType) -> Self {
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

    pub fn budget_type(mut self, value: CreateAdGroupsRequestBudgetType) -> Self {
        self.budget_type = Some(value);
        self
    }

    pub fn conversion_event(mut self, value: ConversionEvent) -> Self {
        self.conversion_event = Some(value);
        self
    }

    pub fn delivery_schedule(mut self, value: CreateAdGroupsRequestDeliverySchedule) -> Self {
        self.delivery_schedule = Some(value);
        self
    }

    pub fn demographics(mut self, value: AdGroupDemographicsBody) -> Self {
        self.demographics = Some(value);
        self
    }

    pub fn desired_cost_per_result(mut self, value: f64) -> Self {
        self.desired_cost_per_result = Some(value);
        self
    }

    pub fn devices(mut self, value: AdGroupDevicesBody) -> Self {
        self.devices = Some(value);
        self
    }

    pub fn ends_at(mut self, value: impl Into<String>) -> Self {
        self.ends_at = Some(value.into());
        self
    }

    pub fn keywords(mut self, value: Vec<CreateAdGroupsRequestKeywordsItem>) -> Self {
        self.keywords = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn regions(mut self, value: AdGroupRegionsBody) -> Self {
        self.regions = Some(value);
        self
    }

    pub fn starts_at(mut self, value: impl Into<String>) -> Self {
        self.starts_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateAdGroupsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdGroupsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_campaign_id`](CreateAdGroupsRequestBuilder::ad_campaign_id)
    pub fn build(self) -> Result<CreateAdGroupsRequest, BuildError> {
        Ok(CreateAdGroupsRequest {
            ad_campaign_id: self
                .ad_campaign_id
                .ok_or_else(|| BuildError::missing_field("ad_campaign_id"))?,
            audiences: self.audiences,
            bid_type: self.bid_type,
            budget_amount: self.budget_amount,
            budget_amount_local: self.budget_amount_local,
            budget_type: self.budget_type,
            conversion_event: self.conversion_event,
            delivery_schedule: self.delivery_schedule,
            demographics: self.demographics,
            desired_cost_per_result: self.desired_cost_per_result,
            devices: self.devices,
            ends_at: self.ends_at,
            keywords: self.keywords,
            languages: self.languages,
            regions: self.regions,
            starts_at: self.starts_at,
            status: self.status,
            title: self.title,
        })
    }
}
