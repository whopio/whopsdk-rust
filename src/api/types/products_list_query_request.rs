pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProductsListQueryRequest {
    /// The unique identifier of the account to list products for. Omit to search the public marketplace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Ranked search against product title and headline. Omit to browse by recency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only return marketplace products assigned to this category route, such as `trading`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_category_route: Option<String>,
    /// Filter to products with a buyable plan of these billing models, such as `one_time` or `renewal`.
    #[serde(default)]
    pub plan_types: Vec<Option<ListProductsRequestPlanTypesItem>>,
    /// Only return products whose advertised buyable plan has a displayed price of at least this amount. Recurring plans use renewal price.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_minimum: Option<f64>,
    /// Only return products whose advertised buyable plan has a displayed price of at most this amount. Recurring plans use renewal price.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub price_maximum: Option<f64>,
    /// Filter to only products matching these visibility states. Ignored on the public marketplace list, which only returns visible products.
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
    /// The field to sort results by. Account lists default to `created_at`. Marketplace lists default to `discoverable_at` and accept `created_at` or `discoverable_at`. Cannot be combined with `query`.
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
    plan_types: Option<Vec<Option<ListProductsRequestPlanTypesItem>>>,
    price_minimum: Option<f64>,
    price_maximum: Option<f64>,
    visibilities: Option<Vec<Option<String>>>,
    access_pass_types: Option<Vec<Option<String>>>,
    labels: Option<Vec<Option<String>>>,
    direction: Option<ListProductsRequestDirection>,
    order: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
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

    pub fn plan_types(mut self, value: Vec<Option<ListProductsRequestPlanTypesItem>>) -> Self {
        self.plan_types = Some(value);
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
    /// - [`plan_types`](ProductsListQueryRequestBuilder::plan_types)
    /// - [`visibilities`](ProductsListQueryRequestBuilder::visibilities)
    /// - [`access_pass_types`](ProductsListQueryRequestBuilder::access_pass_types)
    /// - [`labels`](ProductsListQueryRequestBuilder::labels)
    pub fn build(self) -> Result<ProductsListQueryRequest, BuildError> {
        Ok(ProductsListQueryRequest {
            account_id: self.account_id,
            query: self.query,
            marketplace_category_route: self.marketplace_category_route,
            plan_types: self
                .plan_types
                .ok_or_else(|| BuildError::missing_field("plan_types"))?,
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
            created_after: self.created_after,
            created_before: self.created_before,
        })
    }
}
