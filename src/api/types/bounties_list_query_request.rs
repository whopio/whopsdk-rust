pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BountiesListQueryRequest {
    /// Scope the list to this account (`biz_` tag). Requires read access to the account; account API keys may pass their own account or a connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// List the bounties this user participated in (`user_` tag). Must be the authenticated user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Filter by lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListBountiesRequestStatus>,
    /// Filter by the poster's declared goal. Bounties created before the goal taxonomy carry no goal and never match this filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_goal_type: Option<ListBountiesRequestBusinessGoalType>,
    /// Only bounties workable from this country, as an ISO 3166-1 alpha-2 code. Bounties with no country targeting are workable worldwide and always match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Only bounties posted to this forum experience, prefixed `exp_`. An unknown experience, or one outside the caller's scope, matches nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// Substring match on the bounty title or ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only bounties created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only bounties created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListBountiesRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListBountiesRequestDirection>,
    /// Number of bounties to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to paginate forwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of bounties to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl BountiesListQueryRequest {
    pub fn builder() -> BountiesListQueryRequestBuilder {
        <BountiesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountiesListQueryRequestBuilder {
    account_id: Option<String>,
    user_id: Option<String>,
    status: Option<ListBountiesRequestStatus>,
    business_goal_type: Option<ListBountiesRequestBusinessGoalType>,
    country: Option<String>,
    experience_id: Option<String>,
    query: Option<String>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListBountiesRequestOrder>,
    direction: Option<ListBountiesRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl BountiesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListBountiesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn business_goal_type(mut self, value: ListBountiesRequestBusinessGoalType) -> Self {
        self.business_goal_type = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
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

    pub fn order(mut self, value: ListBountiesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListBountiesRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`BountiesListQueryRequest`].
    pub fn build(self) -> Result<BountiesListQueryRequest, BuildError> {
        Ok(BountiesListQueryRequest {
            account_id: self.account_id,
            user_id: self.user_id,
            status: self.status,
            business_goal_type: self.business_goal_type,
            country: self.country,
            experience_id: self.experience_id,
            query: self.query,
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
