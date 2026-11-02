pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersPasskeysListQueryRequest {
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
    /// The field to sort passkeys by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPasskeysRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPasskeysRequestDirection>,
}

impl UsersPasskeysListQueryRequest {
    pub fn builder() -> UsersPasskeysListQueryRequestBuilder {
        <UsersPasskeysListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersPasskeysListQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListPasskeysRequestOrder>,
    direction: Option<ListPasskeysRequestDirection>,
}

impl UsersPasskeysListQueryRequestBuilder {
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

    pub fn order(mut self, value: ListPasskeysRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListPasskeysRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsersPasskeysListQueryRequest`].
    pub fn build(self) -> Result<UsersPasskeysListQueryRequest, BuildError> {
        Ok(UsersPasskeysListQueryRequest {
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
