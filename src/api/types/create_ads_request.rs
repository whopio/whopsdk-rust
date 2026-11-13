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
    /// The ad's creative assets. Each entry is an uploaded file id with an optional format; omit format for the original asset. Entries with no format become a carousel's ordered cards, sharing the ad's copy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creatives: Option<Vec<CreateAdsRequestCreativesItem>>,
    /// The description shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<CreateAdsRequestDescriptionsItem>>,
    /// The headline shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headlines: Option<Vec<CreateAdsRequestHeadlinesItem>>,
    /// The primary text shown in the ad body. Several entries make text variations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_texts: Option<Vec<CreateAdsRequestPrimaryTextsItem>>,
    /// The display name of the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL the ad links to. Query parameters are merged into url_parameters, so the stored URL is always bare.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Query parameters to append to the destination URL, keyed by parameter name. Merged with any query string on `url`. Whop adds its own click-attribution parameters; those are reserved, and setting one returns an error that names it.
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
    descriptions: Option<Vec<CreateAdsRequestDescriptionsItem>>,
    headlines: Option<Vec<CreateAdsRequestHeadlinesItem>>,
    primary_texts: Option<Vec<CreateAdsRequestPrimaryTextsItem>>,
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

    pub fn descriptions(mut self, value: Vec<CreateAdsRequestDescriptionsItem>) -> Self {
        self.descriptions = Some(value);
        self
    }

    pub fn headlines(mut self, value: Vec<CreateAdsRequestHeadlinesItem>) -> Self {
        self.headlines = Some(value);
        self
    }

    pub fn primary_texts(mut self, value: Vec<CreateAdsRequestPrimaryTextsItem>) -> Self {
        self.primary_texts = Some(value);
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
            headlines: self.headlines,
            primary_texts: self.primary_texts,
            title: self.title,
            url: self.url,
            url_parameters: self.url_parameters,
        })
    }
}
