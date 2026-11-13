pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamMembersListQueryRequest {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Only return members with this status: `joined` (accepted members) or `pending` (pending invites). Both are returned by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTeamMembersRequestStatus>,
    /// Only return the membership for this user ID, prefixed `user_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Only return the memberships of these users, prefixed `user_`, at most 100. IDs that match no user are ignored. With `account_id=global`, a user can have one in each partner account. Can't be combined with `user_id`.
    #[serde(default)]
    pub user_ids: Vec<Option<String>>,
    /// Only return members with this role. `custom` matches members on a dashboard-managed custom role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ListTeamMembersRequestRole>,
    /// Only return members whose name or username contains this text, and invites whose email contains it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only return the direct reports of this team member, whose ID may start with `ausr_` or `mber_`. Pass the account ID, prefixed `biz_`, for the members who report to the business directly. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports_to_id: Option<String>,
    /// Only return members with at least this many direct reports. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_reports_min: Option<i64>,
    /// Only return members with at most this many direct reports; `0` returns the people who manage no one. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_reports_max: Option<i64>,
    /// Only return this user, prefixed `user_`, and everyone under them in today's org chart: the same team the stats API's `team_lead_id` reads. With `account_id=global`, their line in every partner account where they hold a role. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_lead_id: Option<String>,
    /// When true, only members who referred at least one business for their account; when false, only members who referred none. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_referred_businesses: Option<bool>,
    /// Only return members added before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return members added after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Field used to sort members. Members with the view-only Partner role are listed after all other members whichever way the list is sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListTeamMembersRequestOrder>,
    /// Sort direction. Defaults to `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListTeamMembersRequestDirection>,
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

impl TeamMembersListQueryRequest {
    pub fn builder() -> TeamMembersListQueryRequestBuilder {
        <TeamMembersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamMembersListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListTeamMembersRequestStatus>,
    user_id: Option<String>,
    user_ids: Option<Vec<Option<String>>>,
    role: Option<ListTeamMembersRequestRole>,
    query: Option<String>,
    reports_to_id: Option<String>,
    direct_reports_min: Option<i64>,
    direct_reports_max: Option<i64>,
    team_lead_id: Option<String>,
    has_referred_businesses: Option<bool>,
    created_before: Option<String>,
    created_after: Option<String>,
    order: Option<ListTeamMembersRequestOrder>,
    direction: Option<ListTeamMembersRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl TeamMembersListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListTeamMembersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn user_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.user_ids = Some(value);
        self
    }

    pub fn role(mut self, value: ListTeamMembersRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn reports_to_id(mut self, value: impl Into<String>) -> Self {
        self.reports_to_id = Some(value.into());
        self
    }

    pub fn direct_reports_min(mut self, value: i64) -> Self {
        self.direct_reports_min = Some(value);
        self
    }

    pub fn direct_reports_max(mut self, value: i64) -> Self {
        self.direct_reports_max = Some(value);
        self
    }

    pub fn team_lead_id(mut self, value: impl Into<String>) -> Self {
        self.team_lead_id = Some(value.into());
        self
    }

    pub fn has_referred_businesses(mut self, value: bool) -> Self {
        self.has_referred_businesses = Some(value);
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

    pub fn order(mut self, value: ListTeamMembersRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListTeamMembersRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`TeamMembersListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](TeamMembersListQueryRequestBuilder::account_id)
    /// - [`user_ids`](TeamMembersListQueryRequestBuilder::user_ids)
    pub fn build(self) -> Result<TeamMembersListQueryRequest, BuildError> {
        Ok(TeamMembersListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            status: self.status,
            user_id: self.user_id,
            user_ids: self
                .user_ids
                .ok_or_else(|| BuildError::missing_field("user_ids"))?,
            role: self.role,
            query: self.query,
            reports_to_id: self.reports_to_id,
            direct_reports_min: self.direct_reports_min,
            direct_reports_max: self.direct_reports_max,
            team_lead_id: self.team_lead_id,
            has_referred_businesses: self.has_referred_businesses,
            created_before: self.created_before,
            created_after: self.created_after,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
