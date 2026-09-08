pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostAdUpdatedPayloadData {
    /// The ad campaign this ad belongs to.
    #[serde(default)]
    pub ad_campaign: AdEntityReference,
    /// The ad group this ad belongs to.
    #[serde(default)]
    pub ad_group: AdEntityReference,
    /// The call-to-action button shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<PostAdUpdatedPayloadDataCallToAction>,
    /// When the ad was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub creatives: Vec<AdCreative>,
    /// Whether the ad is delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
    pub delivery_status: PostAdUpdatedPayloadDataDeliveryStatus,
    #[serde(default)]
    pub descriptions: Vec<String>,
    /// The post you pointed this ad at, when it promotes one you already published — a Facebook post, Instagram media, or TikTok video ID. `null` when the ad uses uploaded creatives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_post_id: Option<String>,
    #[serde(default)]
    pub headlines: Vec<String>,
    /// Unique identifier for the ad, prefixed `ad_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub issues: Vec<AdPlatformIssue>,
    /// The instant lead form shown when someone taps this ad. `null` when the ad group's conversion_location is not an instant-form destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_form: Option<AdLeadForm>,
    /// The ad platform's ID for the instant form the ad uses. Set when the ad references an existing form via `lead_form_id`, or once a form built from `lead_form` has been created on the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lead_form_id: Option<String>,
    /// Welcome message for click-to-message ads, shown when the conversation opens. `null` when the ad has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messaging_config: Option<AdMessagingConfig>,
    /// Whether the ad can appear alongside other advertisers' ads in the same unit. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_advertiser_ads: Option<bool>,
    /// The advertiser-uploaded MP3 a TikTok carousel ad plays. TikTok-only; `null` elsewhere and for non-carousel ads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music: Option<AdMusic>,
    /// The post the ad network serves for this ad, as `pageID_postID` on Meta — the post Meta created for an uploaded creative, or the post being promoted. Use it to open the live post, or to promote the same post from another ad. `null` until the network has created the post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    /// Identifies the network that owns `existing_post_id`; `null` when the ad uses uploaded creatives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_source: Option<PostAdUpdatedPayloadDataPostSource>,
    /// Preview image of the post named by `existing_post_id`. `null` for ads that use uploaded creatives, or until the post's media has been fetched from the network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_thumbnail_url: Option<String>,
    #[serde(default)]
    pub primary_texts: Vec<String>,
    #[serde(default)]
    pub social_accounts: Vec<AdEntityReference>,
    /// Whether the ad is enabled. `active` and `paused` are set by you; `in_review` and `rejected` come from ad review.
    pub status: PostAdUpdatedPayloadDataStatus,
    /// Display title of the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// When the ad was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The URL the ad links to, without its query string. Parameters belong in `url_parameters`; any you send on `url` are moved there.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Every query parameter appended to the URL, keyed by parameter name — including any you sent on `url` itself. Whop adds its own click-attribution parameters on top; those are reserved and rejected if you set them. Which keys are reserved depends on the ad's network — Meta: utm_meta_ad_id, utm_meta_adset_id, utm_meta_campaign_id, utm_source, utm_placement, utm_medium, utm_content, utm_adset, utm_whop, wacid, wasid, waid, tw_source, tw_adid; TikTok: waid, wasid, wacid, ad_id, adset_id, campaign_id, utm_source, utm_medium, utm_placement, utm_whop, tw_source, tw_adid.
    #[serde(default)]
    pub url_parameters: HashMap<String, serde_json::Value>,
}

impl PostAdUpdatedPayloadData {
    pub fn builder() -> PostAdUpdatedPayloadDataBuilder {
        <PostAdUpdatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostAdUpdatedPayloadDataBuilder {
    ad_campaign: Option<AdEntityReference>,
    ad_group: Option<AdEntityReference>,
    call_to_action: Option<PostAdUpdatedPayloadDataCallToAction>,
    created_at: Option<String>,
    creatives: Option<Vec<AdCreative>>,
    delivery_status: Option<PostAdUpdatedPayloadDataDeliveryStatus>,
    descriptions: Option<Vec<String>>,
    existing_post_id: Option<String>,
    headlines: Option<Vec<String>>,
    id: Option<String>,
    issues: Option<Vec<AdPlatformIssue>>,
    lead_form: Option<AdLeadForm>,
    lead_form_id: Option<String>,
    messaging_config: Option<AdMessagingConfig>,
    multi_advertiser_ads: Option<bool>,
    music: Option<AdMusic>,
    post_id: Option<String>,
    post_source: Option<PostAdUpdatedPayloadDataPostSource>,
    post_thumbnail_url: Option<String>,
    primary_texts: Option<Vec<String>>,
    social_accounts: Option<Vec<AdEntityReference>>,
    status: Option<PostAdUpdatedPayloadDataStatus>,
    title: Option<String>,
    updated_at: Option<String>,
    url: Option<String>,
    url_parameters: Option<HashMap<String, serde_json::Value>>,
}

impl PostAdUpdatedPayloadDataBuilder {
    pub fn ad_campaign(mut self, value: AdEntityReference) -> Self {
        self.ad_campaign = Some(value);
        self
    }

    pub fn ad_group(mut self, value: AdEntityReference) -> Self {
        self.ad_group = Some(value);
        self
    }

    pub fn call_to_action(mut self, value: PostAdUpdatedPayloadDataCallToAction) -> Self {
        self.call_to_action = Some(value);
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

    pub fn delivery_status(mut self, value: PostAdUpdatedPayloadDataDeliveryStatus) -> Self {
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

    pub fn headlines(mut self, value: Vec<String>) -> Self {
        self.headlines = Some(value);
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

    pub fn lead_form(mut self, value: AdLeadForm) -> Self {
        self.lead_form = Some(value);
        self
    }

    pub fn lead_form_id(mut self, value: impl Into<String>) -> Self {
        self.lead_form_id = Some(value.into());
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

    pub fn music(mut self, value: AdMusic) -> Self {
        self.music = Some(value);
        self
    }

    pub fn post_id(mut self, value: impl Into<String>) -> Self {
        self.post_id = Some(value.into());
        self
    }

    pub fn post_source(mut self, value: PostAdUpdatedPayloadDataPostSource) -> Self {
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

    pub fn social_accounts(mut self, value: Vec<AdEntityReference>) -> Self {
        self.social_accounts = Some(value);
        self
    }

    pub fn status(mut self, value: PostAdUpdatedPayloadDataStatus) -> Self {
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

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn url_parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.url_parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostAdUpdatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_campaign`](PostAdUpdatedPayloadDataBuilder::ad_campaign)
    /// - [`ad_group`](PostAdUpdatedPayloadDataBuilder::ad_group)
    /// - [`created_at`](PostAdUpdatedPayloadDataBuilder::created_at)
    /// - [`creatives`](PostAdUpdatedPayloadDataBuilder::creatives)
    /// - [`delivery_status`](PostAdUpdatedPayloadDataBuilder::delivery_status)
    /// - [`descriptions`](PostAdUpdatedPayloadDataBuilder::descriptions)
    /// - [`headlines`](PostAdUpdatedPayloadDataBuilder::headlines)
    /// - [`id`](PostAdUpdatedPayloadDataBuilder::id)
    /// - [`issues`](PostAdUpdatedPayloadDataBuilder::issues)
    /// - [`primary_texts`](PostAdUpdatedPayloadDataBuilder::primary_texts)
    /// - [`social_accounts`](PostAdUpdatedPayloadDataBuilder::social_accounts)
    /// - [`status`](PostAdUpdatedPayloadDataBuilder::status)
    /// - [`updated_at`](PostAdUpdatedPayloadDataBuilder::updated_at)
    /// - [`url_parameters`](PostAdUpdatedPayloadDataBuilder::url_parameters)
    pub fn build(self) -> Result<PostAdUpdatedPayloadData, BuildError> {
        Ok(PostAdUpdatedPayloadData {
            ad_campaign: self
                .ad_campaign
                .ok_or_else(|| BuildError::missing_field("ad_campaign"))?,
            ad_group: self
                .ad_group
                .ok_or_else(|| BuildError::missing_field("ad_group"))?,
            call_to_action: self.call_to_action,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            creatives: self
                .creatives
                .ok_or_else(|| BuildError::missing_field("creatives"))?,
            delivery_status: self
                .delivery_status
                .ok_or_else(|| BuildError::missing_field("delivery_status"))?,
            descriptions: self
                .descriptions
                .ok_or_else(|| BuildError::missing_field("descriptions"))?,
            existing_post_id: self.existing_post_id,
            headlines: self
                .headlines
                .ok_or_else(|| BuildError::missing_field("headlines"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            lead_form: self.lead_form,
            lead_form_id: self.lead_form_id,
            messaging_config: self.messaging_config,
            multi_advertiser_ads: self.multi_advertiser_ads,
            music: self.music,
            post_id: self.post_id,
            post_source: self.post_source,
            post_thumbnail_url: self.post_thumbnail_url,
            primary_texts: self
                .primary_texts
                .ok_or_else(|| BuildError::missing_field("primary_texts"))?,
            social_accounts: self
                .social_accounts
                .ok_or_else(|| BuildError::missing_field("social_accounts"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self.title,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            url: self.url,
            url_parameters: self
                .url_parameters
                .ok_or_else(|| BuildError::missing_field("url_parameters"))?,
        })
    }
}
