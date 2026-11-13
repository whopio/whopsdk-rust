pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProductsListQueryRequest {
    /// Account to list products for, prefixed `biz_`. Without the `access_pass:basic:read` permission on it, only the products its store page lists are returned. Omit to search the public marketplace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Filters products by text. The public marketplace list searches product title, account name, headline and description, ranking products that match every word first and the closest partial matches after them; an account's own list is a plain substring match over title, headline and description. Omit to browse by recency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only return marketplace products assigned to this category route, such as `trading`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_category_route: Option<String>,
    /// Filter to products with a buyable variant of these billing models, such as `one_time` or `renewal`. On an account list, needs the `access_pass:basic:read` permission.
    #[serde(default)]
    pub billing_types: Vec<Option<ListProductsRequestBillingTypesItem>>,
    /// Only return products whose advertised buyable variant has a displayed price of at least this amount. Recurring variants use renewal price. On an account list, needs the `access_pass:basic:read` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_minimum: Option<f64>,
    /// Only return products whose advertised buyable variant has a displayed price of at most this amount. Recurring variants use renewal price. On an account list, needs the `access_pass:basic:read` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_maximum: Option<f64>,
    /// Filter to only products matching these visibility states. Ignored on the public marketplace list, which only returns visible products. Without the `access_pass:basic:read` permission on the account, only `visible` matches anything.
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
    /// Field to sort results by. Account lists default to `created_at`, and `position` with `direction` `asc` matches the order the store page shows. Without the `access_pass:basic:read` permission, an account list sorts only by `created_at` or `position`. Marketplace lists default to `discoverable_at`, accept only `created_at` or `discoverable_at`, and can't be combined with `query`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListProductsRequestOrder>,
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
    /// Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total_count: Option<bool>,
    /// Only return products created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only return products created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
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
    query: Option<String>,
    marketplace_category_route: Option<String>,
    billing_types: Option<Vec<Option<ListProductsRequestBillingTypesItem>>>,
    price_minimum: Option<f64>,
    price_maximum: Option<f64>,
    visibilities: Option<Vec<Option<String>>>,
    access_pass_types: Option<Vec<Option<String>>>,
    labels: Option<Vec<Option<String>>>,
    direction: Option<ListProductsRequestDirection>,
    order: Option<ListProductsRequestOrder>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    include_total_count: Option<bool>,
    created_after: Option<String>,
    created_before: Option<String>,
}

impl ProductsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn marketplace_category_route(mut self, value: impl Into<String>) -> Self {
        self.marketplace_category_route = Some(value.into());
        self
    }

    pub fn billing_types(
        mut self,
        value: Vec<Option<ListProductsRequestBillingTypesItem>>,
    ) -> Self {
        self.billing_types = Some(value);
        self
    }

    pub fn price_minimum(mut self, value: f64) -> Self {
        self.price_minimum = Some(value);
        self
    }

    pub fn price_maximum(mut self, value: f64) -> Self {
        self.price_maximum = Some(value);
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

    pub fn order(mut self, value: ListProductsRequestOrder) -> Self {
        self.order = Some(value);
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

    pub fn include_total_count(mut self, value: bool) -> Self {
        self.include_total_count = Some(value);
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

    /// Consumes the builder and constructs a [`ProductsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_types`](ProductsListQueryRequestBuilder::billing_types)
    /// - [`visibilities`](ProductsListQueryRequestBuilder::visibilities)
    /// - [`access_pass_types`](ProductsListQueryRequestBuilder::access_pass_types)
    /// - [`labels`](ProductsListQueryRequestBuilder::labels)
    pub fn build(self) -> Result<ProductsListQueryRequest, BuildError> {
        Ok(ProductsListQueryRequest {
            account_id: self.account_id,
            query: self.query,
            marketplace_category_route: self.marketplace_category_route,
            billing_types: self
                .billing_types
                .ok_or_else(|| BuildError::missing_field("billing_types"))?,
            price_minimum: self.price_minimum,
            price_maximum: self.price_maximum,
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
            include_total_count: self.include_total_count,
            created_after: self.created_after,
            created_before: self.created_before,
        })
    }
}
