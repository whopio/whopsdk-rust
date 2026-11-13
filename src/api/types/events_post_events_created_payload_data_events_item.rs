pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostEventsCreatedPayloadDataEventsItem {
    /// UTM parameters, ad click ids and ad ids, as the page reported them. Keys the event did not carry are left out.
    #[serde(default)]
    pub context: PostEventsCreatedPayloadDataEventsItemContext,
    /// The custom event's own name, for `custom` events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// The event name as `POST /events` takes it, such as `page`, `view_content`, `add_to_cart` or `custom`.
    #[serde(default)]
    pub event_name: String,
    /// When the event happened.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub event_time: DateTime<FixedOffset>,
    /// The event id, unique together with `event_name`. A redelivered event carries the same pair.
    #[serde(default)]
    pub id: String,
    /// The product the event was about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The page that linked to `url`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_url: Option<String>,
    /// The page title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The page the event happened on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Who the event was about, as far as the page knew. Keys the event did not carry are left out.
    #[serde(default)]
    pub user: PostEventsCreatedPayloadDataEventsItemUser,
    /// The event's monetary value. Null when the event carried no value or no currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Money>,
    /// The variant the event was about.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    /// The visitor's `_wuid` first-party cookie id, so a browser session can be joined to this event. Null when the event carried none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wuid: Option<String>,
}

impl PostEventsCreatedPayloadDataEventsItem {
    pub fn builder() -> PostEventsCreatedPayloadDataEventsItemBuilder {
        <PostEventsCreatedPayloadDataEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostEventsCreatedPayloadDataEventsItemBuilder {
    context: Option<PostEventsCreatedPayloadDataEventsItemContext>,
    custom_name: Option<String>,
    event_name: Option<String>,
    event_time: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    product_id: Option<String>,
    referrer_url: Option<String>,
    title: Option<String>,
    url: Option<String>,
    user: Option<PostEventsCreatedPayloadDataEventsItemUser>,
    value: Option<Money>,
    variant_id: Option<String>,
    wuid: Option<String>,
}

impl PostEventsCreatedPayloadDataEventsItemBuilder {
    pub fn context(mut self, value: PostEventsCreatedPayloadDataEventsItemContext) -> Self {
        self.context = Some(value);
        self
    }

    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn event_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_time = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn referrer_url(mut self, value: impl Into<String>) -> Self {
        self.referrer_url = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn user(mut self, value: PostEventsCreatedPayloadDataEventsItemUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn value(mut self, value: Money) -> Self {
        self.value = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    pub fn wuid(mut self, value: impl Into<String>) -> Self {
        self.wuid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostEventsCreatedPayloadDataEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context`](PostEventsCreatedPayloadDataEventsItemBuilder::context)
    /// - [`event_name`](PostEventsCreatedPayloadDataEventsItemBuilder::event_name)
    /// - [`event_time`](PostEventsCreatedPayloadDataEventsItemBuilder::event_time)
    /// - [`id`](PostEventsCreatedPayloadDataEventsItemBuilder::id)
    /// - [`user`](PostEventsCreatedPayloadDataEventsItemBuilder::user)
    pub fn build(self) -> Result<PostEventsCreatedPayloadDataEventsItem, BuildError> {
        Ok(PostEventsCreatedPayloadDataEventsItem {
            context: self
                .context
                .ok_or_else(|| BuildError::missing_field("context"))?,
            custom_name: self.custom_name,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            product_id: self.product_id,
            referrer_url: self.referrer_url,
            title: self.title,
            url: self.url,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            value: self.value,
            variant_id: self.variant_id,
            wuid: self.wuid,
        })
    }
}
