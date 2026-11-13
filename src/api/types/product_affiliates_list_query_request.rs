pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductAffiliatesListQueryRequest {
    /// Account whose product affiliates are listed (`biz_` tag).
    #[serde(default)]
    pub account_id: String,
    /// Matches the start of the affiliate's username or of any word in their name, or their exact email (with `member:email:read`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only product affiliates with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListProductAffiliatesRequestStatus>,
    /// Only product affiliates of these products.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Only signups at or after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Only signups before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
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

impl ProductAffiliatesListQueryRequest {
    pub fn builder() -> ProductAffiliatesListQueryRequestBuilder {
        <ProductAffiliatesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductAffiliatesListQueryRequestBuilder {
    account_id: Option<String>,
    query: Option<String>,
    status: Option<ListProductAffiliatesRequestStatus>,
    product_ids: Option<Vec<Option<String>>>,
    created_after: Option<DateTime<FixedOffset>>,
    created_before: Option<DateTime<FixedOffset>>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl ProductAffiliatesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListProductAffiliatesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
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

    /// Consumes the builder and constructs a [`ProductAffiliatesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ProductAffiliatesListQueryRequestBuilder::account_id)
    /// - [`product_ids`](ProductAffiliatesListQueryRequestBuilder::product_ids)
    pub fn build(self) -> Result<ProductAffiliatesListQueryRequest, BuildError> {
        Ok(ProductAffiliatesListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            query: self.query,
            status: self.status,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            created_after: self.created_after,
            created_before: self.created_before,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
