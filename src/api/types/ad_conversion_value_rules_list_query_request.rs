pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdConversionValueRulesListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAdConversionValueRulesRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<ListAdConversionValueRulesRequestPlatform>,
    /// Campaign, ad group, or ad ID. Return rules covering this item, its ancestors, or its descendants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAdConversionValueRulesRequestOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAdConversionValueRulesRequestDirection>,
}

impl AdConversionValueRulesListQueryRequest {
    pub fn builder() -> AdConversionValueRulesListQueryRequestBuilder {
        <AdConversionValueRulesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdConversionValueRulesListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListAdConversionValueRulesRequestStatus>,
    platform: Option<ListAdConversionValueRulesRequestPlatform>,
    resource_id: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListAdConversionValueRulesRequestOrder>,
    direction: Option<ListAdConversionValueRulesRequestDirection>,
}

impl AdConversionValueRulesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListAdConversionValueRulesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn platform(mut self, value: ListAdConversionValueRulesRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListAdConversionValueRulesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAdConversionValueRulesRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdConversionValueRulesListQueryRequest`].
    pub fn build(self) -> Result<AdConversionValueRulesListQueryRequest, BuildError> {
        Ok(AdConversionValueRulesListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            platform: self.platform,
            resource_id: self.resource_id,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
