pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnersBusinessesListQueryRequest {
    /// Filter by referral status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListBusinessesRequestStatus>,
    /// When true, only businesses with pending or completed earnings paid to the caller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_earnings: Option<bool>,
    /// Number of partner businesses to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of partner businesses to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort partner businesses by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListBusinessesRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListBusinessesRequestDirection>,
    /// Only return partner businesses created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return partner businesses created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Filter to referrals attributed to this user. For first-tier referrals, this is the referred account owner; for second-tier referrals, this is the partner you recruited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referred_user_id: Option<String>,
    /// Filter by the referred user's exact username. Ignored when `referred_user_id` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referred_username: Option<String>,
    /// Filter to referrals from a single tier: first, second, or blueprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<ListBusinessesRequestTier>,
}

impl PartnersBusinessesListQueryRequest {
    pub fn builder() -> PartnersBusinessesListQueryRequestBuilder {
        <PartnersBusinessesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnersBusinessesListQueryRequestBuilder {
    status: Option<ListBusinessesRequestStatus>,
    has_earnings: Option<bool>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListBusinessesRequestOrder>,
    direction: Option<ListBusinessesRequestDirection>,
    created_before: Option<String>,
    created_after: Option<String>,
    referred_user_id: Option<String>,
    referred_username: Option<String>,
    tier: Option<ListBusinessesRequestTier>,
}

impl PartnersBusinessesListQueryRequestBuilder {
    pub fn status(mut self, value: ListBusinessesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn has_earnings(mut self, value: bool) -> Self {
        self.has_earnings = Some(value);
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

    pub fn order(mut self, value: ListBusinessesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListBusinessesRequestDirection) -> Self {
        self.direction = Some(value);
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

    pub fn referred_user_id(mut self, value: impl Into<String>) -> Self {
        self.referred_user_id = Some(value.into());
        self
    }

    pub fn referred_username(mut self, value: impl Into<String>) -> Self {
        self.referred_username = Some(value.into());
        self
    }

    pub fn tier(mut self, value: ListBusinessesRequestTier) -> Self {
        self.tier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnersBusinessesListQueryRequest`].
    pub fn build(self) -> Result<PartnersBusinessesListQueryRequest, BuildError> {
        Ok(PartnersBusinessesListQueryRequest {
            status: self.status,
            has_earnings: self.has_earnings,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
            created_before: self.created_before,
            created_after: self.created_after,
            referred_user_id: self.referred_user_id,
            referred_username: self.referred_username,
            tier: self.tier,
        })
    }
}
