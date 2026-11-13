pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnerReferralRequestsListQueryRequest {
    /// Only requests for this business ID, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only requests received by this user ID, prefixed `user_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<String>,
    /// Only requests sent by this partner's user ID, prefixed `user_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_id: Option<String>,
    /// Only the referral links created for memberships on this business's team, prefixed `biz_`. Needs read access to the business: its owner sees every member's links, a manager the links of the people reporting to them, and other members only their own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_account_id: Option<String>,
    /// Only requests with this approval status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPartnerReferralRequestsRequestStatus>,
    /// Only requests initiated in this way.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<ListPartnerReferralRequestsRequestRequestType>,
    /// Only requests created for this team membership, prefixed `ausr_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_user_id: Option<String>,
    /// Only requests that credit this member record, prefixed `mber_`. A request created for a team membership also credits its holder's member record.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// When true, only personal requests, created for neither a team membership nor a member record. When false, only requests created for one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,
    /// Field used to sort requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPartnerReferralRequestsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPartnerReferralRequestsRequestDirection>,
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

impl PartnerReferralRequestsListQueryRequest {
    pub fn builder() -> PartnerReferralRequestsListQueryRequestBuilder {
        <PartnerReferralRequestsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerReferralRequestsListQueryRequestBuilder {
    account_id: Option<String>,
    target_user_id: Option<String>,
    partner_id: Option<String>,
    referring_account_id: Option<String>,
    status: Option<ListPartnerReferralRequestsRequestStatus>,
    request_type: Option<ListPartnerReferralRequestsRequestRequestType>,
    authorized_user_id: Option<String>,
    member_id: Option<String>,
    personal: Option<bool>,
    order: Option<ListPartnerReferralRequestsRequestOrder>,
    direction: Option<ListPartnerReferralRequestsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PartnerReferralRequestsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn target_user_id(mut self, value: impl Into<String>) -> Self {
        self.target_user_id = Some(value.into());
        self
    }

    pub fn partner_id(mut self, value: impl Into<String>) -> Self {
        self.partner_id = Some(value.into());
        self
    }

    pub fn referring_account_id(mut self, value: impl Into<String>) -> Self {
        self.referring_account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPartnerReferralRequestsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn request_type(mut self, value: ListPartnerReferralRequestsRequestRequestType) -> Self {
        self.request_type = Some(value);
        self
    }

    pub fn authorized_user_id(mut self, value: impl Into<String>) -> Self {
        self.authorized_user_id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn personal(mut self, value: bool) -> Self {
        self.personal = Some(value);
        self
    }

    pub fn order(mut self, value: ListPartnerReferralRequestsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListPartnerReferralRequestsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`PartnerReferralRequestsListQueryRequest`].
    pub fn build(self) -> Result<PartnerReferralRequestsListQueryRequest, BuildError> {
        Ok(PartnerReferralRequestsListQueryRequest {
            account_id: self.account_id,
            target_user_id: self.target_user_id,
            partner_id: self.partner_id,
            referring_account_id: self.referring_account_id,
            status: self.status,
            request_type: self.request_type,
            authorized_user_id: self.authorized_user_id,
            member_id: self.member_id,
            personal: self.personal,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
