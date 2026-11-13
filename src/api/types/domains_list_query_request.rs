pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainsListQueryRequest {
    /// Only domains belonging to this account, prefixed biz_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only domains whose website serves this app, prefixed app_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Only domains with this status: the most pressing state of their capabilities, or `idle` when they want nothing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListDomainsRequestStatus>,
    /// Field to sort by. Domains with the same value keep their creation order in the same direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListDomainsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListDomainsRequestDirection>,
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
    /// A name or domain to find domains to buy, such as `example` or `example.com`; a subdomain or URL searches its registrable domain. Returns domains with `availability` instead of your domains, without other filters or pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// With `search`, check only these extensions, such as `com` or `co.uk`, returned in this order. Repeat for several, up to 100.
    #[serde(default)]
    pub tlds: Vec<Option<String>>,
    /// Only your domain with this hostname, such as `example.com`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Only domains with verification in this state, such as `ready`, or `any` for every domain that has verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<ListDomainsRequestVerification>,
    /// Only domains with registration in this state, such as `ready`, or `any` for every domain that has registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration: Option<ListDomainsRequestRegistration>,
    /// Only domains with platform in this state, such as `ready`, or `any` for every domain that has platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<ListDomainsRequestPlatform>,
    /// Only domains with subdomain in this state, such as `ready`, or `any` for every domain that has subdomain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<ListDomainsRequestSubdomain>,
    /// Only domains with website in this state, such as `ready`, or `any` for every domain that has website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<ListDomainsRequestWebsite>,
    /// Only domains with apple_pay in this state, such as `ready`, or `any` for every domain that has apple_pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<ListDomainsRequestApplePay>,
    /// Only domains with pixel in this state, such as `ready`, or `any` for every domain that has pixel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pixel: Option<ListDomainsRequestPixel>,
    /// Only domains with email in this state, such as `ready`, or `any` for every domain that has email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<ListDomainsRequestEmail>,
    /// Only domains with transfer in this state, such as `ready`, or `any` for every domain that has transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<ListDomainsRequestTransfer>,
}

impl DomainsListQueryRequest {
    pub fn builder() -> DomainsListQueryRequestBuilder {
        <DomainsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainsListQueryRequestBuilder {
    account_id: Option<String>,
    app_id: Option<String>,
    status: Option<ListDomainsRequestStatus>,
    order: Option<ListDomainsRequestOrder>,
    direction: Option<ListDomainsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    search: Option<String>,
    tlds: Option<Vec<Option<String>>>,
    domain: Option<String>,
    verification: Option<ListDomainsRequestVerification>,
    registration: Option<ListDomainsRequestRegistration>,
    platform: Option<ListDomainsRequestPlatform>,
    subdomain: Option<ListDomainsRequestSubdomain>,
    website: Option<ListDomainsRequestWebsite>,
    apple_pay: Option<ListDomainsRequestApplePay>,
    pixel: Option<ListDomainsRequestPixel>,
    email: Option<ListDomainsRequestEmail>,
    transfer: Option<ListDomainsRequestTransfer>,
}

impl DomainsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListDomainsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn order(mut self, value: ListDomainsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListDomainsRequestDirection) -> Self {
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

    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn tlds(mut self, value: Vec<Option<String>>) -> Self {
        self.tlds = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn verification(mut self, value: ListDomainsRequestVerification) -> Self {
        self.verification = Some(value);
        self
    }

    pub fn registration(mut self, value: ListDomainsRequestRegistration) -> Self {
        self.registration = Some(value);
        self
    }

    pub fn platform(mut self, value: ListDomainsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn subdomain(mut self, value: ListDomainsRequestSubdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn website(mut self, value: ListDomainsRequestWebsite) -> Self {
        self.website = Some(value);
        self
    }

    pub fn apple_pay(mut self, value: ListDomainsRequestApplePay) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn pixel(mut self, value: ListDomainsRequestPixel) -> Self {
        self.pixel = Some(value);
        self
    }

    pub fn email(mut self, value: ListDomainsRequestEmail) -> Self {
        self.email = Some(value);
        self
    }

    pub fn transfer(mut self, value: ListDomainsRequestTransfer) -> Self {
        self.transfer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tlds`](DomainsListQueryRequestBuilder::tlds)
    pub fn build(self) -> Result<DomainsListQueryRequest, BuildError> {
        Ok(DomainsListQueryRequest {
            account_id: self.account_id,
            app_id: self.app_id,
            status: self.status,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            search: self.search,
            tlds: self.tlds.ok_or_else(|| BuildError::missing_field("tlds"))?,
            domain: self.domain,
            verification: self.verification,
            registration: self.registration,
            platform: self.platform,
            subdomain: self.subdomain,
            website: self.website,
            apple_pay: self.apple_pay,
            pixel: self.pixel,
            email: self.email,
            transfer: self.transfer,
        })
    }
}
