pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipsListQueryRequest {
    /// Narrow to one account (`biz_` tag). With read access to the account this lists all of its memberships; without, only the caller's own memberships in it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Narrow to one user's memberships (`user_` tag, or `me` for the caller). A user outside the caller's visible set returns an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Filter by billing state. `canceling` matches active memberships set to cancel at period end; `paused` matches memberships with payment collection paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListMembershipsRequestStatus>,
    /// Filter to memberships of this product (`prod_` tag). Repeat as product_ids[] for several.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Filter to memberships of this plan (`plan_` tag). Repeat as plan_ids[] for several.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// Only memberships created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only memberships created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListMembershipsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListMembershipsRequestDirection>,
    /// Number of memberships to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to paginate forwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of memberships to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl MembershipsListQueryRequest {
    pub fn builder() -> MembershipsListQueryRequestBuilder {
        <MembershipsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipsListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    status: Option<ListMembershipsRequestStatus>,
    product_id: Option<String>,
    plan_id: Option<String>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListMembershipsRequestOrder>,
    direction: Option<ListMembershipsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl MembershipsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListMembershipsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
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

    pub fn order(mut self, value: ListMembershipsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListMembershipsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`MembershipsListQueryRequest`].
    pub fn build(self) -> Result<MembershipsListQueryRequest, BuildError> {
        Ok(MembershipsListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            status: self.status,
            product_id: self.product_id,
            plan_id: self.plan_id,
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
