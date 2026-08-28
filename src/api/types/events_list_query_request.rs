pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EventsListQueryRequest {
    /// Any hard identifier of the person: a person ID (prsn_*), user ID, email, phone number, or a tracking cookie value (wuid, anonymous ID, fbp/fbc/ttp/ga). Omit to list recent events for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Start of the time range as an ISO 8601 timestamp. Required when identifier is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub from: Option<DateTime<FixedOffset>>,
    /// End of the time range as an ISO 8601 timestamp. Required when identifier is omitted; otherwise defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub to: Option<DateTime<FixedOffset>>,
    /// The number of events to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor for fetching events after a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for fetching events before a later page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The order events are returned in by time. Defaults to desc (most recent first); asc reads a journey forwards from where it starts. after and before always page forwards and backwards through that order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListEventsRequestDirection>,
    /// Full event names to filter by, comma-separated (payment.completed, pixel.lead, pixel.page, pixel.custom:<name>) — the same vocabulary the events / people metrics use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Canonical source path, exact or with a trailing :* prefix (whop:<campaign>:*, ext:meta:*, referrer:<domain>, direct). Restricts the list to conversion targets attributed to that source — the debuggability twin of a metric cell's source parameter. A whop:... source combined with non-conversion event names (event=pixel.page) instead lists the events whose ad click resolved to that entity — the page views an ad drove.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Attribution model for the source filter (defaults to last_touch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_model: Option<ListEventsRequestAttributionModel>,
    /// Country codes to filter by, comma-separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Cities to filter by, comma-separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Device families to filter by, comma-separated (e.g. iPhone, Mac).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Browser families to filter by, comma-separated (e.g. Chrome, Mobile Safari).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    /// Operating system families to filter by, comma-separated (e.g. iOS, Windows).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// utm_source values to filter by, comma-separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    /// Page hostnames to filter by, comma-separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Page paths to filter by, comma-separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

impl EventsListQueryRequest {
    pub fn builder() -> EventsListQueryRequestBuilder {
        <EventsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventsListQueryRequestBuilder {
    identifier: Option<String>,
    account_id: Option<String>,
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    first: Option<i64>,
    after: Option<String>,
    before: Option<String>,
    direction: Option<ListEventsRequestDirection>,
    event: Option<String>,
    source: Option<String>,
    attribution_model: Option<ListEventsRequestAttributionModel>,
    country: Option<String>,
    city: Option<String>,
    device: Option<String>,
    browser: Option<String>,
    os: Option<String>,
    utm_source: Option<String>,
    hostname: Option<String>,
    page: Option<String>,
}

impl EventsListQueryRequestBuilder {
    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
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

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn direction(mut self, value: ListEventsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn attribution_model(mut self, value: ListEventsRequestAttributionModel) -> Self {
        self.attribution_model = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn device(mut self, value: impl Into<String>) -> Self {
        self.device = Some(value.into());
        self
    }

    pub fn browser(mut self, value: impl Into<String>) -> Self {
        self.browser = Some(value.into());
        self
    }

    pub fn os(mut self, value: impl Into<String>) -> Self {
        self.os = Some(value.into());
        self
    }

    pub fn utm_source(mut self, value: impl Into<String>) -> Self {
        self.utm_source = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn page(mut self, value: impl Into<String>) -> Self {
        self.page = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EventsListQueryRequest`].
    pub fn build(self) -> Result<EventsListQueryRequest, BuildError> {
        Ok(EventsListQueryRequest {
            identifier: self.identifier,
            account_id: self.account_id,
            from: self.from,
            to: self.to,
            first: self.first,
            after: self.after,
            before: self.before,
            direction: self.direction,
            event: self.event,
            source: self.source,
            attribution_model: self.attribution_model,
            country: self.country,
            city: self.city,
            device: self.device,
            browser: self.browser,
            os: self.os,
            utm_source: self.utm_source,
            hostname: self.hostname,
            page: self.page,
        })
    }
}
