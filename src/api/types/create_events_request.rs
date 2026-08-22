pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateEventsRequest {
    /// The account to associate with this event.
    #[serde(default)]
    pub account_id: String,
    /// Where the event originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_source: Option<CreateEventsRequestActionSource>,
    /// Tracking and attribution context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<CreateEventsRequestContext>,
    /// ISO 4217 currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<CreateEventsRequestCurrency>,
    /// Custom event name when event_name is 'custom'. Maximum 35 chars for this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// For 'leave' events: milliseconds the visitor spent on the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Client-provided identifier for deduplication. Generated if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The type of event.
    ///
    /// Use a standard event (lead, submit_application, contact, complete_registration, schedule, view_content, add_to_cart) or pass your own name directly for a custom event.
    #[serde(default)]
    pub event_name: String,
    /// When the event occurred. Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<DateTime<FixedOffset>>,
    /// The plan associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// The product associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The referring URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_url: Option<String>,
    /// For 'page' events: true when the page was restored from the back/forward cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumed: Option<bool>,
    /// For 'identify' events: where the identity was captured (url, form, manual, iframe).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// For 'page' events: the document title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The URL where the event occurred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// User identity and profile data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<CreateEventsRequestUser>,
    /// Monetary value associated with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl CreateEventsRequest {
    pub fn builder() -> CreateEventsRequestBuilder {
        <CreateEventsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEventsRequestBuilder {
    account_id: Option<String>,
    action_source: Option<CreateEventsRequestActionSource>,
    context: Option<CreateEventsRequestContext>,
    currency: Option<CreateEventsRequestCurrency>,
    custom_name: Option<String>,
    duration: Option<i64>,
    event_id: Option<String>,
    event_name: Option<String>,
    event_time: Option<DateTime<FixedOffset>>,
    plan_id: Option<String>,
    product_id: Option<String>,
    referrer_url: Option<String>,
    resumed: Option<bool>,
    source: Option<String>,
    title: Option<String>,
    url: Option<String>,
    user: Option<CreateEventsRequestUser>,
    value: Option<f64>,
}

impl CreateEventsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn action_source(mut self, value: CreateEventsRequestActionSource) -> Self {
        self.action_source = Some(value);
        self
    }

    pub fn context(mut self, value: CreateEventsRequestContext) -> Self {
        self.context = Some(value);
        self
    }

    pub fn currency(mut self, value: CreateEventsRequestCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
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

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
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

    pub fn resumed(mut self, value: bool) -> Self {
        self.resumed = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
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

    pub fn user(mut self, value: CreateEventsRequestUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateEventsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateEventsRequestBuilder::account_id)
    /// - [`event_name`](CreateEventsRequestBuilder::event_name)
    pub fn build(self) -> Result<CreateEventsRequest, BuildError> {
        Ok(CreateEventsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            action_source: self.action_source,
            context: self.context,
            currency: self.currency,
            custom_name: self.custom_name,
            duration: self.duration,
            event_id: self.event_id,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            event_time: self.event_time,
            plan_id: self.plan_id,
            product_id: self.product_id,
            referrer_url: self.referrer_url,
            resumed: self.resumed,
            source: self.source,
            title: self.title,
            url: self.url,
            user: self.user,
            value: self.value,
        })
    }
}
