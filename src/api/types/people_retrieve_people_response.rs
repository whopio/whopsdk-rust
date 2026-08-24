pub use crate::prelude::*;

/// The full profile a retrieve returns: the summary plus every linked identity, purchase rows, all acquisition sources, and exact usage breakdowns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrievePeopleResponse {
    #[serde(default)]
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub aov: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_event_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<RetrievePeopleResponseDevice>,
    /// The email from the person's most recent event that carried one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Every linked email, primary first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    #[serde(default)]
    pub event_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_purchase_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub first_seen_at: DateTime<FixedOffset>,
    /// Where a visit came from: a whop ad click, a lead form, an external ad, or a referring site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_source: Option<RetrievePeopleResponseFirstSource>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_purchase_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub last_seen_at: DateTime<FixedOffset>,
    /// Where a visit came from: a whop ad click, a lead form, an external ad, or a referring site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_source: Option<RetrievePeopleResponseLastSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<RetrievePeopleResponseLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ltv: Option<f64>,
    /// The user's member record at this account, when they are a member of it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<RetrievePeopleResponseMember>,
    /// The name from the person's most recent event that carried one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Every name the person's linked identities carried, primary first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// The phone from the person's most recent event that carried one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Every linked phone, primary first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<String>>,
    #[serde(default)]
    pub purchase_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchases: Option<Vec<RetrievePeopleResponsePurchasesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// Every distinct acquisition signal the person ever carried, ad entities hydrated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<RetrievePeopleResponseSourcesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Exact usage breakdowns for the person's browser traffic (distinct events per value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<RetrievePeopleResponseUsage>,
    /// The person's primary whop user, when one of their identities is a whop account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<RetrievePeopleResponseUser>,
    /// Every linked whop account, the most used one first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl RetrievePeopleResponse {
    pub fn builder() -> RetrievePeopleResponseBuilder {
        <RetrievePeopleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponseBuilder {
    account_id: Option<String>,
    aov: Option<f64>,
    audience_ids: Option<Vec<String>>,
    custom_event_names: Option<Vec<String>>,
    device: Option<RetrievePeopleResponseDevice>,
    email: Option<String>,
    emails: Option<Vec<String>>,
    event_count: Option<i64>,
    event_names: Option<Vec<String>>,
    first_purchase_at: Option<DateTime<FixedOffset>>,
    first_seen_at: Option<DateTime<FixedOffset>>,
    first_source: Option<RetrievePeopleResponseFirstSource>,
    id: Option<String>,
    last_ip: Option<String>,
    last_purchase_at: Option<DateTime<FixedOffset>>,
    last_seen_at: Option<DateTime<FixedOffset>>,
    last_source: Option<RetrievePeopleResponseLastSource>,
    location: Option<RetrievePeopleResponseLocation>,
    ltv: Option<f64>,
    member: Option<RetrievePeopleResponseMember>,
    name: Option<String>,
    names: Option<Vec<String>>,
    phone: Option<String>,
    phones: Option<Vec<String>>,
    purchase_count: Option<i64>,
    purchases: Option<Vec<RetrievePeopleResponsePurchasesItem>>,
    roles: Option<Vec<String>>,
    sources: Option<Vec<RetrievePeopleResponseSourcesItem>>,
    timezone: Option<String>,
    usage: Option<RetrievePeopleResponseUsage>,
    user: Option<RetrievePeopleResponseUser>,
    user_ids: Option<Vec<String>>,
}

impl RetrievePeopleResponseBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn aov(mut self, value: f64) -> Self {
        self.aov = Some(value);
        self
    }

    pub fn audience_ids(mut self, value: Vec<String>) -> Self {
        self.audience_ids = Some(value);
        self
    }

    pub fn custom_event_names(mut self, value: Vec<String>) -> Self {
        self.custom_event_names = Some(value);
        self
    }

    pub fn device(mut self, value: RetrievePeopleResponseDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
        self
    }

    pub fn event_names(mut self, value: Vec<String>) -> Self {
        self.event_names = Some(value);
        self
    }

    pub fn first_purchase_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_purchase_at = Some(value);
        self
    }

    pub fn first_seen_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn first_source(mut self, value: RetrievePeopleResponseFirstSource) -> Self {
        self.first_source = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_ip(mut self, value: impl Into<String>) -> Self {
        self.last_ip = Some(value.into());
        self
    }

    pub fn last_purchase_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_purchase_at = Some(value);
        self
    }

    pub fn last_seen_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn last_source(mut self, value: RetrievePeopleResponseLastSource) -> Self {
        self.last_source = Some(value);
        self
    }

    pub fn location(mut self, value: RetrievePeopleResponseLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn ltv(mut self, value: f64) -> Self {
        self.ltv = Some(value);
        self
    }

    pub fn member(mut self, value: RetrievePeopleResponseMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn names(mut self, value: Vec<String>) -> Self {
        self.names = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn phones(mut self, value: Vec<String>) -> Self {
        self.phones = Some(value);
        self
    }

    pub fn purchase_count(mut self, value: i64) -> Self {
        self.purchase_count = Some(value);
        self
    }

    pub fn purchases(mut self, value: Vec<RetrievePeopleResponsePurchasesItem>) -> Self {
        self.purchases = Some(value);
        self
    }

    pub fn roles(mut self, value: Vec<String>) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<RetrievePeopleResponseSourcesItem>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn usage(mut self, value: RetrievePeopleResponseUsage) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn user(mut self, value: RetrievePeopleResponseUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn user_ids(mut self, value: Vec<String>) -> Self {
        self.user_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](RetrievePeopleResponseBuilder::account_id)
    /// - [`event_count`](RetrievePeopleResponseBuilder::event_count)
    /// - [`first_seen_at`](RetrievePeopleResponseBuilder::first_seen_at)
    /// - [`id`](RetrievePeopleResponseBuilder::id)
    /// - [`last_seen_at`](RetrievePeopleResponseBuilder::last_seen_at)
    /// - [`purchase_count`](RetrievePeopleResponseBuilder::purchase_count)
    pub fn build(self) -> Result<RetrievePeopleResponse, BuildError> {
        Ok(RetrievePeopleResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            aov: self.aov,
            audience_ids: self.audience_ids,
            custom_event_names: self.custom_event_names,
            device: self.device,
            email: self.email,
            emails: self.emails,
            event_count: self
                .event_count
                .ok_or_else(|| BuildError::missing_field("event_count"))?,
            event_names: self.event_names,
            first_purchase_at: self.first_purchase_at,
            first_seen_at: self
                .first_seen_at
                .ok_or_else(|| BuildError::missing_field("first_seen_at"))?,
            first_source: self.first_source,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_ip: self.last_ip,
            last_purchase_at: self.last_purchase_at,
            last_seen_at: self
                .last_seen_at
                .ok_or_else(|| BuildError::missing_field("last_seen_at"))?,
            last_source: self.last_source,
            location: self.location,
            ltv: self.ltv,
            member: self.member,
            name: self.name,
            names: self.names,
            phone: self.phone,
            phones: self.phones,
            purchase_count: self
                .purchase_count
                .ok_or_else(|| BuildError::missing_field("purchase_count"))?,
            purchases: self.purchases,
            roles: self.roles,
            sources: self.sources,
            timezone: self.timezone,
            usage: self.usage,
            user: self.user,
            user_ids: self.user_ids,
        })
    }
}
