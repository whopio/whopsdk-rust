pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAdGroupsRequest {
    /// Saved audiences to deliver to or exclude. Can't be combined with demographics.automatic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<AdGroupAudiencesBody>,
    /// How delivery bids are set in the ad auction. Target-based strategies use `desired_cost_per_result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<UpdateAdGroupsRequestBidType>,
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
    pub budget_type: Option<UpdateAdGroupsRequestBudgetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_event: Option<ConversionEvent>,
    /// Hours the ad group delivers in each week, keyed by day. Each window runs from `start` to `end` on the hour, with `24:00` for midnight, and a day's windows can't overlap or touch. A day that's empty or left out doesn't deliver. Replaces the whole schedule; `null` delivers at every hour. Some platforms need a lifetime `budget_type` for a schedule, on the ad group or on its campaign when the campaign holds the budget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_schedule: Option<UpdateAdGroupsRequestDeliverySchedule>,
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
    pub keywords: Option<Vec<UpdateAdGroupsRequestKeywordsItem>>,
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
    pub status: Option<UpdateAdGroupsRequestStatus>,
    /// The display name of the ad group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateAdGroupsRequest {
    pub fn builder() -> UpdateAdGroupsRequestBuilder {
        <UpdateAdGroupsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdGroupsRequestBuilder {
    audiences: Option<AdGroupAudiencesBody>,
    bid_type: Option<UpdateAdGroupsRequestBidType>,
    budget_amount: Option<f64>,
    budget_amount_local: Option<f64>,
    budget_type: Option<UpdateAdGroupsRequestBudgetType>,
    conversion_event: Option<ConversionEvent>,
    delivery_schedule: Option<UpdateAdGroupsRequestDeliverySchedule>,
    demographics: Option<AdGroupDemographicsBody>,
    desired_cost_per_result: Option<f64>,
    devices: Option<AdGroupDevicesBody>,
    ends_at: Option<String>,
    keywords: Option<Vec<UpdateAdGroupsRequestKeywordsItem>>,
    languages: Option<Vec<String>>,
    regions: Option<AdGroupRegionsBody>,
    starts_at: Option<String>,
    status: Option<UpdateAdGroupsRequestStatus>,
    title: Option<String>,
}

impl UpdateAdGroupsRequestBuilder {
    pub fn audiences(mut self, value: AdGroupAudiencesBody) -> Self {
        self.audiences = Some(value);
        self
    }

    pub fn bid_type(mut self, value: UpdateAdGroupsRequestBidType) -> Self {
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

    pub fn budget_type(mut self, value: UpdateAdGroupsRequestBudgetType) -> Self {
        self.budget_type = Some(value);
        self
    }

    pub fn conversion_event(mut self, value: ConversionEvent) -> Self {
        self.conversion_event = Some(value);
        self
    }

    pub fn delivery_schedule(mut self, value: UpdateAdGroupsRequestDeliverySchedule) -> Self {
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

    pub fn keywords(mut self, value: Vec<UpdateAdGroupsRequestKeywordsItem>) -> Self {
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

    pub fn status(mut self, value: UpdateAdGroupsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdGroupsRequest`].
    pub fn build(self) -> Result<UpdateAdGroupsRequest, BuildError> {
        Ok(UpdateAdGroupsRequest {
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
