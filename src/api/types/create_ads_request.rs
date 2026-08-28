pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAdsRequest {
    /// An inline ad group to create (same shape as POST /ad_groups, including ad_campaign_id). Creates the ad group and the ad together. Provide this OR ad_group_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<HashMap<String, serde_json::Value>>,
    /// The existing ad group to create the ad in. Provide this OR ad_group, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group_id: Option<String>,
    /// The call-to-action button shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<CreateAdsRequestCallToAction>,
    /// The ad's creative assets. Each entry is an uploaded file id with an optional format; omit format for the original asset. Two or more entries with no format become a carousel (2-10 attachments), in order, sharing the ad's copy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creatives: Option<Vec<CreateAdsRequestCreativesItem>>,
    /// The description variants shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    /// Promote a post you already published instead of uploading creatives — a Facebook post or Instagram media id. Mutually exclusive with creatives. Pair with post_source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_post_id: Option<String>,
    /// The headline variants shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headlines: Option<Vec<String>>,
    /// Instant lead form for the ad. Only allowed when the ad group's conversion_location is an instant-form destination (instant_forms, instant_forms_and_messenger, website_and_instant_forms). Mutually exclusive with lead_form_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_form: Option<CreateAdsRequestLeadForm>,
    /// Use an existing instant form instead of creating one — the form's platform ID, from a form already on the ad's Facebook page. Only allowed when the ad group's conversion_location is an instant-form destination. Mutually exclusive with lead_form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_form_id: Option<String>,
    /// Click-to-message welcome copy: the greeting (message) and the ice-breaker prompt (keyword).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messaging_config: Option<CreateAdsRequestMessagingConfig>,
    /// Whether the ad can appear alongside other advertisers' ads in the same unit. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_advertiser_ads: Option<bool>,
    /// Identifies the network that owns `existing_post_id`. The source is inferred from the ID shape when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_source: Option<CreateAdsRequestPostSource>,
    /// The primary text variants shown in the ad body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_texts: Option<Vec<String>>,
    /// The social accounts the ad runs under — a connected Facebook page and, optionally, an Instagram profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_accounts: Option<Vec<CreateAdsRequestSocialAccountsItem>>,
    /// The display name of the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL the ad links to. Query parameters are merged into url_parameters, so the stored URL is always bare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Query parameters to append to the destination URL, keyed by parameter name. Merged with any query string on `url`. Whop adds its own click-attribution parameters; those are reserved and rejected if you set them. Which keys are reserved depends on the ad's network — Meta: utm_meta_ad_id, utm_meta_adset_id, utm_meta_campaign_id, utm_source, utm_placement, utm_medium, utm_content, utm_adset, utm_whop, wacid, wasid, waid, tw_source, tw_adid; TikTok: waid, wasid, wacid, ad_id, adset_id, campaign_id, utm_source, utm_medium, utm_placement, utm_whop, tw_source, tw_adid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_parameters: Option<HashMap<String, serde_json::Value>>,
}

impl CreateAdsRequest {
    pub fn builder() -> CreateAdsRequestBuilder {
        <CreateAdsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestBuilder {
    ad_group: Option<HashMap<String, serde_json::Value>>,
    ad_group_id: Option<String>,
    call_to_action: Option<CreateAdsRequestCallToAction>,
    creatives: Option<Vec<CreateAdsRequestCreativesItem>>,
    descriptions: Option<Vec<String>>,
    existing_post_id: Option<String>,
    headlines: Option<Vec<String>>,
    lead_form: Option<CreateAdsRequestLeadForm>,
    lead_form_id: Option<String>,
    messaging_config: Option<CreateAdsRequestMessagingConfig>,
    multi_advertiser_ads: Option<bool>,
    post_source: Option<CreateAdsRequestPostSource>,
    primary_texts: Option<Vec<String>>,
    social_accounts: Option<Vec<CreateAdsRequestSocialAccountsItem>>,
    title: Option<String>,
    url: Option<String>,
    url_parameters: Option<HashMap<String, serde_json::Value>>,
}

impl CreateAdsRequestBuilder {
    pub fn ad_group(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.ad_group = Some(value);
        self
    }

    pub fn ad_group_id(mut self, value: impl Into<String>) -> Self {
        self.ad_group_id = Some(value.into());
        self
    }

    pub fn call_to_action(mut self, value: CreateAdsRequestCallToAction) -> Self {
        self.call_to_action = Some(value);
        self
    }

    pub fn creatives(mut self, value: Vec<CreateAdsRequestCreativesItem>) -> Self {
        self.creatives = Some(value);
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

    pub fn headlines(mut self, value: Vec<String>) -> Self {
        self.headlines = Some(value);
        self
    }

    pub fn lead_form(mut self, value: CreateAdsRequestLeadForm) -> Self {
        self.lead_form = Some(value);
        self
    }

    pub fn lead_form_id(mut self, value: impl Into<String>) -> Self {
        self.lead_form_id = Some(value.into());
        self
    }

    pub fn messaging_config(mut self, value: CreateAdsRequestMessagingConfig) -> Self {
        self.messaging_config = Some(value);
        self
    }

    pub fn multi_advertiser_ads(mut self, value: bool) -> Self {
        self.multi_advertiser_ads = Some(value);
        self
    }

    pub fn post_source(mut self, value: CreateAdsRequestPostSource) -> Self {
        self.post_source = Some(value);
        self
    }

    pub fn primary_texts(mut self, value: Vec<String>) -> Self {
        self.primary_texts = Some(value);
        self
    }

    pub fn social_accounts(mut self, value: Vec<CreateAdsRequestSocialAccountsItem>) -> Self {
        self.social_accounts = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateAdsRequest`].
    pub fn build(self) -> Result<CreateAdsRequest, BuildError> {
        Ok(CreateAdsRequest {
            ad_group: self.ad_group,
            ad_group_id: self.ad_group_id,
            call_to_action: self.call_to_action,
            creatives: self.creatives,
            descriptions: self.descriptions,
            existing_post_id: self.existing_post_id,
            headlines: self.headlines,
            lead_form: self.lead_form,
            lead_form_id: self.lead_form_id,
            messaging_config: self.messaging_config,
            multi_advertiser_ads: self.multi_advertiser_ads,
            post_source: self.post_source,
            primary_texts: self.primary_texts,
            social_accounts: self.social_accounts,
            title: self.title,
            url: self.url,
            url_parameters: self.url_parameters,
        })
    }
}
