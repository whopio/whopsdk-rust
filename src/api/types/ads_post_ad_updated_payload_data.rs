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
    /// Whop's content review of this ad: whether it has finished, which ad networks it is recommended for, and the content rules it breaks.
    pub content_review: AdContentReview,
    /// When the ad was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub creatives: Vec<AdCreative>,
    /// Whether the ad is delivering right now, and if not, why. When several states apply at once, the highest-precedence one is returned.
    pub delivery_status: PostAdUpdatedPayloadDataDeliveryStatus,
    #[serde(default)]
    pub descriptions: Vec<AdText>,
    #[serde(default)]
    pub headlines: Vec<AdText>,
    /// Unique identifier for the ad, prefixed `ad_`.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub issues: Vec<AdPlatformIssue>,
    #[serde(default)]
    pub primary_texts: Vec<AdText>,
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
    /// Query parameters appended to `url`, keyed by name. Whop's click-attribution parameters are reserved.
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
    content_review: Option<AdContentReview>,
    created_at: Option<String>,
    creatives: Option<Vec<AdCreative>>,
    delivery_status: Option<PostAdUpdatedPayloadDataDeliveryStatus>,
    descriptions: Option<Vec<AdText>>,
    headlines: Option<Vec<AdText>>,
    id: Option<String>,
    issues: Option<Vec<AdPlatformIssue>>,
    primary_texts: Option<Vec<AdText>>,
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

    pub fn content_review(mut self, value: AdContentReview) -> Self {
        self.content_review = Some(value);
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

    pub fn descriptions(mut self, value: Vec<AdText>) -> Self {
        self.descriptions = Some(value);
        self
    }

    pub fn headlines(mut self, value: Vec<AdText>) -> Self {
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

    pub fn primary_texts(mut self, value: Vec<AdText>) -> Self {
        self.primary_texts = Some(value);
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
    /// - [`content_review`](PostAdUpdatedPayloadDataBuilder::content_review)
    /// - [`created_at`](PostAdUpdatedPayloadDataBuilder::created_at)
    /// - [`creatives`](PostAdUpdatedPayloadDataBuilder::creatives)
    /// - [`delivery_status`](PostAdUpdatedPayloadDataBuilder::delivery_status)
    /// - [`descriptions`](PostAdUpdatedPayloadDataBuilder::descriptions)
    /// - [`headlines`](PostAdUpdatedPayloadDataBuilder::headlines)
    /// - [`id`](PostAdUpdatedPayloadDataBuilder::id)
    /// - [`issues`](PostAdUpdatedPayloadDataBuilder::issues)
    /// - [`primary_texts`](PostAdUpdatedPayloadDataBuilder::primary_texts)
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
            content_review: self
                .content_review
                .ok_or_else(|| BuildError::missing_field("content_review"))?,
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
            headlines: self
                .headlines
                .ok_or_else(|| BuildError::missing_field("headlines"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            primary_texts: self
                .primary_texts
                .ok_or_else(|| BuildError::missing_field("primary_texts"))?,
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
