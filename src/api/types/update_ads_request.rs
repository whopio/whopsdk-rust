pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAdsRequest {
    /// The call-to-action button shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<UpdateAdsRequestCallToAction>,
    /// The ad's creative assets. Each entry is an uploaded file id with an optional format; omit format for the original asset. Replaces a live ad's creative. Entries with no format replace it with a carousel's ordered cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creatives: Option<Vec<UpdateAdsRequestCreativesItem>>,
    /// The description shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<UpdateAdsRequestDescriptionsItem>>,
    /// The headline shown on the ad.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headlines: Option<Vec<UpdateAdsRequestHeadlinesItem>>,
    /// The primary text shown in the ad body. Several entries make text variations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_texts: Option<Vec<UpdateAdsRequestPrimaryTextsItem>>,
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

impl UpdateAdsRequest {
    pub fn builder() -> UpdateAdsRequestBuilder {
        <UpdateAdsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestBuilder {
    call_to_action: Option<UpdateAdsRequestCallToAction>,
    creatives: Option<Vec<UpdateAdsRequestCreativesItem>>,
    descriptions: Option<Vec<UpdateAdsRequestDescriptionsItem>>,
    headlines: Option<Vec<UpdateAdsRequestHeadlinesItem>>,
    primary_texts: Option<Vec<UpdateAdsRequestPrimaryTextsItem>>,
    title: Option<String>,
    url: Option<String>,
    url_parameters: Option<HashMap<String, serde_json::Value>>,
}

impl UpdateAdsRequestBuilder {
    pub fn call_to_action(mut self, value: UpdateAdsRequestCallToAction) -> Self {
        self.call_to_action = Some(value);
        self
    }

    pub fn creatives(mut self, value: Vec<UpdateAdsRequestCreativesItem>) -> Self {
        self.creatives = Some(value);
        self
    }

    pub fn descriptions(mut self, value: Vec<UpdateAdsRequestDescriptionsItem>) -> Self {
        self.descriptions = Some(value);
        self
    }

    pub fn headlines(mut self, value: Vec<UpdateAdsRequestHeadlinesItem>) -> Self {
        self.headlines = Some(value);
        self
    }

    pub fn primary_texts(mut self, value: Vec<UpdateAdsRequestPrimaryTextsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateAdsRequest`].
    pub fn build(self) -> Result<UpdateAdsRequest, BuildError> {
        Ok(UpdateAdsRequest {
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
