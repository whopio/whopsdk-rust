pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersListQueryRequest {
    /// The account to list members for (`biz_` tag). Defaults to the account the credential acts as.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Filter by what the member can reach on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<ListMembersRequestAccessLevel>,
    /// Filter by whether the member is still part of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListMembersRequestStatus>,
    /// Only return members whose users match these `user_` identifiers.
    #[serde(default)]
    pub user_ids: Vec<Option<String>>,
    /// Only return members whose `most_recent_action` is one of these values.
    #[serde(default)]
    pub most_recent_actions: Vec<Option<ListMembersRequestMostRecentActionsItem>>,
    /// `active` returns members holding at least one membership that currently grants access; `inactive` returns members holding at least one membership that does not. Combined with `product_ids`, `variant_ids`, or `promo_code_ids`, one membership must match all of them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_status: Option<ListMembersRequestMembershipStatus>,
    /// Only return members holding a membership to one of these products, prefixed `prod_`. Unknown IDs match nothing.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Only return members holding a membership on one of these variants, prefixed `plan_`. Unknown IDs match nothing.
    #[serde(default)]
    pub variant_ids: Vec<Option<String>>,
    /// Only return members holding a membership bought with one of these promo codes, prefixed `promo_`. Unknown IDs match nothing.
    #[serde(default)]
    pub promo_code_ids: Vec<Option<String>>,
    /// Search members by name or username. An exact email address also matches when the credential holds the member:email:read scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only members who joined after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only members who joined before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Sort field. `most_recent_action_at` and `last_activity_at` place members without one first when ascending and last when descending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListMembersRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListMembersRequestDirection>,
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
}

impl MembersListQueryRequest {
    pub fn builder() -> MembersListQueryRequestBuilder {
        <MembersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersListQueryRequestBuilder {
    account_id: Option<String>,
    access_level: Option<ListMembersRequestAccessLevel>,
    status: Option<ListMembersRequestStatus>,
    user_ids: Option<Vec<Option<String>>>,
    most_recent_actions: Option<Vec<Option<ListMembersRequestMostRecentActionsItem>>>,
    membership_status: Option<ListMembersRequestMembershipStatus>,
    product_ids: Option<Vec<Option<String>>>,
    variant_ids: Option<Vec<Option<String>>>,
    promo_code_ids: Option<Vec<Option<String>>>,
    query: Option<String>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListMembersRequestOrder>,
    direction: Option<ListMembersRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    include_total_count: Option<bool>,
}

impl MembersListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn access_level(mut self, value: ListMembersRequestAccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    pub fn status(mut self, value: ListMembersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.user_ids = Some(value);
        self
    }

    pub fn most_recent_actions(
        mut self,
        value: Vec<Option<ListMembersRequestMostRecentActionsItem>>,
    ) -> Self {
        self.most_recent_actions = Some(value);
        self
    }

    pub fn membership_status(mut self, value: ListMembersRequestMembershipStatus) -> Self {
        self.membership_status = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn variant_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    pub fn promo_code_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.promo_code_ids = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
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

    pub fn order(mut self, value: ListMembersRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListMembersRequestDirection) -> Self {
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

    pub fn include_total_count(mut self, value: bool) -> Self {
        self.include_total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MembersListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_ids`](MembersListQueryRequestBuilder::user_ids)
    /// - [`most_recent_actions`](MembersListQueryRequestBuilder::most_recent_actions)
    /// - [`product_ids`](MembersListQueryRequestBuilder::product_ids)
    /// - [`variant_ids`](MembersListQueryRequestBuilder::variant_ids)
    /// - [`promo_code_ids`](MembersListQueryRequestBuilder::promo_code_ids)
    pub fn build(self) -> Result<MembersListQueryRequest, BuildError> {
        Ok(MembersListQueryRequest {
            account_id: self.account_id,
            access_level: self.access_level,
            status: self.status,
            user_ids: self
                .user_ids
                .ok_or_else(|| BuildError::missing_field("user_ids"))?,
            most_recent_actions: self
                .most_recent_actions
                .ok_or_else(|| BuildError::missing_field("most_recent_actions"))?,
            membership_status: self.membership_status,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            variant_ids: self
                .variant_ids
                .ok_or_else(|| BuildError::missing_field("variant_ids"))?,
            promo_code_ids: self
                .promo_code_ids
                .ok_or_else(|| BuildError::missing_field("promo_code_ids"))?,
            query: self.query,
            created_after: self.created_after,
            created_before: self.created_before,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            include_total_count: self.include_total_count,
        })
    }
}
