pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdCampaign {
    /// USD value attributed to add-to-cart events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub added_to_cart_value: f64,
    /// Whop pixel-attributed add-to-cart events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub added_to_carts: f64,
    /// How delivery bids in the ad auction: `minimum_cost` gets the most results for the budget, `average_target` holds an average cost per result, and `maximum_target` never bids above a cap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_type: Option<AdCampaignBidType>,
    /// The campaign's budget, in the ad account's currency. `null` when each ad group sets its own budget instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub budget_amount: Option<f64>,
    /// Which level owns the budget: the whole campaign (`ad_campaign`) or each ad group individually (`ad_group`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_optimization: Option<AdCampaignBudgetOptimization>,
    /// Whether `budget_amount` is spent per day (`daily`) or over the campaign's full run (`lifetime`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget_type: Option<AdCampaignBudgetType>,
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
    /// When the campaign was created, as an ISO 8601 timestamp.
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
    /// Whether the campaign's ads are delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
    pub delivery_status: AdCampaignDeliveryStatus,
    /// Platform-reported impressions divided by reach.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub frequency: Option<f64>,
    /// Unique identifier for the ad campaign, prefixed `adcamp_`.
    #[serde(default)]
    pub id: String,
    /// The number of impressions.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub impressions: f64,
    #[serde(default)]
    pub issues: Vec<AdPlatformIssue>,
    /// USD value attributed to lead events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lead_value: f64,
    /// Whop pixel-attributed leads, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub leads: f64,
    /// The goal the campaign optimizes toward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective: Option<AdCampaignObjective>,
    /// The event the campaign optimizes for when a single goal is set campaign-wide. `null` when each ad group sets its own optimization_goal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimization_goal: Option<String>,
    /// The ad network the campaign runs on.
    pub platform: AdCampaignPlatform,
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
    /// The Whop pixel conversion event whose attributed count represents results — the optimization goal, or the highest-volume attributed event for campaigns that budget per ad group. Null when the goal isn't a Whop-attributed event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_event: Option<AdCampaignResultEvent>,
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
    #[serde(default)]
    pub special_ad_categories: Vec<AdCampaignSpecialAdCategoriesItem>,
    /// The amount charged, in spend_currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub spend: f64,
    /// The ISO 4217 currency code of all monetary metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_currency: Option<String>,
    /// The lifecycle status of the ad campaign.
    pub status: AdCampaignStatus,
    /// USD value attributed to submit-application events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub submitted_application_value: f64,
    /// Whop pixel-attributed submit-application events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub submitted_applications: f64,
    /// Display name of the ad campaign.
    #[serde(default)]
    pub title: String,
    /// Unique clicks divided by impressions, between 0 and 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unique_click_through_rate: Option<f64>,
    /// People who clicked, reported by the Whop pixel, counted once per person.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub unique_clicks: f64,
    /// When the campaign was last updated, as an ISO 8601 timestamp.
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

impl AdCampaign {
    pub fn builder() -> AdCampaignBuilder {
        <AdCampaignBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdCampaignBuilder {
    added_to_cart_value: Option<f64>,
    added_to_carts: Option<f64>,
    bid_type: Option<AdCampaignBidType>,
    budget_amount: Option<f64>,
    budget_optimization: Option<AdCampaignBudgetOptimization>,
    budget_type: Option<AdCampaignBudgetType>,
    click_through_rate: Option<f64>,
    clicks: Option<f64>,
    completed_registration_value: Option<f64>,
    completed_registrations: Option<f64>,
    contact_value: Option<f64>,
    contacts: Option<f64>,
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
    delivery_status: Option<AdCampaignDeliveryStatus>,
    frequency: Option<f64>,
    id: Option<String>,
    impressions: Option<f64>,
    issues: Option<Vec<AdPlatformIssue>>,
    lead_value: Option<f64>,
    leads: Option<f64>,
    objective: Option<AdCampaignObjective>,
    optimization_goal: Option<String>,
    platform: Option<AdCampaignPlatform>,
    purchase_value: Option<f64>,
    purchases: Option<f64>,
    reach: Option<f64>,
    result_event: Option<AdCampaignResultEvent>,
    result_event_name: Option<String>,
    results: Option<f64>,
    return_on_ad_spend: Option<f64>,
    schedule_value: Option<f64>,
    schedules: Option<f64>,
    special_ad_categories: Option<Vec<AdCampaignSpecialAdCategoriesItem>>,
    spend: Option<f64>,
    spend_currency: Option<String>,
    status: Option<AdCampaignStatus>,
    submitted_application_value: Option<f64>,
    submitted_applications: Option<f64>,
    title: Option<String>,
    unique_click_through_rate: Option<f64>,
    unique_clicks: Option<f64>,
    updated_at: Option<String>,
    viewed_content_value: Option<f64>,
    viewed_contents: Option<f64>,
}

impl AdCampaignBuilder {
    pub fn added_to_cart_value(mut self, value: f64) -> Self {
        self.added_to_cart_value = Some(value);
        self
    }

    pub fn added_to_carts(mut self, value: f64) -> Self {
        self.added_to_carts = Some(value);
        self
    }

    pub fn bid_type(mut self, value: AdCampaignBidType) -> Self {
        self.bid_type = Some(value);
        self
    }

    pub fn budget_amount(mut self, value: f64) -> Self {
        self.budget_amount = Some(value);
        self
    }

    pub fn budget_optimization(mut self, value: AdCampaignBudgetOptimization) -> Self {
        self.budget_optimization = Some(value);
        self
    }

    pub fn budget_type(mut self, value: AdCampaignBudgetType) -> Self {
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

    pub fn delivery_status(mut self, value: AdCampaignDeliveryStatus) -> Self {
        self.delivery_status = Some(value);
        self
    }

    pub fn frequency(mut self, value: f64) -> Self {
        self.frequency = Some(value);
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

    pub fn lead_value(mut self, value: f64) -> Self {
        self.lead_value = Some(value);
        self
    }

    pub fn leads(mut self, value: f64) -> Self {
        self.leads = Some(value);
        self
    }

    pub fn objective(mut self, value: AdCampaignObjective) -> Self {
        self.objective = Some(value);
        self
    }

    pub fn optimization_goal(mut self, value: impl Into<String>) -> Self {
        self.optimization_goal = Some(value.into());
        self
    }

    pub fn platform(mut self, value: AdCampaignPlatform) -> Self {
        self.platform = Some(value);
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

    pub fn result_event(mut self, value: AdCampaignResultEvent) -> Self {
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

    pub fn special_ad_categories(mut self, value: Vec<AdCampaignSpecialAdCategoriesItem>) -> Self {
        self.special_ad_categories = Some(value);
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

    pub fn status(mut self, value: AdCampaignStatus) -> Self {
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

    /// Consumes the builder and constructs a [`AdCampaign`].
    /// This method will fail if any of the following fields are not set:
    /// - [`added_to_cart_value`](AdCampaignBuilder::added_to_cart_value)
    /// - [`added_to_carts`](AdCampaignBuilder::added_to_carts)
    /// - [`click_through_rate`](AdCampaignBuilder::click_through_rate)
    /// - [`clicks`](AdCampaignBuilder::clicks)
    /// - [`completed_registration_value`](AdCampaignBuilder::completed_registration_value)
    /// - [`completed_registrations`](AdCampaignBuilder::completed_registrations)
    /// - [`contact_value`](AdCampaignBuilder::contact_value)
    /// - [`contacts`](AdCampaignBuilder::contacts)
    /// - [`cost_per_click`](AdCampaignBuilder::cost_per_click)
    /// - [`cost_per_mille`](AdCampaignBuilder::cost_per_mille)
    /// - [`created_at`](AdCampaignBuilder::created_at)
    /// - [`custom_conversions`](AdCampaignBuilder::custom_conversions)
    /// - [`custom_event_counts`](AdCampaignBuilder::custom_event_counts)
    /// - [`custom_event_values`](AdCampaignBuilder::custom_event_values)
    /// - [`delivery_status`](AdCampaignBuilder::delivery_status)
    /// - [`id`](AdCampaignBuilder::id)
    /// - [`impressions`](AdCampaignBuilder::impressions)
    /// - [`issues`](AdCampaignBuilder::issues)
    /// - [`lead_value`](AdCampaignBuilder::lead_value)
    /// - [`leads`](AdCampaignBuilder::leads)
    /// - [`platform`](AdCampaignBuilder::platform)
    /// - [`purchase_value`](AdCampaignBuilder::purchase_value)
    /// - [`purchases`](AdCampaignBuilder::purchases)
    /// - [`reach`](AdCampaignBuilder::reach)
    /// - [`return_on_ad_spend`](AdCampaignBuilder::return_on_ad_spend)
    /// - [`schedule_value`](AdCampaignBuilder::schedule_value)
    /// - [`schedules`](AdCampaignBuilder::schedules)
    /// - [`special_ad_categories`](AdCampaignBuilder::special_ad_categories)
    /// - [`spend`](AdCampaignBuilder::spend)
    /// - [`status`](AdCampaignBuilder::status)
    /// - [`submitted_application_value`](AdCampaignBuilder::submitted_application_value)
    /// - [`submitted_applications`](AdCampaignBuilder::submitted_applications)
    /// - [`title`](AdCampaignBuilder::title)
    /// - [`unique_clicks`](AdCampaignBuilder::unique_clicks)
    /// - [`updated_at`](AdCampaignBuilder::updated_at)
    /// - [`viewed_content_value`](AdCampaignBuilder::viewed_content_value)
    /// - [`viewed_contents`](AdCampaignBuilder::viewed_contents)
    pub fn build(self) -> Result<AdCampaign, BuildError> {
        Ok(AdCampaign {
            added_to_cart_value: self
                .added_to_cart_value
                .ok_or_else(|| BuildError::missing_field("added_to_cart_value"))?,
            added_to_carts: self
                .added_to_carts
                .ok_or_else(|| BuildError::missing_field("added_to_carts"))?,
            bid_type: self.bid_type,
            budget_amount: self.budget_amount,
            budget_optimization: self.budget_optimization,
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
            frequency: self.frequency,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            impressions: self
                .impressions
                .ok_or_else(|| BuildError::missing_field("impressions"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            lead_value: self
                .lead_value
                .ok_or_else(|| BuildError::missing_field("lead_value"))?,
            leads: self
                .leads
                .ok_or_else(|| BuildError::missing_field("leads"))?,
            objective: self.objective,
            optimization_goal: self.optimization_goal,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            purchase_value: self
                .purchase_value
                .ok_or_else(|| BuildError::missing_field("purchase_value"))?,
            purchases: self
                .purchases
                .ok_or_else(|| BuildError::missing_field("purchases"))?,
            reach: self
                .reach
                .ok_or_else(|| BuildError::missing_field("reach"))?,
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
            special_ad_categories: self
                .special_ad_categories
                .ok_or_else(|| BuildError::missing_field("special_ad_categories"))?,
            spend: self
                .spend
                .ok_or_else(|| BuildError::missing_field("spend"))?,
            spend_currency: self.spend_currency,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            submitted_application_value: self
                .submitted_application_value
                .ok_or_else(|| BuildError::missing_field("submitted_application_value"))?,
            submitted_applications: self
                .submitted_applications
                .ok_or_else(|| BuildError::missing_field("submitted_applications"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
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
