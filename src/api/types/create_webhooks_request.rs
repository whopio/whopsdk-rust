pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhooksRequest {
    /// The dated API version (Api-Version-Date) to pin this webhook's payloads to. Omit to leave the webhook unpinned, tracking the current payload shape.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<String>,
    /// Whether to send events for child resources. For example, if the webhook is created for an account, enabling this sends events only from its connected accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_resource_events: Option<bool>,
    /// Whether or not the webhook is enabled. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The events to send the webhook for, in dot form (for example `payment.succeeded`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<CreateWebhooksRequestEventsItem>>,
    /// The account or app to create the webhook for. Defaults to the current account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// The URL to send the webhook to.
    #[serde(default)]
    pub url: String,
}

impl CreateWebhooksRequest {
    pub fn builder() -> CreateWebhooksRequestBuilder {
        <CreateWebhooksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhooksRequestBuilder {
    api_version_date: Option<String>,
    child_resource_events: Option<bool>,
    enabled: Option<bool>,
    events: Option<Vec<CreateWebhooksRequestEventsItem>>,
    resource_id: Option<String>,
    url: Option<String>,
}

impl CreateWebhooksRequestBuilder {
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

    pub fn events(mut self, value: Vec<CreateWebhooksRequestEventsItem>) -> Self {
        self.events = Some(value);
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

    /// Consumes the builder and constructs a [`CreateWebhooksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateWebhooksRequestBuilder::url)
    pub fn build(self) -> Result<CreateWebhooksRequest, BuildError> {
        Ok(CreateWebhooksRequest {
            api_version_date: self.api_version_date,
            child_resource_events: self.child_resource_events,
            enabled: self.enabled,
            events: self.events,
            resource_id: self.resource_id,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
