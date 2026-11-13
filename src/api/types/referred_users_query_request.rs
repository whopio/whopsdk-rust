pub use crate::prelude::*;

/// Query parameters for referredUsers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferredUsersQueryRequest {
    /// The selected referring partner's user_ ID. Requires user_id=global; cannot be combined with earning_partner_username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earning_partner_id: Option<String>,
    /// The selected referring partner's exact username. Requires user_id=global; cannot be combined with earning_partner_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earning_partner_username: Option<String>,
    /// List the users this business's team members referred, past members included, instead of the caller's own. Requires company:basic:read on that business, which every team member holds, and cannot be combined with user_id. The owner role reads every team member's referrals; anyone else reads only their own and those of the people under them in the org chart. earning_partner_id or earning_partner_username then narrows to one team member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_account_id: Option<String>,
    /// Search referred users by name or username. In global mode, matches the beginning of usernames only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// When true, only referred users who brought at least one business onto Whop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_businesses: Option<bool>,
    /// When true, only referred users with at least one business that has generated earnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_earning_businesses: Option<bool>,
    /// The field to sort by. created_at uses the user's signup date; earnings_usd uses the current primary referrer's cached total affiliate earnings from that user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ReferredUsersPartnersRequestOrder>,
    /// The direction to sort results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ReferredUsersPartnersRequestDirection>,
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

impl ReferredUsersQueryRequest {
    pub fn builder() -> ReferredUsersQueryRequestBuilder {
        <ReferredUsersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferredUsersQueryRequestBuilder {
    earning_partner_id: Option<String>,
    earning_partner_username: Option<String>,
    referring_account_id: Option<String>,
    query: Option<String>,
    has_businesses: Option<bool>,
    has_earning_businesses: Option<bool>,
    order: Option<ReferredUsersPartnersRequestOrder>,
    direction: Option<ReferredUsersPartnersRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl ReferredUsersQueryRequestBuilder {
    pub fn earning_partner_id(mut self, value: impl Into<String>) -> Self {
        self.earning_partner_id = Some(value.into());
        self
    }

    pub fn earning_partner_username(mut self, value: impl Into<String>) -> Self {
        self.earning_partner_username = Some(value.into());
        self
    }

    pub fn referring_account_id(mut self, value: impl Into<String>) -> Self {
        self.referring_account_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn has_businesses(mut self, value: bool) -> Self {
        self.has_businesses = Some(value);
        self
    }

    pub fn has_earning_businesses(mut self, value: bool) -> Self {
        self.has_earning_businesses = Some(value);
        self
    }

    pub fn order(mut self, value: ReferredUsersPartnersRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ReferredUsersPartnersRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`ReferredUsersQueryRequest`].
    pub fn build(self) -> Result<ReferredUsersQueryRequest, BuildError> {
        Ok(ReferredUsersQueryRequest {
            earning_partner_id: self.earning_partner_id,
            earning_partner_username: self.earning_partner_username,
            referring_account_id: self.referring_account_id,
            query: self.query,
            has_businesses: self.has_businesses,
            has_earning_businesses: self.has_earning_businesses,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
