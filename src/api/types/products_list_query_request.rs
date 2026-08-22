pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductsListQueryRequest {
    /// The unique identifier of the account to list products for.
    #[serde(default)]
    pub account_id: String,
    /// Filter to only products matching these visibility states.
    #[serde(default)]
    pub visibilities: Vec<Option<String>>,
    /// Filter to only products matching these types.
    #[serde(default)]
    pub access_pass_types: Vec<Option<String>>,
    /// Filter to only products carrying all of these labels. Labels are matched lowercased.
    #[serde(default)]
    pub labels: Vec<Option<String>>,
    /// The sort direction for results. Defaults to descending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListProductsRequestDirection>,
    /// The field to sort results by. Defaults to created_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// The number of products to return (default and max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns products after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of products to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns products before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl ProductsListQueryRequest {
    pub fn builder() -> ProductsListQueryRequestBuilder {
        <ProductsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductsListQueryRequestBuilder {
    account_id: Option<String>,
    visibilities: Option<Vec<Option<String>>>,
    access_pass_types: Option<Vec<Option<String>>>,
    labels: Option<Vec<Option<String>>>,
    direction: Option<ListProductsRequestDirection>,
    order: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl ProductsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn visibilities(mut self, value: Vec<Option<String>>) -> Self {
        self.visibilities = Some(value);
        self
    }

    pub fn access_pass_types(mut self, value: Vec<Option<String>>) -> Self {
        self.access_pass_types = Some(value);
        self
    }

    pub fn labels(mut self, value: Vec<Option<String>>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn direction(mut self, value: ListProductsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn order(mut self, value: impl Into<String>) -> Self {
        self.order = Some(value.into());
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

    /// Consumes the builder and constructs a [`ProductsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ProductsListQueryRequestBuilder::account_id)
    /// - [`visibilities`](ProductsListQueryRequestBuilder::visibilities)
    /// - [`access_pass_types`](ProductsListQueryRequestBuilder::access_pass_types)
    /// - [`labels`](ProductsListQueryRequestBuilder::labels)
    pub fn build(self) -> Result<ProductsListQueryRequest, BuildError> {
        Ok(ProductsListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            visibilities: self
                .visibilities
                .ok_or_else(|| BuildError::missing_field("visibilities"))?,
            access_pass_types: self
                .access_pass_types
                .ok_or_else(|| BuildError::missing_field("access_pass_types"))?,
            labels: self
                .labels
                .ok_or_else(|| BuildError::missing_field("labels"))?,
            direction: self.direction,
            order: self.order,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
