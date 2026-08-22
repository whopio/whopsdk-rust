pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebhookListItem {
    /// The API version used to format payloads sent to this webhook endpoint.
    pub api_version: WebhookListItemApiVersion,
    /// The dated API version (Api-Version-Date) that v1 payloads for this endpoint are pinned to: events serialize exactly like a REST read at this version (the native serializer where the resource has one). Null when unpinned — legacy (v2/v5) webhooks, and v1 webhooks on the legacy payload shape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<String>,
    /// Whether events are sent for child resources. For example, if the webhook is on an account, enabling this sends events only from its connected accounts.
    #[serde(default)]
    pub child_resource_events: bool,
    /// Number of consecutive deliveries whose first attempt to this endpoint failed since it last accepted one. Later retries of the same delivery do not increment it. Resets to `0` when a delivery succeeds or the webhook is re-enabled.
    #[serde(default)]
    pub consecutive_failures: i64,
    /// When the webhook was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When Whop automatically disabled this webhook, as an ISO 8601 timestamp. `null` unless the webhook was disabled by Whop; a webhook you disabled yourself has `enabled: false` and a `null` `disabled_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_at: Option<String>,
    /// Why Whop disabled this webhook. `delivery_failures` means every delivery failed for 3 days straight. `null` when `disabled_at` is `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<WebhookListItemDisabledReason>,
    /// Whether this webhook endpoint is currently active and receiving events.
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub events: Vec<WebhookListItemEventsItem>,
    /// When the current failure streak began, as an ISO 8601 timestamp. Unlike `last_failure_at`, this is set on the streak's first failed attempt, so it shows an endpoint that is failing right now. `null` when the endpoint is healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failing_since: Option<String>,
    /// Webhook ID, prefixed `hook_`.
    #[serde(default)]
    pub id: String,
    /// When a delivery to this endpoint most recently failed after exhausting retries, as an ISO 8601 timestamp. `null` if no delivery has ever failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_at: Option<String>,
    /// ID of the resource (account or app) this webhook is attached to.
    #[serde(default)]
    pub resource_id: String,
    /// Destination URL where webhook payloads are delivered via HTTP POST.
    #[serde(default)]
    pub url: String,
    /// Secret key used to sign webhook payloads for verification. Include this in your HMAC validation logic. Returned on the create response and to interactive dashboard sessions; `null` for API-key and OAuth callers on later reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_secret: Option<String>,
}

impl WebhookListItem {
    pub fn builder() -> WebhookListItemBuilder {
        <WebhookListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookListItemBuilder {
    api_version: Option<WebhookListItemApiVersion>,
    api_version_date: Option<String>,
    child_resource_events: Option<bool>,
    consecutive_failures: Option<i64>,
    created_at: Option<String>,
    disabled_at: Option<String>,
    disabled_reason: Option<WebhookListItemDisabledReason>,
    enabled: Option<bool>,
    events: Option<Vec<WebhookListItemEventsItem>>,
    failing_since: Option<String>,
    id: Option<String>,
    last_failure_at: Option<String>,
    resource_id: Option<String>,
    url: Option<String>,
    webhook_secret: Option<String>,
}

impl WebhookListItemBuilder {
    pub fn api_version(mut self, value: WebhookListItemApiVersion) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn api_version_date(mut self, value: impl Into<String>) -> Self {
        self.api_version_date = Some(value.into());
        self
    }

    pub fn child_resource_events(mut self, value: bool) -> Self {
        self.child_resource_events = Some(value);
        self
    }

    pub fn consecutive_failures(mut self, value: i64) -> Self {
        self.consecutive_failures = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn disabled_at(mut self, value: impl Into<String>) -> Self {
        self.disabled_at = Some(value.into());
        self
    }

    pub fn disabled_reason(mut self, value: WebhookListItemDisabledReason) -> Self {
        self.disabled_reason = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<WebhookListItemEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn failing_since(mut self, value: impl Into<String>) -> Self {
        self.failing_since = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_failure_at(mut self, value: impl Into<String>) -> Self {
        self.last_failure_at = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn webhook_secret(mut self, value: impl Into<String>) -> Self {
        self.webhook_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](WebhookListItemBuilder::api_version)
    /// - [`child_resource_events`](WebhookListItemBuilder::child_resource_events)
    /// - [`consecutive_failures`](WebhookListItemBuilder::consecutive_failures)
    /// - [`created_at`](WebhookListItemBuilder::created_at)
    /// - [`enabled`](WebhookListItemBuilder::enabled)
    /// - [`events`](WebhookListItemBuilder::events)
    /// - [`id`](WebhookListItemBuilder::id)
    /// - [`resource_id`](WebhookListItemBuilder::resource_id)
    /// - [`url`](WebhookListItemBuilder::url)
    pub fn build(self) -> Result<WebhookListItem, BuildError> {
        Ok(WebhookListItem {
            api_version: self
                .api_version
                .ok_or_else(|| BuildError::missing_field("api_version"))?,
            api_version_date: self.api_version_date,
            child_resource_events: self
                .child_resource_events
                .ok_or_else(|| BuildError::missing_field("child_resource_events"))?,
            consecutive_failures: self
                .consecutive_failures
                .ok_or_else(|| BuildError::missing_field("consecutive_failures"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            disabled_at: self.disabled_at,
            disabled_reason: self.disabled_reason,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            failing_since: self.failing_since,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_failure_at: self.last_failure_at,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            webhook_secret: self.webhook_secret,
        })
    }
}
