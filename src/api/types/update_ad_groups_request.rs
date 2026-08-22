pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAdGroupsRequest {
    /// Saved audiences to deliver to or exclude. Can't be combined with demographics.automatic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audiences: Option<AdGroupAudiencesBody>,
    /// How delivery bids are set in the ad auction. Target-based strategies use `desired_cost_per_result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<UpdateAdGroupsRequestBidType>,
    /// This ad group's budget, in the ad account's currency. Omit when the budget is set on the campaign instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// Whether budget_amount is spent per day (`daily`) or over the ad group's full run (`lifetime`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<UpdateAdGroupsRequestBudgetType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_event: Option<ConversionEvent>,
    /// Where the outcome being optimized for occurs, such as a website visit, social-profile visit, messaging conversation, ad interaction, or lead-form submission. The lead form itself is set on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_location: Option<UpdateAdGroupsRequestConversionLocation>,
    /// Age, gender, and automatic-audience targeting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographics: Option<AdGroupDemographicsBody>,
    /// Cost per result to aim for (`average_target`) or never exceed (`maximum_target`).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub desired_cost_per_result: Option<f64>,
    /// Interest, behavior, and demographic targeting, using categories from the ad platform's targeting taxonomy. Entries across interests, behaviors, and demographics are OR'd together (anyone matching any entry is reached), matching Ads Manager's detailed-targeting box. At most 100 entries per section. Can't be combined with demographics.automatic, and unavailable to campaigns with special_ad_categories. Send the complete intended state — a section you omit is cleared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_targeting: Option<AdGroupDetailedTargetingBody>,
    /// Device platforms and operating systems to target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<AdGroupDevicesBody>,
    /// When the ad group stops delivering, as an ISO 8601 timestamp. Omit to run until paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Cap on how often one person sees ads from this ad group. Only available on campaigns with the `awareness` objective.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_cap: Option<UpdateAdGroupsRequestFrequencyCap>,
    /// Languages to target, as ISO 639 codes such as `en` or `es`. Empty or omitted targets all languages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// Apps the conversation opens in. Required when setting `conversion_location` to `messaging`, and rejected unless the ad group's conversion location is `messaging`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_apps: Option<Vec<UpdateAdGroupsRequestMessageAppsItem>>,
    /// Minimum the ad group tries to spend each day.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub minimum_daily_spend: Option<f64>,
    /// The result the ad group's delivery is optimized to get the most of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_goal: Option<UpdateAdGroupsRequestOptimizationGoal>,
    /// `automatic` to let the ad platform choose placements, or the list of platforms and positions to target. Omit a platform's positions to target all of them.
    ///
    /// Valid positions per platform:
    ///
    /// - `facebook`: `feed`, `right_hand_column`, `marketplace`, `search`, `profile_feed`, `notification`, `story`, `instream_video`, `facebook_reels`, `facebook_reels_overlay`, `biz_disco_feed`
    /// - `instagram`: `stream`, `story`, `explore`, `explore_home`, `reels`, `profile_feed`, `profile_reels`, `ig_search`
    /// - `messenger`: `story`
    /// - `audience_network`: `classic`, `rewarded_video`
    /// - `threads`: `threads_stream`
    /// - `whatsapp`: `status`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placements: Option<UpdateAdGroupsRequestPlacements>,
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
    budget_type: Option<UpdateAdGroupsRequestBudgetType>,
    conversion_event: Option<ConversionEvent>,
    conversion_location: Option<UpdateAdGroupsRequestConversionLocation>,
    demographics: Option<AdGroupDemographicsBody>,
    desired_cost_per_result: Option<f64>,
    detailed_targeting: Option<AdGroupDetailedTargetingBody>,
    devices: Option<AdGroupDevicesBody>,
    ends_at: Option<String>,
    frequency_cap: Option<UpdateAdGroupsRequestFrequencyCap>,
    languages: Option<Vec<String>>,
    message_apps: Option<Vec<UpdateAdGroupsRequestMessageAppsItem>>,
    minimum_daily_spend: Option<f64>,
    optimization_goal: Option<UpdateAdGroupsRequestOptimizationGoal>,
    placements: Option<UpdateAdGroupsRequestPlacements>,
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

    pub fn budget_type(mut self, value: UpdateAdGroupsRequestBudgetType) -> Self {
        self.budget_type = Some(value);
        self
    }

    pub fn conversion_event(mut self, value: ConversionEvent) -> Self {
        self.conversion_event = Some(value);
        self
    }

    pub fn conversion_location(mut self, value: UpdateAdGroupsRequestConversionLocation) -> Self {
        self.conversion_location = Some(value);
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

    pub fn detailed_targeting(mut self, value: AdGroupDetailedTargetingBody) -> Self {
        self.detailed_targeting = Some(value);
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

    pub fn frequency_cap(mut self, value: UpdateAdGroupsRequestFrequencyCap) -> Self {
        self.frequency_cap = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn message_apps(mut self, value: Vec<UpdateAdGroupsRequestMessageAppsItem>) -> Self {
        self.message_apps = Some(value);
        self
    }

    pub fn minimum_daily_spend(mut self, value: f64) -> Self {
        self.minimum_daily_spend = Some(value);
        self
    }

    pub fn optimization_goal(mut self, value: UpdateAdGroupsRequestOptimizationGoal) -> Self {
        self.optimization_goal = Some(value);
        self
    }

    pub fn placements(mut self, value: UpdateAdGroupsRequestPlacements) -> Self {
        self.placements = Some(value);
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
            budget_type: self.budget_type,
            conversion_event: self.conversion_event,
            conversion_location: self.conversion_location,
            demographics: self.demographics,
            desired_cost_per_result: self.desired_cost_per_result,
            detailed_targeting: self.detailed_targeting,
            devices: self.devices,
            ends_at: self.ends_at,
            frequency_cap: self.frequency_cap,
            languages: self.languages,
            message_apps: self.message_apps,
            minimum_daily_spend: self.minimum_daily_spend,
            optimization_goal: self.optimization_goal,
            placements: self.placements,
            regions: self.regions,
            starts_at: self.starts_at,
            status: self.status,
            title: self.title,
        })
    }
}
