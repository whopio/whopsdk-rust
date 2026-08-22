pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BountiesSubmissionsListQueryRequest {
    /// Filter by lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListSubmissionsRequestStatus>,
    /// Only submissions created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only submissions created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListSubmissionsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListSubmissionsRequestDirection>,
    /// Number of submissions to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to paginate forwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of submissions to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl BountiesSubmissionsListQueryRequest {
    pub fn builder() -> BountiesSubmissionsListQueryRequestBuilder {
        <BountiesSubmissionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountiesSubmissionsListQueryRequestBuilder {
    status: Option<ListSubmissionsRequestStatus>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListSubmissionsRequestOrder>,
    direction: Option<ListSubmissionsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl BountiesSubmissionsListQueryRequestBuilder {
    pub fn status(mut self, value: ListSubmissionsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListSubmissionsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListSubmissionsRequestDirection) -> Self {
        self.direction = Some(value);
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

    /// Consumes the builder and constructs a [`BountiesSubmissionsListQueryRequest`].
    pub fn build(self) -> Result<BountiesSubmissionsListQueryRequest, BuildError> {
        Ok(BountiesSubmissionsListQueryRequest {
            status: self.status,
            created_after: self.created_after,
            created_before: self.created_before,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
