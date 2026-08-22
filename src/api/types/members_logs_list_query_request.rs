pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersLogsListQueryRequest {
    /// Number of log entries to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to paginate forwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of log entries to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl MembersLogsListQueryRequest {
    pub fn builder() -> MembersLogsListQueryRequestBuilder {
        <MembersLogsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersLogsListQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl MembersLogsListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`MembersLogsListQueryRequest`].
    pub fn build(self) -> Result<MembersLogsListQueryRequest, BuildError> {
        Ok(MembersLogsListQueryRequest {
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
