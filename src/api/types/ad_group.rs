pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdGroup {
    /// The ad campaign this ad group belongs to.
    #[serde(default)]
    pub ad_campaign: AdEntityReference,
    /// USD value attributed to add-to-cart events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub added_to_cart_value: f64,
    /// Whop pixel-attributed add-to-cart events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub added_to_carts: f64,
    /// Saved audiences this ad group delivers to or excludes.
    #[serde(default)]
    pub audiences: AdGroupAudiences,
    /// How delivery bids are set in the ad auction. Target-based strategies use `desired_cost_per_result`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<AdGroupBidType>,
    /// This ad group's budget, in the ad account's currency. `null` when the budget is set on the campaign instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// Whether `budget_amount` is spent per day (`daily`) or over the ad group's full run (`lifetime`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<AdGroupBudgetType>,
    /// Clicks divided by impressions, between 0 and 1.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub click_through_rate: f64,
    /// The number of clicks.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub clicks: f64,
    /// USD value attributed to complete-registration events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub completed_registration_value: f64,
    /// Whop pixel-attributed complete-registration events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub completed_registrations: f64,
    /// USD value attributed to contact events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub contact_value: f64,
    /// Whop pixel-attributed contact events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub contacts: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_event: Option<ConversionEvent>,
    /// Where the outcome being optimized for occurs, such as a website visit, social-profile visit, messaging conversation, ad interaction, or lead-form submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_location: Option<AdGroupConversionLocation>,
    /// Spend divided by attributed add-to-cart events; null when they are not the goal and none are attributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_added_to_cart: Option<f64>,
    /// Spend divided by clicks; 0 when there are no clicks.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub cost_per_click: f64,
    /// Spend divided by attributed complete-registration events; null when they are not the goal and none are attributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_completed_registration: Option<f64>,
    /// Spend divided by attributed contact events; null when contacts are not the goal and none are attributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_contact: Option<f64>,
    /// Spend divided by attributed leads; null when leads are not a goal and none are attributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_lead: Option<f64>,
    /// Spend per 1,000 impressions; 0 when there are no impressions.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub cost_per_mille: f64,
    /// Spend divided by attributed purchases; null when purchases are not a goal and none are attributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_purchase: Option<f64>,
    /// Spend divided by Whop pixel-attributed results; null when nothing Whop-attributable is being optimized for.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_result: Option<f64>,
    /// Spend divided by attributed schedule events; null when schedules are not the goal and none are attributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_schedule: Option<f64>,
    /// Spend divided by attributed submit-application events; null when they are not the goal and none are attributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_submitted_application: Option<f64>,
    /// Spend divided by unique clicks; null when there are no unique clicks.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_unique_click: Option<f64>,
    /// Spend divided by attributed view-content events; null when they are not the goal and none are attributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cost_per_viewed_content: Option<f64>,
    /// When the ad group was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Whop pixel-attributed custom (merchant-defined) conversion events, last-click, across all custom event names.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub custom_conversions: f64,
    /// Whop pixel-attributed custom conversions, keyed by your event name with its last-click count as the value. Empty when no named custom events are attributed. Custom events fired without a name are counted in custom_conversions but omitted here, so these values sum to at most custom_conversions.
    #[serde(default)]
    pub custom_event_counts: HashMap<String, serde_json::Value>,
    /// Conversion value attributed to each custom event, keyed by event name like custom_event_counts. Sums the value passed to whop.track, normalized to USD; events fired without a value contribute 0.
    #[serde(default)]
    pub custom_event_values: HashMap<String, serde_json::Value>,
    /// Whether ads in this ad group are delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
    pub delivery_status: AdGroupDeliveryStatus,
    /// Age, gender, and automatic-audience targeting.
    pub demographics: AdGroupDemographics,
    /// Cost per result to aim for (`average_target`) or never exceed (`maximum_target`). `null` for `minimum_cost` bidding.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub desired_cost_per_result: Option<f64>,
    /// Interest, behavior, and demographic targeting, using categories from the ad platform's targeting taxonomy. Entries across interests, behaviors, and demographics are OR'd together (anyone matching any entry is reached), matching Ads Manager's detailed-targeting box. Can't be combined with automatic audience targeting, and unavailable to campaigns with special_ad_categories.
    #[serde(default)]
    pub detailed_targeting: AdGroupDetailedTargeting,
    /// Device platforms and operating systems targeted.
    #[serde(default)]
    pub devices: AdGroupDevices,
    /// Whether the ad platform automatically mixes and matches this ad group's creatives and copy to find the best-performing combinations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_creative: Option<bool>,
    /// When the ad group stops delivering, as an ISO 8601 timestamp. `null` when it runs until paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// Platform-reported impressions divided by reach.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub frequency: Option<f64>,
    /// Cap on how often one person sees ads from this ad group. Only available on campaigns with the `awareness` objective; `null` when uncapped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_cap: Option<AdGroupFrequencyCap>,
    /// Unique identifier for the ad group, prefixed `adgrp_`.
    #[serde(default)]
    pub id: String,
    /// The number of impressions.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub impressions: f64,
    #[serde(default)]
    pub issues: Vec<AdPlatformIssue>,
    #[serde(default)]
    pub languages: Vec<String>,
    /// USD value attributed to lead events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lead_value: f64,
    /// Whop pixel-attributed leads, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub leads: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_apps: Option<Vec<AdGroupMessageAppsItem>>,
    /// Minimum the ad group tries to spend each day. `null` when no floor is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_daily_spend: Option<f64>,
    /// The result the ad group's delivery is optimized to get the most of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_goal: Option<AdGroupOptimizationGoal>,
    #[serde(default)]
    pub placements: Vec<AdGroupPlacement>,
    /// USD value of pixel-attributed purchases.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub purchase_value: f64,
    /// Whop pixel-attributed purchases, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub purchases: f64,
    /// The number of unique people who saw this.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub reach: f64,
    /// Locations targeted and excluded.
    #[serde(default)]
    pub regions: AdGroupRegions,
    /// The Whop pixel conversion event whose attributed count represents results — the optimization goal, or the highest-volume attributed event for campaigns that budget per ad group. Null when the goal isn't a Whop-attributed event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_event: Option<AdGroupResultEvent>,
    /// The merchant-defined event name when result_event is custom; null for the standard events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_event_name: Option<String>,
    /// The Whop pixel-attributed count behind result_event. When a campaign's ad groups optimize different goals there is no single result_event (it is null), and this is instead the sum of each ad group's own attributed results. Null when nothing Whop-attributable is being optimized for.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub results: Option<f64>,
    /// Purchase value divided by spend, both in USD (a currency-neutral ratio); 0 when there is no spend.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub return_on_ad_spend: f64,
    /// USD value attributed to schedule events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub schedule_value: f64,
    /// Whop pixel-attributed schedule events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub schedules: f64,
    /// The amount charged, in spend_currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub spend: f64,
    /// The ISO 4217 currency code of all monetary metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_currency: Option<String>,
    /// When the ad group starts delivering, as an ISO 8601 timestamp. `null` when it starts as soon as it's active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    /// Whether the ad group is enabled. `active` and `paused` are set by you; `rejected` means it failed ad review; `duplicating` is a copy still being filled in.
    pub status: AdGroupStatus,
    /// USD value attributed to submit-application events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub submitted_application_value: f64,
    /// Whop pixel-attributed submit-application events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub submitted_applications: f64,
    /// Display name of the ad group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Unique clicks divided by impressions, between 0 and 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unique_click_through_rate: Option<f64>,
    /// People who clicked, reported by the Whop pixel, counted once per person.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub unique_clicks: f64,
    /// When the ad group was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// USD value attributed to view-content events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub viewed_content_value: f64,
    /// Whop pixel-attributed view-content events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub viewed_contents: f64,
}

impl AdGroup {
    pub fn builder() -> AdGroupBuilder {
        <AdGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupBuilder {
    ad_campaign: Option<AdEntityReference>,
    added_to_cart_value: Option<f64>,
    added_to_carts: Option<f64>,
    audiences: Option<AdGroupAudiences>,
    bid_type: Option<AdGroupBidType>,
    budget_amount: Option<f64>,
    budget_type: Option<AdGroupBudgetType>,
    click_through_rate: Option<f64>,
    clicks: Option<f64>,
    completed_registration_value: Option<f64>,
    completed_registrations: Option<f64>,
    contact_value: Option<f64>,
    contacts: Option<f64>,
    conversion_event: Option<ConversionEvent>,
    conversion_location: Option<AdGroupConversionLocation>,
    cost_per_added_to_cart: Option<f64>,
    cost_per_click: Option<f64>,
    cost_per_completed_registration: Option<f64>,
    cost_per_contact: Option<f64>,
    cost_per_lead: Option<f64>,
    cost_per_mille: Option<f64>,
    cost_per_purchase: Option<f64>,
    cost_per_result: Option<f64>,
    cost_per_schedule: Option<f64>,
    cost_per_submitted_application: Option<f64>,
    cost_per_unique_click: Option<f64>,
    cost_per_viewed_content: Option<f64>,
    created_at: Option<String>,
    custom_conversions: Option<f64>,
    custom_event_counts: Option<HashMap<String, serde_json::Value>>,
    custom_event_values: Option<HashMap<String, serde_json::Value>>,
    delivery_status: Option<AdGroupDeliveryStatus>,
    demographics: Option<AdGroupDemographics>,
    desired_cost_per_result: Option<f64>,
    detailed_targeting: Option<AdGroupDetailedTargeting>,
    devices: Option<AdGroupDevices>,
    dynamic_creative: Option<bool>,
    ends_at: Option<String>,
    frequency: Option<f64>,
    frequency_cap: Option<AdGroupFrequencyCap>,
    id: Option<String>,
    impressions: Option<f64>,
    issues: Option<Vec<AdPlatformIssue>>,
    languages: Option<Vec<String>>,
    lead_value: Option<f64>,
    leads: Option<f64>,
    message_apps: Option<Vec<AdGroupMessageAppsItem>>,
    minimum_daily_spend: Option<f64>,
    optimization_goal: Option<AdGroupOptimizationGoal>,
    placements: Option<Vec<AdGroupPlacement>>,
    purchase_value: Option<f64>,
    purchases: Option<f64>,
    reach: Option<f64>,
    regions: Option<AdGroupRegions>,
    result_event: Option<AdGroupResultEvent>,
    result_event_name: Option<String>,
    results: Option<f64>,
    return_on_ad_spend: Option<f64>,
    schedule_value: Option<f64>,
    schedules: Option<f64>,
    spend: Option<f64>,
    spend_currency: Option<String>,
    starts_at: Option<String>,
    status: Option<AdGroupStatus>,
    submitted_application_value: Option<f64>,
    submitted_applications: Option<f64>,
    title: Option<String>,
    unique_click_through_rate: Option<f64>,
    unique_clicks: Option<f64>,
    updated_at: Option<String>,
    viewed_content_value: Option<f64>,
    viewed_contents: Option<f64>,
}

impl AdGroupBuilder {
    pub fn ad_campaign(mut self, value: AdEntityReference) -> Self {
        self.ad_campaign = Some(value);
        self
    }

    pub fn added_to_cart_value(mut self, value: f64) -> Self {
        self.added_to_cart_value = Some(value);
        self
    }

    pub fn added_to_carts(mut self, value: f64) -> Self {
        self.added_to_carts = Some(value);
        self
    }

    pub fn audiences(mut self, value: AdGroupAudiences) -> Self {
        self.audiences = Some(value);
        self
    }

    pub fn bid_type(mut self, value: AdGroupBidType) -> Self {
        self.bid_type = Some(value);
        self
    }

    pub fn budget_amount(mut self, value: f64) -> Self {
        self.budget_amount = Some(value);
        self
    }

    pub fn budget_type(mut self, value: AdGroupBudgetType) -> Self {
        self.budget_type = Some(value);
        self
    }

    pub fn click_through_rate(mut self, value: f64) -> Self {
        self.click_through_rate = Some(value);
        self
    }

    pub fn clicks(mut self, value: f64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn completed_registration_value(mut self, value: f64) -> Self {
        self.completed_registration_value = Some(value);
        self
    }

    pub fn completed_registrations(mut self, value: f64) -> Self {
        self.completed_registrations = Some(value);
        self
    }

    pub fn contact_value(mut self, value: f64) -> Self {
        self.contact_value = Some(value);
        self
    }

    pub fn contacts(mut self, value: f64) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn conversion_event(mut self, value: ConversionEvent) -> Self {
        self.conversion_event = Some(value);
        self
    }

    pub fn conversion_location(mut self, value: AdGroupConversionLocation) -> Self {
        self.conversion_location = Some(value);
        self
    }

    pub fn cost_per_added_to_cart(mut self, value: f64) -> Self {
        self.cost_per_added_to_cart = Some(value);
        self
    }

    pub fn cost_per_click(mut self, value: f64) -> Self {
        self.cost_per_click = Some(value);
        self
    }

    pub fn cost_per_completed_registration(mut self, value: f64) -> Self {
        self.cost_per_completed_registration = Some(value);
        self
    }

    pub fn cost_per_contact(mut self, value: f64) -> Self {
        self.cost_per_contact = Some(value);
        self
    }

    pub fn cost_per_lead(mut self, value: f64) -> Self {
        self.cost_per_lead = Some(value);
        self
    }

    pub fn cost_per_mille(mut self, value: f64) -> Self {
        self.cost_per_mille = Some(value);
        self
    }

    pub fn cost_per_purchase(mut self, value: f64) -> Self {
        self.cost_per_purchase = Some(value);
        self
    }

    pub fn cost_per_result(mut self, value: f64) -> Self {
        self.cost_per_result = Some(value);
        self
    }

    pub fn cost_per_schedule(mut self, value: f64) -> Self {
        self.cost_per_schedule = Some(value);
        self
    }

    pub fn cost_per_submitted_application(mut self, value: f64) -> Self {
        self.cost_per_submitted_application = Some(value);
        self
    }

    pub fn cost_per_unique_click(mut self, value: f64) -> Self {
        self.cost_per_unique_click = Some(value);
        self
    }

    pub fn cost_per_viewed_content(mut self, value: f64) -> Self {
        self.cost_per_viewed_content = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn custom_conversions(mut self, value: f64) -> Self {
        self.custom_conversions = Some(value);
        self
    }

    pub fn custom_event_counts(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_event_counts = Some(value);
        self
    }

    pub fn custom_event_values(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_event_values = Some(value);
        self
    }

    pub fn delivery_status(mut self, value: AdGroupDeliveryStatus) -> Self {
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

    pub fn detailed_targeting(mut self, value: AdGroupDetailedTargeting) -> Self {
        self.detailed_targeting = Some(value);
        self
    }

    pub fn devices(mut self, value: AdGroupDevices) -> Self {
        self.devices = Some(value);
        self
    }

    pub fn dynamic_creative(mut self, value: bool) -> Self {
        self.dynamic_creative = Some(value);
        self
    }

    pub fn ends_at(mut self, value: impl Into<String>) -> Self {
        self.ends_at = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: f64) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn frequency_cap(mut self, value: AdGroupFrequencyCap) -> Self {
        self.frequency_cap = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn impressions(mut self, value: f64) -> Self {
        self.impressions = Some(value);
        self
    }

    pub fn issues(mut self, value: Vec<AdPlatformIssue>) -> Self {
        self.issues = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn lead_value(mut self, value: f64) -> Self {
        self.lead_value = Some(value);
        self
    }

    pub fn leads(mut self, value: f64) -> Self {
        self.leads = Some(value);
        self
    }

    pub fn message_apps(mut self, value: Vec<AdGroupMessageAppsItem>) -> Self {
        self.message_apps = Some(value);
        self
    }

    pub fn minimum_daily_spend(mut self, value: f64) -> Self {
        self.minimum_daily_spend = Some(value);
        self
    }

    pub fn optimization_goal(mut self, value: AdGroupOptimizationGoal) -> Self {
        self.optimization_goal = Some(value);
        self
    }

    pub fn placements(mut self, value: Vec<AdGroupPlacement>) -> Self {
        self.placements = Some(value);
        self
    }

    pub fn purchase_value(mut self, value: f64) -> Self {
        self.purchase_value = Some(value);
        self
    }

    pub fn purchases(mut self, value: f64) -> Self {
        self.purchases = Some(value);
        self
    }

    pub fn reach(mut self, value: f64) -> Self {
        self.reach = Some(value);
        self
    }

    pub fn regions(mut self, value: AdGroupRegions) -> Self {
        self.regions = Some(value);
        self
    }

    pub fn result_event(mut self, value: AdGroupResultEvent) -> Self {
        self.result_event = Some(value);
        self
    }

    pub fn result_event_name(mut self, value: impl Into<String>) -> Self {
        self.result_event_name = Some(value.into());
        self
    }

    pub fn results(mut self, value: f64) -> Self {
        self.results = Some(value);
        self
    }

    pub fn return_on_ad_spend(mut self, value: f64) -> Self {
        self.return_on_ad_spend = Some(value);
        self
    }

    pub fn schedule_value(mut self, value: f64) -> Self {
        self.schedule_value = Some(value);
        self
    }

    pub fn schedules(mut self, value: f64) -> Self {
        self.schedules = Some(value);
        self
    }

    pub fn spend(mut self, value: f64) -> Self {
        self.spend = Some(value);
        self
    }

    pub fn spend_currency(mut self, value: impl Into<String>) -> Self {
        self.spend_currency = Some(value.into());
        self
    }

    pub fn starts_at(mut self, value: impl Into<String>) -> Self {
        self.starts_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: AdGroupStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn submitted_application_value(mut self, value: f64) -> Self {
        self.submitted_application_value = Some(value);
        self
    }

    pub fn submitted_applications(mut self, value: f64) -> Self {
        self.submitted_applications = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn unique_click_through_rate(mut self, value: f64) -> Self {
        self.unique_click_through_rate = Some(value);
        self
    }

    pub fn unique_clicks(mut self, value: f64) -> Self {
        self.unique_clicks = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn viewed_content_value(mut self, value: f64) -> Self {
        self.viewed_content_value = Some(value);
        self
    }

    pub fn viewed_contents(mut self, value: f64) -> Self {
        self.viewed_contents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_campaign`](AdGroupBuilder::ad_campaign)
    /// - [`added_to_cart_value`](AdGroupBuilder::added_to_cart_value)
    /// - [`added_to_carts`](AdGroupBuilder::added_to_carts)
    /// - [`audiences`](AdGroupBuilder::audiences)
    /// - [`click_through_rate`](AdGroupBuilder::click_through_rate)
    /// - [`clicks`](AdGroupBuilder::clicks)
    /// - [`completed_registration_value`](AdGroupBuilder::completed_registration_value)
    /// - [`completed_registrations`](AdGroupBuilder::completed_registrations)
    /// - [`contact_value`](AdGroupBuilder::contact_value)
    /// - [`contacts`](AdGroupBuilder::contacts)
    /// - [`cost_per_click`](AdGroupBuilder::cost_per_click)
    /// - [`cost_per_mille`](AdGroupBuilder::cost_per_mille)
    /// - [`created_at`](AdGroupBuilder::created_at)
    /// - [`custom_conversions`](AdGroupBuilder::custom_conversions)
    /// - [`custom_event_counts`](AdGroupBuilder::custom_event_counts)
    /// - [`custom_event_values`](AdGroupBuilder::custom_event_values)
    /// - [`delivery_status`](AdGroupBuilder::delivery_status)
    /// - [`demographics`](AdGroupBuilder::demographics)
    /// - [`detailed_targeting`](AdGroupBuilder::detailed_targeting)
    /// - [`devices`](AdGroupBuilder::devices)
    /// - [`id`](AdGroupBuilder::id)
    /// - [`impressions`](AdGroupBuilder::impressions)
    /// - [`issues`](AdGroupBuilder::issues)
    /// - [`languages`](AdGroupBuilder::languages)
    /// - [`lead_value`](AdGroupBuilder::lead_value)
    /// - [`leads`](AdGroupBuilder::leads)
    /// - [`placements`](AdGroupBuilder::placements)
    /// - [`purchase_value`](AdGroupBuilder::purchase_value)
    /// - [`purchases`](AdGroupBuilder::purchases)
    /// - [`reach`](AdGroupBuilder::reach)
    /// - [`regions`](AdGroupBuilder::regions)
    /// - [`return_on_ad_spend`](AdGroupBuilder::return_on_ad_spend)
    /// - [`schedule_value`](AdGroupBuilder::schedule_value)
    /// - [`schedules`](AdGroupBuilder::schedules)
    /// - [`spend`](AdGroupBuilder::spend)
    /// - [`status`](AdGroupBuilder::status)
    /// - [`submitted_application_value`](AdGroupBuilder::submitted_application_value)
    /// - [`submitted_applications`](AdGroupBuilder::submitted_applications)
    /// - [`unique_clicks`](AdGroupBuilder::unique_clicks)
    /// - [`updated_at`](AdGroupBuilder::updated_at)
    /// - [`viewed_content_value`](AdGroupBuilder::viewed_content_value)
    /// - [`viewed_contents`](AdGroupBuilder::viewed_contents)
    pub fn build(self) -> Result<AdGroup, BuildError> {
        Ok(AdGroup {
            ad_campaign: self
                .ad_campaign
                .ok_or_else(|| BuildError::missing_field("ad_campaign"))?,
            added_to_cart_value: self
                .added_to_cart_value
                .ok_or_else(|| BuildError::missing_field("added_to_cart_value"))?,
            added_to_carts: self
                .added_to_carts
                .ok_or_else(|| BuildError::missing_field("added_to_carts"))?,
            audiences: self
                .audiences
                .ok_or_else(|| BuildError::missing_field("audiences"))?,
            bid_type: self.bid_type,
            budget_amount: self.budget_amount,
            budget_type: self.budget_type,
            click_through_rate: self
                .click_through_rate
                .ok_or_else(|| BuildError::missing_field("click_through_rate"))?,
            clicks: self
                .clicks
                .ok_or_else(|| BuildError::missing_field("clicks"))?,
            completed_registration_value: self
                .completed_registration_value
                .ok_or_else(|| BuildError::missing_field("completed_registration_value"))?,
            completed_registrations: self
                .completed_registrations
                .ok_or_else(|| BuildError::missing_field("completed_registrations"))?,
            contact_value: self
                .contact_value
                .ok_or_else(|| BuildError::missing_field("contact_value"))?,
            contacts: self
                .contacts
                .ok_or_else(|| BuildError::missing_field("contacts"))?,
            conversion_event: self.conversion_event,
            conversion_location: self.conversion_location,
            cost_per_added_to_cart: self.cost_per_added_to_cart,
            cost_per_click: self
                .cost_per_click
                .ok_or_else(|| BuildError::missing_field("cost_per_click"))?,
            cost_per_completed_registration: self.cost_per_completed_registration,
            cost_per_contact: self.cost_per_contact,
            cost_per_lead: self.cost_per_lead,
            cost_per_mille: self
                .cost_per_mille
                .ok_or_else(|| BuildError::missing_field("cost_per_mille"))?,
            cost_per_purchase: self.cost_per_purchase,
            cost_per_result: self.cost_per_result,
            cost_per_schedule: self.cost_per_schedule,
            cost_per_submitted_application: self.cost_per_submitted_application,
            cost_per_unique_click: self.cost_per_unique_click,
            cost_per_viewed_content: self.cost_per_viewed_content,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            custom_conversions: self
                .custom_conversions
                .ok_or_else(|| BuildError::missing_field("custom_conversions"))?,
            custom_event_counts: self
                .custom_event_counts
                .ok_or_else(|| BuildError::missing_field("custom_event_counts"))?,
            custom_event_values: self
                .custom_event_values
                .ok_or_else(|| BuildError::missing_field("custom_event_values"))?,
            delivery_status: self
                .delivery_status
                .ok_or_else(|| BuildError::missing_field("delivery_status"))?,
            demographics: self
                .demographics
                .ok_or_else(|| BuildError::missing_field("demographics"))?,
            desired_cost_per_result: self.desired_cost_per_result,
            detailed_targeting: self
                .detailed_targeting
                .ok_or_else(|| BuildError::missing_field("detailed_targeting"))?,
            devices: self
                .devices
                .ok_or_else(|| BuildError::missing_field("devices"))?,
            dynamic_creative: self.dynamic_creative,
            ends_at: self.ends_at,
            frequency: self.frequency,
            frequency_cap: self.frequency_cap,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            impressions: self
                .impressions
                .ok_or_else(|| BuildError::missing_field("impressions"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            languages: self
                .languages
                .ok_or_else(|| BuildError::missing_field("languages"))?,
            lead_value: self
                .lead_value
                .ok_or_else(|| BuildError::missing_field("lead_value"))?,
            leads: self
                .leads
                .ok_or_else(|| BuildError::missing_field("leads"))?,
            message_apps: self.message_apps,
            minimum_daily_spend: self.minimum_daily_spend,
            optimization_goal: self.optimization_goal,
            placements: self
                .placements
                .ok_or_else(|| BuildError::missing_field("placements"))?,
            purchase_value: self
                .purchase_value
                .ok_or_else(|| BuildError::missing_field("purchase_value"))?,
            purchases: self
                .purchases
                .ok_or_else(|| BuildError::missing_field("purchases"))?,
            reach: self
                .reach
                .ok_or_else(|| BuildError::missing_field("reach"))?,
            regions: self
                .regions
                .ok_or_else(|| BuildError::missing_field("regions"))?,
            result_event: self.result_event,
            result_event_name: self.result_event_name,
            results: self.results,
            return_on_ad_spend: self
                .return_on_ad_spend
                .ok_or_else(|| BuildError::missing_field("return_on_ad_spend"))?,
            schedule_value: self
                .schedule_value
                .ok_or_else(|| BuildError::missing_field("schedule_value"))?,
            schedules: self
                .schedules
                .ok_or_else(|| BuildError::missing_field("schedules"))?,
            spend: self
                .spend
                .ok_or_else(|| BuildError::missing_field("spend"))?,
            spend_currency: self.spend_currency,
            starts_at: self.starts_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            submitted_application_value: self
                .submitted_application_value
                .ok_or_else(|| BuildError::missing_field("submitted_application_value"))?,
            submitted_applications: self
                .submitted_applications
                .ok_or_else(|| BuildError::missing_field("submitted_applications"))?,
            title: self.title,
            unique_click_through_rate: self.unique_click_through_rate,
            unique_clicks: self
                .unique_clicks
                .ok_or_else(|| BuildError::missing_field("unique_clicks"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            viewed_content_value: self
                .viewed_content_value
                .ok_or_else(|| BuildError::missing_field("viewed_content_value"))?,
            viewed_contents: self
                .viewed_contents
                .ok_or_else(|| BuildError::missing_field("viewed_contents"))?,
        })
    }
}
