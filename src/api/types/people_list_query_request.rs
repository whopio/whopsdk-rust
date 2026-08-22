pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PeopleListQueryRequest {
    /// Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Search people by name, email, phone, or whop user ID (case-insensitive substring match).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only include people acquired from any of these sources — canonical paths (whop:<campaign>:<group>:<ad>, ext:<platform>:..., referrer:<domain>, direct, other), exact or with a trailing :* prefix. The same vocabulary the events / people metrics use.
    #[serde(default)]
    pub source: Vec<Option<String>>,
    /// Attribution model the source filter matches against (defaults to last_touch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_model: Option<ListPeopleRequestAttributionModel>,
    /// Only include people who fired any of these events, e.g. payment.completed or page.checkout.view.
    #[serde(default)]
    pub event_name: Vec<Option<String>>,
    /// Only include people who fired this custom pixel event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_event: Option<String>,
    /// With event_to plus an event or source filter, switches to exact-population mode: person ids are resolved and paginated on the events side within this window (the same query the people metric counts), then hydrated per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub event_from: Option<DateTime<FixedOffset>>,
    /// The inclusive end of the event window for exact-population mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub event_to: Option<DateTime<FixedOffset>>,
    /// Only include people in this audience. An audience that keeps itself up to date resolves to the People filters that define it, so this always reflects who matches now; uploaded lists and point-in-time snapshots match their recorded members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_id: Option<String>,
    /// Only include the person linked to this whop user ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Only include the person linked to this email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Only include the person linked to this phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Only include people whose most recent visit came from this ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// true for customers only, false for people who have never purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_purchased: Option<bool>,
    /// true for people who have an email address or phone number — the ones an ad platform can match.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contactable: Option<bool>,
    /// Only include people first seen within this many days, as a rolling window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_within_days: Option<i64>,
    /// Only include people last seen within this many days, as a rolling window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_within_days: Option<i64>,
    /// Only include people first seen at or after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub first_seen_after: Option<DateTime<FixedOffset>>,
    /// Only include people first seen before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub first_seen_before: Option<DateTime<FixedOffset>>,
    /// Only include people last seen at or after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_seen_after: Option<DateTime<FixedOffset>>,
    /// Only include people last seen before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_seen_before: Option<DateTime<FixedOffset>>,
    /// The number of people to return (default 100, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor for fetching people after a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for fetching people before a later page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Column to sort by. Defaults to last_seen_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListPeopleRequestOrder>,
    /// Sort direction. Defaults to desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListPeopleRequestDirection>,
}

impl PeopleListQueryRequest {
    pub fn builder() -> PeopleListQueryRequestBuilder {
        <PeopleListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeopleListQueryRequestBuilder {
    account_id: Option<String>,
    query: Option<String>,
    source: Option<Vec<Option<String>>>,
    attribution_model: Option<ListPeopleRequestAttributionModel>,
    event_name: Option<Vec<Option<String>>>,
    custom_event: Option<String>,
    event_from: Option<DateTime<FixedOffset>>,
    event_to: Option<DateTime<FixedOffset>>,
    audience_id: Option<String>,
    user_id: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    country: Option<String>,
    has_purchased: Option<bool>,
    contactable: Option<bool>,
    first_seen_within_days: Option<i64>,
    last_seen_within_days: Option<i64>,
    first_seen_after: Option<DateTime<FixedOffset>>,
    first_seen_before: Option<DateTime<FixedOffset>>,
    last_seen_after: Option<DateTime<FixedOffset>>,
    last_seen_before: Option<DateTime<FixedOffset>>,
    first: Option<i64>,
    after: Option<String>,
    before: Option<String>,
    order: Option<ListPeopleRequestOrder>,
    direction: Option<ListPeopleRequestDirection>,
}

impl PeopleListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn source(mut self, value: Vec<Option<String>>) -> Self {
        self.source = Some(value);
        self
    }

    pub fn attribution_model(mut self, value: ListPeopleRequestAttributionModel) -> Self {
        self.attribution_model = Some(value);
        self
    }

    pub fn event_name(mut self, value: Vec<Option<String>>) -> Self {
        self.event_name = Some(value);
        self
    }

    pub fn custom_event(mut self, value: impl Into<String>) -> Self {
        self.custom_event = Some(value.into());
        self
    }

    pub fn event_from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_from = Some(value);
        self
    }

    pub fn event_to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_to = Some(value);
        self
    }

    pub fn audience_id(mut self, value: impl Into<String>) -> Self {
        self.audience_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn has_purchased(mut self, value: bool) -> Self {
        self.has_purchased = Some(value);
        self
    }

    pub fn contactable(mut self, value: bool) -> Self {
        self.contactable = Some(value);
        self
    }

    pub fn first_seen_within_days(mut self, value: i64) -> Self {
        self.first_seen_within_days = Some(value);
        self
    }

    pub fn last_seen_within_days(mut self, value: i64) -> Self {
        self.last_seen_within_days = Some(value);
        self
    }

    pub fn first_seen_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_seen_after = Some(value);
        self
    }

    pub fn first_seen_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_seen_before = Some(value);
        self
    }

    pub fn last_seen_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_seen_after = Some(value);
        self
    }

    pub fn last_seen_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_seen_before = Some(value);
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

    pub fn order(mut self, value: ListPeopleRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListPeopleRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeopleListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`source`](PeopleListQueryRequestBuilder::source)
    /// - [`event_name`](PeopleListQueryRequestBuilder::event_name)
    pub fn build(self) -> Result<PeopleListQueryRequest, BuildError> {
        Ok(PeopleListQueryRequest {
            account_id: self.account_id,
            query: self.query,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            attribution_model: self.attribution_model,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            custom_event: self.custom_event,
            event_from: self.event_from,
            event_to: self.event_to,
            audience_id: self.audience_id,
            user_id: self.user_id,
            email: self.email,
            phone: self.phone,
            country: self.country,
            has_purchased: self.has_purchased,
            contactable: self.contactable,
            first_seen_within_days: self.first_seen_within_days,
            last_seen_within_days: self.last_seen_within_days,
            first_seen_after: self.first_seen_after,
            first_seen_before: self.first_seen_before,
            last_seen_after: self.last_seen_after,
            last_seen_before: self.last_seen_before,
            first: self.first,
            after: self.after,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
