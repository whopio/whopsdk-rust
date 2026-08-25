pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PlansListQueryRequest {
    /// The unique identifier of the account to list plans for. Required unless `product_ids` is provided for a public product-plan read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The sort direction for results. Defaults to descending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPlansRequestDirection>,
    /// The field to sort results by. Defaults to created_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPlansRequestOrder>,
    /// Filter to only plans matching these release methods.
    #[serde(default)]
    pub release_methods: Vec<Option<String>>,
    /// Filter to only plans matching these visibility states.
    #[serde(default)]
    pub visibilities: Vec<Option<String>>,
    /// Filter to only plans matching these billing types.
    #[serde(default)]
    pub plan_types: Vec<Option<String>>,
    /// Filter to only plans belonging to these product identifiers. When `account_id` is omitted, this is required and the response is publicly readable: only visible, non-invoice plans are returned.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Only return plans created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return plans created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// The number of plans to return (default and max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns plans after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of plans to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns plans before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl PlansListQueryRequest {
    pub fn builder() -> PlansListQueryRequestBuilder {
        <PlansListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlansListQueryRequestBuilder {
    account_id: Option<String>,
    direction: Option<ListPlansRequestDirection>,
    order: Option<ListPlansRequestOrder>,
    release_methods: Option<Vec<Option<String>>>,
    visibilities: Option<Vec<Option<String>>>,
    plan_types: Option<Vec<Option<String>>>,
    product_ids: Option<Vec<Option<String>>>,
    created_before: Option<String>,
    created_after: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PlansListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn direction(mut self, value: ListPlansRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn order(mut self, value: ListPlansRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn release_methods(mut self, value: Vec<Option<String>>) -> Self {
        self.release_methods = Some(value);
        self
    }

    pub fn visibilities(mut self, value: Vec<Option<String>>) -> Self {
        self.visibilities = Some(value);
        self
    }

    pub fn plan_types(mut self, value: Vec<Option<String>>) -> Self {
        self.plan_types = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
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

    /// Consumes the builder and constructs a [`PlansListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`release_methods`](PlansListQueryRequestBuilder::release_methods)
    /// - [`visibilities`](PlansListQueryRequestBuilder::visibilities)
    /// - [`plan_types`](PlansListQueryRequestBuilder::plan_types)
    /// - [`product_ids`](PlansListQueryRequestBuilder::product_ids)
    pub fn build(self) -> Result<PlansListQueryRequest, BuildError> {
        Ok(PlansListQueryRequest {
            account_id: self.account_id,
            direction: self.direction,
            order: self.order,
            release_methods: self
                .release_methods
                .ok_or_else(|| BuildError::missing_field("release_methods"))?,
            visibilities: self
                .visibilities
                .ok_or_else(|| BuildError::missing_field("visibilities"))?,
            plan_types: self
                .plan_types
                .ok_or_else(|| BuildError::missing_field("plan_types"))?,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            created_before: self.created_before,
            created_after: self.created_after,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
