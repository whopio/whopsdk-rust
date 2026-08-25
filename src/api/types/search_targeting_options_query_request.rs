pub use crate::prelude::*;

/// Query parameters for searchTargetingOptions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SearchTargetingOptionsQueryRequest {
    /// Account to search on behalf of. Defaults to the authenticated account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The ad network whose targeting taxonomy to search.
    pub platform: SearchTargetingOptionsAdGroupsRequestPlatform,
    /// The search term. Blank browses the fixed lists; interests, work employers, job titles, schools, majors, and locations return nothing without one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Kinds of targeting options to search. Defaults to all of them.
    #[serde(default)]
    pub types: Vec<Option<SearchTargetingOptionsAdGroupsRequestTypesItem>>,
    /// Narrow location results to these kinds of places. Only applies when `types` includes `locations`.
    #[serde(default)]
    pub location_types: Vec<Option<SearchTargetingOptionsAdGroupsRequestLocationTypesItem>>,
    /// Narrow location results to one country, as an ISO 3166-1 code such as `US`. Only applies when `types` includes `locations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Maximum number of results per requested type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The campaign's declared special ad categories. Under `housing`, `employment`, or `financial_products` the ad platform allows interests only, drawn from a short approved list, so results are narrowed to what such a campaign can launch with and other kinds return nothing. Blank `query` browses that approved list instead of the usual fixed lists.
    #[serde(default)]
    pub special_ad_categories:
        Vec<Option<SearchTargetingOptionsAdGroupsRequestSpecialAdCategoriesItem>>,
}

impl SearchTargetingOptionsQueryRequest {
    pub fn builder() -> SearchTargetingOptionsQueryRequestBuilder {
        <SearchTargetingOptionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchTargetingOptionsQueryRequestBuilder {
    account_id: Option<String>,
    platform: Option<SearchTargetingOptionsAdGroupsRequestPlatform>,
    query: Option<String>,
    types: Option<Vec<Option<SearchTargetingOptionsAdGroupsRequestTypesItem>>>,
    location_types: Option<Vec<Option<SearchTargetingOptionsAdGroupsRequestLocationTypesItem>>>,
    country: Option<String>,
    limit: Option<i64>,
    special_ad_categories:
        Option<Vec<Option<SearchTargetingOptionsAdGroupsRequestSpecialAdCategoriesItem>>>,
}

impl SearchTargetingOptionsQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: SearchTargetingOptionsAdGroupsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn types(
        mut self,
        value: Vec<Option<SearchTargetingOptionsAdGroupsRequestTypesItem>>,
    ) -> Self {
        self.types = Some(value);
        self
    }

    pub fn location_types(
        mut self,
        value: Vec<Option<SearchTargetingOptionsAdGroupsRequestLocationTypesItem>>,
    ) -> Self {
        self.location_types = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn special_ad_categories(
        mut self,
        value: Vec<Option<SearchTargetingOptionsAdGroupsRequestSpecialAdCategoriesItem>>,
    ) -> Self {
        self.special_ad_categories = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchTargetingOptionsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](SearchTargetingOptionsQueryRequestBuilder::platform)
    /// - [`types`](SearchTargetingOptionsQueryRequestBuilder::types)
    /// - [`location_types`](SearchTargetingOptionsQueryRequestBuilder::location_types)
    /// - [`special_ad_categories`](SearchTargetingOptionsQueryRequestBuilder::special_ad_categories)
    pub fn build(self) -> Result<SearchTargetingOptionsQueryRequest, BuildError> {
        Ok(SearchTargetingOptionsQueryRequest {
            account_id: self.account_id,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            query: self.query,
            types: self
                .types
                .ok_or_else(|| BuildError::missing_field("types"))?,
            location_types: self
                .location_types
                .ok_or_else(|| BuildError::missing_field("location_types"))?,
            country: self.country,
            limit: self.limit,
            special_ad_categories: self
                .special_ad_categories
                .ok_or_else(|| BuildError::missing_field("special_ad_categories"))?,
        })
    }
}
