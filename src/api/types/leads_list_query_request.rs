pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeadsListQueryRequest {
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
    /// Only return leads created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Only return leads created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Filter leads to only those associated with these specific product identifiers.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// The unique identifier of the company to list leads for.
    #[serde(default)]
    pub account_id: String,
}

impl LeadsListQueryRequest {
    pub fn builder() -> LeadsListQueryRequestBuilder {
        <LeadsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    created_after: Option<DateTime<FixedOffset>>,
    created_before: Option<DateTime<FixedOffset>>,
    product_ids: Option<Vec<Option<String>>>,
    account_id: Option<String>,
}

impl LeadsListQueryRequestBuilder {
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

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeadsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_ids`](LeadsListQueryRequestBuilder::product_ids)
    /// - [`account_id`](LeadsListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<LeadsListQueryRequest, BuildError> {
        Ok(LeadsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            created_after: self.created_after,
            created_before: self.created_before,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
