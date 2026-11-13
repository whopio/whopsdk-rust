pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersListQueryRequest {
    /// Prioritize people relevant to this channel for mentions. Results remain global; channel access is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_channel_id: Option<String>,
    /// A search term to filter users by name or username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
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

impl UsersListQueryRequest {
    pub fn builder() -> UsersListQueryRequestBuilder {
        <UsersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersListQueryRequestBuilder {
    context_channel_id: Option<String>,
    query: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl UsersListQueryRequestBuilder {
    pub fn context_channel_id(mut self, value: impl Into<String>) -> Self {
        self.context_channel_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
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

    /// Consumes the builder and constructs a [`UsersListQueryRequest`].
    pub fn build(self) -> Result<UsersListQueryRequest, BuildError> {
        Ok(UsersListQueryRequest {
            context_channel_id: self.context_channel_id,
            query: self.query,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
