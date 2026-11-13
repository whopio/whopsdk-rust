pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppsCategoriesListQueryRequest {
    /// Only return the categories of this app type's store, in that store's order: `b2b_app` for the business App Store, `b2c_app` for the community App Store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<ListCategoriesRequestAppType>,
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
}

impl AppsCategoriesListQueryRequest {
    pub fn builder() -> AppsCategoriesListQueryRequestBuilder {
        <AppsCategoriesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppsCategoriesListQueryRequestBuilder {
    app_type: Option<ListCategoriesRequestAppType>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl AppsCategoriesListQueryRequestBuilder {
    pub fn app_type(mut self, value: ListCategoriesRequestAppType) -> Self {
        self.app_type = Some(value);
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

    /// Consumes the builder and constructs a [`AppsCategoriesListQueryRequest`].
    pub fn build(self) -> Result<AppsCategoriesListQueryRequest, BuildError> {
        Ok(AppsCategoriesListQueryRequest {
            app_type: self.app_type,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
