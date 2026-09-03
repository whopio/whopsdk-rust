pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntriesListQueryRequest {
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
    pub order: Option<EntriesSortableColumns>,
    /// Filter entries to only those for specific products.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Filter entries to only those for specific plans.
    #[serde(default)]
    pub plan_ids: Vec<Option<String>>,
    /// Filter entries by their current status.
    #[serde(default)]
    pub statuses: Vec<Option<EntryStatus>>,
    /// Only return entries created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only return entries created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// The unique identifier of the company to list waitlist entries for.
    #[serde(default)]
    pub account_id: String,
}

impl EntriesListQueryRequest {
    pub fn builder() -> EntriesListQueryRequestBuilder {
        <EntriesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntriesListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    direction: Option<Direction>,
    order: Option<EntriesSortableColumns>,
    product_ids: Option<Vec<Option<String>>>,
    plan_ids: Option<Vec<Option<String>>>,
    statuses: Option<Vec<Option<EntryStatus>>>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    account_id: Option<String>,
}

impl EntriesListQueryRequestBuilder {
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

    pub fn order(mut self, value: EntriesSortableColumns) -> Self {
        self.order = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn plan_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.plan_ids = Some(value);
        self
    }

    pub fn statuses(mut self, value: Vec<Option<EntryStatus>>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntriesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_ids`](EntriesListQueryRequestBuilder::product_ids)
    /// - [`plan_ids`](EntriesListQueryRequestBuilder::plan_ids)
    /// - [`statuses`](EntriesListQueryRequestBuilder::statuses)
    /// - [`account_id`](EntriesListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<EntriesListQueryRequest, BuildError> {
        Ok(EntriesListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            direction: self.direction,
            order: self.order,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            plan_ids: self
                .plan_ids
                .ok_or_else(|| BuildError::missing_field("plan_ids"))?,
            statuses: self
                .statuses
                .ok_or_else(|| BuildError::missing_field("statuses"))?,
            created_before: self.created_before,
            created_after: self.created_after,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
