pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ad {
    /// The ad campaign this ad belongs to.
    #[serde(default)]
    pub ad_campaign: AdEntityReference,
    /// The ad group this ad belongs to.
    #[serde(default)]
    pub ad_group: AdEntityReference,
    /// USD value attributed to add-to-cart events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub added_to_cart_value: f64,
    /// Whop pixel-attributed add-to-cart events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub added_to_carts: f64,
    /// The call-to-action button shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<AdCallToAction>,
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
    /// When the ad was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub creatives: Vec<AdCreative>,
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
    /// Whether the ad is delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
    pub delivery_status: AdDeliveryStatus,
    #[serde(default)]
    pub descriptions: Vec<String>,
    /// The post you pointed this ad at, when it promotes one you already published — a Facebook post, Instagram media, or TikTok video ID. `null` when the ad uses uploaded creatives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_post_id: Option<String>,
    /// Platform-reported impressions divided by reach.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub frequency: Option<f64>,
    #[serde(default)]
    pub headlines: Vec<String>,
    /// Unique identifier for the ad, prefixed `ad_`.
    #[serde(default)]
    pub id: String,
    /// The number of impressions.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub impressions: f64,
    #[serde(default)]
    pub issues: Vec<AdPlatformIssue>,
    /// The instant lead form shown when someone taps this ad. `null` when the ad group's conversion_location is not an instant-form destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_form: Option<AdLeadForm>,
    /// The ad platform's ID for the instant form the ad uses. Set when the ad references an existing form via `lead_form_id`, or once a form built from `lead_form` has been created on the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_form_id: Option<String>,
    /// USD value attributed to lead events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lead_value: f64,
    /// Whop pixel-attributed leads, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub leads: f64,
    /// Clicks on links in the ad that lead to your destination, as reported by the ad platform. A subset of clicks, which also counts likes, comments, and other interactions with the ad.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub link_clicks: f64,
    /// Welcome message for click-to-message ads, shown when the conversation opens. `null` when the ad has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messaging_config: Option<AdMessagingConfig>,
    /// Whether the ad can appear alongside other advertisers' ads in the same unit. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_advertiser_ads: Option<bool>,
    /// The post the ad network serves for this ad, as `pageID_postID` on Meta — the post Meta created for an uploaded creative, or the post being promoted. Use it to open the live post, or to promote the same post from another ad. `null` until the network has created the post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    /// Identifies the network that owns `existing_post_id`; `null` when the ad uses uploaded creatives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_source: Option<AdPostSource>,
    /// Preview image of the post named by `existing_post_id`. `null` for ads that use uploaded creatives, or until the post's media has been fetched from the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_thumbnail_url: Option<String>,
    #[serde(default)]
    pub primary_texts: Vec<String>,
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
    pub result_event: Option<AdResultEvent>,
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
    pub social_accounts: Vec<AdEntityReference>,
    /// The amount charged, in spend_currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub spend: f64,
    /// The ISO 4217 currency code of all monetary metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spend_currency: Option<String>,
    /// Whether the ad is enabled. `active` and `paused` are set by you; `in_review` and `rejected` come from ad review.
    pub status: AdStatus,
    /// USD value attributed to submit-application events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub submitted_application_value: f64,
    /// Whop pixel-attributed submit-application events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub submitted_applications: f64,
    /// Display title of the ad.
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
    /// When the ad was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The URL the ad links to, without its query string. Parameters belong in `url_parameters`; any you send on `url` are moved there.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Every query parameter appended to the URL, keyed by parameter name — including any you sent on `url` itself. Whop adds its own click-attribution parameters on top; those are reserved and rejected if you set them. Which keys are reserved depends on the ad's network — Meta: utm_meta_ad_id, utm_meta_adset_id, utm_meta_campaign_id, utm_source, utm_placement, utm_medium, utm_content, utm_adset, utm_whop, wacid, wasid, waid, tw_source, tw_adid; TikTok: waid, wasid, wacid, ad_id, adset_id, campaign_id, utm_source, utm_medium, utm_placement, utm_whop, tw_source, tw_adid.
    #[serde(default)]
    pub url_parameters: HashMap<String, serde_json::Value>,
    /// USD value attributed to view-content events. Sums the value sent with each event, normalized to USD; events without a value contribute 0.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub viewed_content_value: f64,
    /// Whop pixel-attributed view-content events, last-click.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub viewed_contents: f64,
}

impl Ad {
    pub fn builder() -> AdBuilder {
        <AdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdBuilder {
    ad_campaign: Option<AdEntityReference>,
    ad_group: Option<AdEntityReference>,
    added_to_cart_value: Option<f64>,
    added_to_carts: Option<f64>,
    call_to_action: Option<AdCallToAction>,
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
    creatives: Option<Vec<AdCreative>>,
    custom_conversions: Option<f64>,
    custom_event_counts: Option<HashMap<String, serde_json::Value>>,
    custom_event_values: Option<HashMap<String, serde_json::Value>>,
    delivery_status: Option<AdDeliveryStatus>,
    descriptions: Option<Vec<String>>,
    existing_post_id: Option<String>,
    frequency: Option<f64>,
    headlines: Option<Vec<String>>,
    id: Option<String>,
    impressions: Option<f64>,
    issues: Option<Vec<AdPlatformIssue>>,
    lead_form: Option<AdLeadForm>,
    lead_form_id: Option<String>,
    lead_value: Option<f64>,
    leads: Option<f64>,
    link_clicks: Option<f64>,
    messaging_config: Option<AdMessagingConfig>,
    multi_advertiser_ads: Option<bool>,
    post_id: Option<String>,
    post_source: Option<AdPostSource>,
    post_thumbnail_url: Option<String>,
    primary_texts: Option<Vec<String>>,
    purchase_value: Option<f64>,
    purchases: Option<f64>,
    reach: Option<f64>,
    result_event: Option<AdResultEvent>,
    result_event_name: Option<String>,
    results: Option<f64>,
    return_on_ad_spend: Option<f64>,
    schedule_value: Option<f64>,
    schedules: Option<f64>,
    social_accounts: Option<Vec<AdEntityReference>>,
    spend: Option<f64>,
    spend_currency: Option<String>,
    status: Option<AdStatus>,
    submitted_application_value: Option<f64>,
    submitted_applications: Option<f64>,
    title: Option<String>,
    unique_click_through_rate: Option<f64>,
    unique_clicks: Option<f64>,
    updated_at: Option<String>,
    url: Option<String>,
    url_parameters: Option<HashMap<String, serde_json::Value>>,
    viewed_content_value: Option<f64>,
    viewed_contents: Option<f64>,
}

impl AdBuilder {
    pub fn ad_campaign(mut self, value: AdEntityReference) -> Self {
        self.ad_campaign = Some(value);
        self
    }

    pub fn ad_group(mut self, value: AdEntityReference) -> Self {
        self.ad_group = Some(value);
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

    pub fn call_to_action(mut self, value: AdCallToAction) -> Self {
        self.call_to_action = Some(value);
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

    pub fn creatives(mut self, value: Vec<AdCreative>) -> Self {
        self.creatives = Some(value);
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

    pub fn delivery_status(mut self, value: AdDeliveryStatus) -> Self {
        self.delivery_status = Some(value);
        self
    }

    pub fn descriptions(mut self, value: Vec<String>) -> Self {
        self.descriptions = Some(value);
        self
    }

    pub fn existing_post_id(mut self, value: impl Into<String>) -> Self {
        self.existing_post_id = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: f64) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn headlines(mut self, value: Vec<String>) -> Self {
        self.headlines = Some(value);
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

    pub fn lead_form(mut self, value: AdLeadForm) -> Self {
        self.lead_form = Some(value);
        self
    }

    pub fn lead_form_id(mut self, value: impl Into<String>) -> Self {
        self.lead_form_id = Some(value.into());
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

    pub fn link_clicks(mut self, value: f64) -> Self {
        self.link_clicks = Some(value);
        self
    }

    pub fn messaging_config(mut self, value: AdMessagingConfig) -> Self {
        self.messaging_config = Some(value);
        self
    }

    pub fn multi_advertiser_ads(mut self, value: bool) -> Self {
        self.multi_advertiser_ads = Some(value);
        self
    }

    pub fn post_id(mut self, value: impl Into<String>) -> Self {
        self.post_id = Some(value.into());
        self
    }

    pub fn post_source(mut self, value: AdPostSource) -> Self {
        self.post_source = Some(value);
        self
    }

    pub fn post_thumbnail_url(mut self, value: impl Into<String>) -> Self {
        self.post_thumbnail_url = Some(value.into());
        self
    }

    pub fn primary_texts(mut self, value: Vec<String>) -> Self {
        self.primary_texts = Some(value);
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

    pub fn result_event(mut self, value: AdResultEvent) -> Self {
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

    pub fn social_accounts(mut self, value: Vec<AdEntityReference>) -> Self {
        self.social_accounts = Some(value);
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

    pub fn status(mut self, value: AdStatus) -> Self {
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

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn url_parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.url_parameters = Some(value);
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

    /// Consumes the builder and constructs a [`Ad`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_campaign`](AdBuilder::ad_campaign)
    /// - [`ad_group`](AdBuilder::ad_group)
    /// - [`added_to_cart_value`](AdBuilder::added_to_cart_value)
    /// - [`added_to_carts`](AdBuilder::added_to_carts)
    /// - [`click_through_rate`](AdBuilder::click_through_rate)
    /// - [`clicks`](AdBuilder::clicks)
    /// - [`completed_registration_value`](AdBuilder::completed_registration_value)
    /// - [`completed_registrations`](AdBuilder::completed_registrations)
    /// - [`contact_value`](AdBuilder::contact_value)
    /// - [`contacts`](AdBuilder::contacts)
    /// - [`cost_per_click`](AdBuilder::cost_per_click)
    /// - [`cost_per_mille`](AdBuilder::cost_per_mille)
    /// - [`created_at`](AdBuilder::created_at)
    /// - [`creatives`](AdBuilder::creatives)
    /// - [`custom_conversions`](AdBuilder::custom_conversions)
    /// - [`custom_event_counts`](AdBuilder::custom_event_counts)
    /// - [`custom_event_values`](AdBuilder::custom_event_values)
    /// - [`delivery_status`](AdBuilder::delivery_status)
    /// - [`descriptions`](AdBuilder::descriptions)
    /// - [`headlines`](AdBuilder::headlines)
    /// - [`id`](AdBuilder::id)
    /// - [`impressions`](AdBuilder::impressions)
    /// - [`issues`](AdBuilder::issues)
    /// - [`lead_value`](AdBuilder::lead_value)
    /// - [`leads`](AdBuilder::leads)
    /// - [`link_clicks`](AdBuilder::link_clicks)
    /// - [`primary_texts`](AdBuilder::primary_texts)
    /// - [`purchase_value`](AdBuilder::purchase_value)
    /// - [`purchases`](AdBuilder::purchases)
    /// - [`reach`](AdBuilder::reach)
    /// - [`return_on_ad_spend`](AdBuilder::return_on_ad_spend)
    /// - [`schedule_value`](AdBuilder::schedule_value)
    /// - [`schedules`](AdBuilder::schedules)
    /// - [`social_accounts`](AdBuilder::social_accounts)
    /// - [`spend`](AdBuilder::spend)
    /// - [`status`](AdBuilder::status)
    /// - [`submitted_application_value`](AdBuilder::submitted_application_value)
    /// - [`submitted_applications`](AdBuilder::submitted_applications)
    /// - [`unique_clicks`](AdBuilder::unique_clicks)
    /// - [`updated_at`](AdBuilder::updated_at)
    /// - [`url_parameters`](AdBuilder::url_parameters)
    /// - [`viewed_content_value`](AdBuilder::viewed_content_value)
    /// - [`viewed_contents`](AdBuilder::viewed_contents)
    pub fn build(self) -> Result<Ad, BuildError> {
        Ok(Ad {
            ad_campaign: self
                .ad_campaign
                .ok_or_else(|| BuildError::missing_field("ad_campaign"))?,
            ad_group: self
                .ad_group
                .ok_or_else(|| BuildError::missing_field("ad_group"))?,
            added_to_cart_value: self
                .added_to_cart_value
                .ok_or_else(|| BuildError::missing_field("added_to_cart_value"))?,
            added_to_carts: self
                .added_to_carts
                .ok_or_else(|| BuildError::missing_field("added_to_carts"))?,
            call_to_action: self.call_to_action,
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
            creatives: self
                .creatives
                .ok_or_else(|| BuildError::missing_field("creatives"))?,
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
            descriptions: self
                .descriptions
                .ok_or_else(|| BuildError::missing_field("descriptions"))?,
            existing_post_id: self.existing_post_id,
            frequency: self.frequency,
            headlines: self
                .headlines
                .ok_or_else(|| BuildError::missing_field("headlines"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            impressions: self
                .impressions
                .ok_or_else(|| BuildError::missing_field("impressions"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            lead_form: self.lead_form,
            lead_form_id: self.lead_form_id,
            lead_value: self
                .lead_value
                .ok_or_else(|| BuildError::missing_field("lead_value"))?,
            leads: self
                .leads
                .ok_or_else(|| BuildError::missing_field("leads"))?,
            link_clicks: self
                .link_clicks
                .ok_or_else(|| BuildError::missing_field("link_clicks"))?,
            messaging_config: self.messaging_config,
            multi_advertiser_ads: self.multi_advertiser_ads,
            post_id: self.post_id,
            post_source: self.post_source,
            post_thumbnail_url: self.post_thumbnail_url,
            primary_texts: self
                .primary_texts
                .ok_or_else(|| BuildError::missing_field("primary_texts"))?,
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
            social_accounts: self
                .social_accounts
                .ok_or_else(|| BuildError::missing_field("social_accounts"))?,
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
            title: self.title,
            unique_click_through_rate: self.unique_click_through_rate,
            unique_clicks: self
                .unique_clicks
                .ok_or_else(|| BuildError::missing_field("unique_clicks"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            url: self.url,
            url_parameters: self
                .url_parameters
                .ok_or_else(|| BuildError::missing_field("url_parameters"))?,
            viewed_content_value: self
                .viewed_content_value
                .ok_or_else(|| BuildError::missing_field("viewed_content_value"))?,
            viewed_contents: self
                .viewed_contents
                .ok_or_else(|| BuildError::missing_field("viewed_contents"))?,
        })
    }
}
