pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodDomainsListQueryRequest {
    /// Only domains registered for this account (`biz_` tag). Defaults to the caller's account plus its connected accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only the domain with this exact hostname.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Only domains with this verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListPaymentMethodDomainsRequestStatus>,
    /// Only domains registered with this wallet provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<ListPaymentMethodDomainsRequestProvider>,
    /// Only domains created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only domains created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPaymentMethodDomainsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPaymentMethodDomainsRequestDirection>,
    /// Number of domains to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to paginate forwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of domains to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl PaymentMethodDomainsListQueryRequest {
    pub fn builder() -> PaymentMethodDomainsListQueryRequestBuilder {
        <PaymentMethodDomainsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDomainsListQueryRequestBuilder {
    account_id: Option<String>,
    hostname: Option<String>,
    status: Option<ListPaymentMethodDomainsRequestStatus>,
    provider: Option<ListPaymentMethodDomainsRequestProvider>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    order: Option<ListPaymentMethodDomainsRequestOrder>,
    direction: Option<ListPaymentMethodDomainsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl PaymentMethodDomainsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPaymentMethodDomainsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn provider(mut self, value: ListPaymentMethodDomainsRequestProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn order(mut self, value: ListPaymentMethodDomainsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListPaymentMethodDomainsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`PaymentMethodDomainsListQueryRequest`].
    pub fn build(self) -> Result<PaymentMethodDomainsListQueryRequest, BuildError> {
        Ok(PaymentMethodDomainsListQueryRequest {
            account_id: self.account_id,
            hostname: self.hostname,
            status: self.status,
            provider: self.provider,
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
