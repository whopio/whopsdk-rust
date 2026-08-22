pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersOauthGrantsListQueryRequest {
    /// Only return grants for this app, prefixed `app_`. An app the user has never authorized returns an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The number of grants to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns grants after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of grants to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns grants before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort grants by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListOauthGrantsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListOauthGrantsRequestDirection>,
}

impl UsersOauthGrantsListQueryRequest {
    pub fn builder() -> UsersOauthGrantsListQueryRequestBuilder {
        <UsersOauthGrantsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersOauthGrantsListQueryRequestBuilder {
    app_id: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListOauthGrantsRequestOrder>,
    direction: Option<ListOauthGrantsRequestDirection>,
}

impl UsersOauthGrantsListQueryRequestBuilder {
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
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

    pub fn order(mut self, value: ListOauthGrantsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListOauthGrantsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersOauthGrantsListQueryRequest`].
    pub fn build(self) -> Result<UsersOauthGrantsListQueryRequest, BuildError> {
        Ok(UsersOauthGrantsListQueryRequest {
            app_id: self.app_id,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
