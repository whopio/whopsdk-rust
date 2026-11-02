pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffiliatesListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<AffiliatesSortableColumns>,
    /// Search affiliates by username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The unique identifier of the company to list affiliates for.
    #[serde(default)]
    pub account_id: String,
}

impl AffiliatesListQueryRequest {
    pub fn builder() -> AffiliatesListQueryRequestBuilder {
        <AffiliatesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffiliatesListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    direction: Option<Direction>,
    order: Option<AffiliatesSortableColumns>,
    query: Option<String>,
    status: Option<Status>,
    account_id: Option<String>,
}

impl AffiliatesListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn order(mut self, value: AffiliatesSortableColumns) -> Self {
        self.order = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn status(mut self, value: Status) -> Self {
        self.status = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AffiliatesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AffiliatesListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<AffiliatesListQueryRequest, BuildError> {
        Ok(AffiliatesListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            direction: self.direction,
            order: self.order,
            query: self.query,
            status: self.status,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
