pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWebhooksRequest {
    /// The dated API version (Api-Version-Date) to pin this webhook's payloads to. Only valid for `v1` webhooks. Omit to leave the current pin unchanged, or pass `null` to unpin and track the current payload shape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<String>,
    /// Whether or not to send events for child resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_resource_events: Option<bool>,
    /// Whether or not the webhook is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The events to send the webhook for, in dot form (for example `payment.succeeded`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<UpdateWebhooksRequestEventsItem>>,
    /// The URL to send the webhook to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpdateWebhooksRequest {
    pub fn builder() -> UpdateWebhooksRequestBuilder {
        <UpdateWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWebhooksRequestBuilder {
    api_version_date: Option<String>,
    child_resource_events: Option<bool>,
    enabled: Option<bool>,
    events: Option<Vec<UpdateWebhooksRequestEventsItem>>,
    url: Option<String>,
}

impl UpdateWebhooksRequestBuilder {
    pub fn api_version_date(mut self, value: impl Into<String>) -> Self {
        self.api_version_date = Some(value.into());
        self
    }

    pub fn child_resource_events(mut self, value: bool) -> Self {
        self.child_resource_events = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<UpdateWebhooksRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateWebhooksRequest`].
    pub fn build(self) -> Result<UpdateWebhooksRequest, BuildError> {
        Ok(UpdateWebhooksRequest {
            api_version_date: self.api_version_date,
            child_resource_events: self.child_resource_events,
            enabled: self.enabled,
            events: self.events,
            url: self.url,
        })
    }
}
