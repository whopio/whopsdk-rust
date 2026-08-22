pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPeopleResponseDataItem {
    #[serde(default)]
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub aov: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<ListPeopleResponseDataItemDevice>,
    /// The email from the person's most recent event that carried one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    pub event_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_purchase_at: Option<DateTime<FixedOffset>>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub first_seen_at: DateTime<FixedOffset>,
    /// Where a visit came from: a whop ad click, a lead form, an external ad, or a referring site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_source: Option<ListPeopleResponseDataItemFirstSource>,
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
    pub last_source: Option<ListPeopleResponseDataItemLastSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ListPeopleResponseDataItemLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ltv: Option<f64>,
    /// The user's member record at this account, when they are a member of it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<ListPeopleResponseDataItemMember>,
    /// The name from the person's most recent event that carried one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The phone from the person's most recent event that carried one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub purchase_count: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The person's primary whop user, when one of their identities is a whop account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ListPeopleResponseDataItemUser>,
}

impl ListPeopleResponseDataItem {
    pub fn builder() -> ListPeopleResponseDataItemBuilder {
        <ListPeopleResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseDataItemBuilder {
    account_id: Option<String>,
    aov: Option<f64>,
    device: Option<ListPeopleResponseDataItemDevice>,
    email: Option<String>,
    event_count: Option<i64>,
    first_purchase_at: Option<DateTime<FixedOffset>>,
    first_seen_at: Option<DateTime<FixedOffset>>,
    first_source: Option<ListPeopleResponseDataItemFirstSource>,
    id: Option<String>,
    last_ip: Option<String>,
    last_purchase_at: Option<DateTime<FixedOffset>>,
    last_seen_at: Option<DateTime<FixedOffset>>,
    last_source: Option<ListPeopleResponseDataItemLastSource>,
    location: Option<ListPeopleResponseDataItemLocation>,
    ltv: Option<f64>,
    member: Option<ListPeopleResponseDataItemMember>,
    name: Option<String>,
    phone: Option<String>,
    purchase_count: Option<i64>,
    timezone: Option<String>,
    user: Option<ListPeopleResponseDataItemUser>,
}

impl ListPeopleResponseDataItemBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn aov(mut self, value: f64) -> Self {
        self.aov = Some(value);
        self
    }

    pub fn device(mut self, value: ListPeopleResponseDataItemDevice) -> Self {
        self.device = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn event_count(mut self, value: i64) -> Self {
        self.event_count = Some(value);
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

    pub fn first_source(mut self, value: ListPeopleResponseDataItemFirstSource) -> Self {
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

    pub fn last_source(mut self, value: ListPeopleResponseDataItemLastSource) -> Self {
        self.last_source = Some(value);
        self
    }

    pub fn location(mut self, value: ListPeopleResponseDataItemLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn ltv(mut self, value: f64) -> Self {
        self.ltv = Some(value);
        self
    }

    pub fn member(mut self, value: ListPeopleResponseDataItemMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn purchase_count(mut self, value: i64) -> Self {
        self.purchase_count = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn user(mut self, value: ListPeopleResponseDataItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPeopleResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ListPeopleResponseDataItemBuilder::account_id)
    /// - [`event_count`](ListPeopleResponseDataItemBuilder::event_count)
    /// - [`first_seen_at`](ListPeopleResponseDataItemBuilder::first_seen_at)
    /// - [`id`](ListPeopleResponseDataItemBuilder::id)
    /// - [`last_seen_at`](ListPeopleResponseDataItemBuilder::last_seen_at)
    /// - [`purchase_count`](ListPeopleResponseDataItemBuilder::purchase_count)
    pub fn build(self) -> Result<ListPeopleResponseDataItem, BuildError> {
        Ok(ListPeopleResponseDataItem {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            aov: self.aov,
            device: self.device,
            email: self.email,
            event_count: self
                .event_count
                .ok_or_else(|| BuildError::missing_field("event_count"))?,
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
            phone: self.phone,
            purchase_count: self
                .purchase_count
                .ok_or_else(|| BuildError::missing_field("purchase_count"))?,
            timezone: self.timezone,
            user: self.user,
        })
    }
}
