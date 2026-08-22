pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransfersListQueryRequest {
    /// Filter to transfers sent from this account. Provide this or destination_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    /// Filter to transfers received by this account. Provide this or origin_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_id: Option<String>,
    /// Sort column. Defaults to created_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListTransfersRequestOrder>,
    /// Sort direction. Defaults to desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListTransfersRequestDirection>,
    /// Only transfers created strictly before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only transfers created strictly after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Number of transfers to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of transfers to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl TransfersListQueryRequest {
    pub fn builder() -> TransfersListQueryRequestBuilder {
        <TransfersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransfersListQueryRequestBuilder {
    origin_id: Option<String>,
    destination_id: Option<String>,
    order: Option<ListTransfersRequestOrder>,
    direction: Option<ListTransfersRequestDirection>,
    created_before: Option<String>,
    created_after: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl TransfersListQueryRequestBuilder {
    pub fn origin_id(mut self, value: impl Into<String>) -> Self {
        self.origin_id = Some(value.into());
        self
    }

    pub fn destination_id(mut self, value: impl Into<String>) -> Self {
        self.destination_id = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListTransfersRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListTransfersRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
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

    /// Consumes the builder and constructs a [`TransfersListQueryRequest`].
    pub fn build(self) -> Result<TransfersListQueryRequest, BuildError> {
        Ok(TransfersListQueryRequest {
            origin_id: self.origin_id,
            destination_id: self.destination_id,
            order: self.order,
            direction: self.direction,
            created_before: self.created_before,
            created_after: self.created_after,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
