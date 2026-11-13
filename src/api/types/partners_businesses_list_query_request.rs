pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnersBusinessesListQueryRequest {
    /// List the referrals this business's team members made, past members included, instead of the authenticated user's own. Requires company:basic:read on that business, which every team member holds, and cannot be combined with user_id. The owner role reads every referral; anyone else reads only the referrals made by themselves and by the people under them in the org chart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_account_id: Option<String>,
    /// Filter the authenticated partner's CPA offers by status, or with referring_account_id, the CPA terms of the reps the caller manages. Use offered for every state after qualification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advance_status: Option<ListBusinessesRequestAdvanceStatus>,
    /// Filter by referral status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListBusinessesRequestStatus>,
    /// When true, only businesses with pending or completed earnings paid to the caller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_earnings: Option<bool>,
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
    /// Only return partner businesses whose first paid sale above zero happened before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_sale_before: Option<String>,
    /// Only return partner businesses whose first paid sale above zero happened after this timestamp. Combined with first_sale_before, this is the net new set: businesses that processed their first $1 inside the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_sale_after: Option<String>,
    /// Filter to referrals attributed to this user. For first-tier referrals, this is the referred account owner; for second-tier referrals, this is the partner you recruited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referred_user_id: Option<String>,
    /// Filter by the referred user's exact username. Ignored when `referred_user_id` is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referred_username: Option<String>,
    /// Filter to referrals from a single tier: first, second, or blueprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<ListBusinessesRequestTier>,
    /// Case-insensitive business-name prefix, or an exact `biz_` account ID. Surrounding whitespace is ignored; blank values apply no filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_prefix_query: Option<String>,
    /// Only the referrals this team member made, plus the ones they were added to as a deal participant. User ID, prefixed `user_`. Requires `referring_account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earning_partner_id: Option<String>,
}

impl PartnersBusinessesListQueryRequest {
    pub fn builder() -> PartnersBusinessesListQueryRequestBuilder {
        <PartnersBusinessesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnersBusinessesListQueryRequestBuilder {
    referring_account_id: Option<String>,
    advance_status: Option<ListBusinessesRequestAdvanceStatus>,
    status: Option<ListBusinessesRequestStatus>,
    has_earnings: Option<bool>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    include_total_count: Option<bool>,
    order: Option<ListBusinessesRequestOrder>,
    direction: Option<ListBusinessesRequestDirection>,
    created_before: Option<String>,
    created_after: Option<String>,
    first_sale_before: Option<String>,
    first_sale_after: Option<String>,
    referred_user_id: Option<String>,
    referred_username: Option<String>,
    tier: Option<ListBusinessesRequestTier>,
    business_prefix_query: Option<String>,
    earning_partner_id: Option<String>,
}

impl PartnersBusinessesListQueryRequestBuilder {
    pub fn referring_account_id(mut self, value: impl Into<String>) -> Self {
        self.referring_account_id = Some(value.into());
        self
    }

    pub fn advance_status(mut self, value: ListBusinessesRequestAdvanceStatus) -> Self {
        self.advance_status = Some(value);
        self
    }

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

    pub fn include_total_count(mut self, value: bool) -> Self {
        self.include_total_count = Some(value);
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

    pub fn first_sale_before(mut self, value: impl Into<String>) -> Self {
        self.first_sale_before = Some(value.into());
        self
    }

    pub fn first_sale_after(mut self, value: impl Into<String>) -> Self {
        self.first_sale_after = Some(value.into());
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

    pub fn business_prefix_query(mut self, value: impl Into<String>) -> Self {
        self.business_prefix_query = Some(value.into());
        self
    }

    pub fn earning_partner_id(mut self, value: impl Into<String>) -> Self {
        self.earning_partner_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartnersBusinessesListQueryRequest`].
    pub fn build(self) -> Result<PartnersBusinessesListQueryRequest, BuildError> {
        Ok(PartnersBusinessesListQueryRequest {
            referring_account_id: self.referring_account_id,
            advance_status: self.advance_status,
            status: self.status,
            has_earnings: self.has_earnings,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            include_total_count: self.include_total_count,
            order: self.order,
            direction: self.direction,
            created_before: self.created_before,
            created_after: self.created_after,
            first_sale_before: self.first_sale_before,
            first_sale_after: self.first_sale_after,
            referred_user_id: self.referred_user_id,
            referred_username: self.referred_username,
            tier: self.tier,
            business_prefix_query: self.business_prefix_query,
            earning_partner_id: self.earning_partner_id,
        })
    }
}
