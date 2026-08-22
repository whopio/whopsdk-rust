pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhookDelivery {
    /// The event type this delivery carried, for example `payment.succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Unique identifier for this delivery attempt. Pass it to the replay endpoint to re-send this exact payload.
    #[serde(default)]
    pub id: String,
    /// The id of the delivery attempt this one replayed. `null` for an original delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replayed_from: Option<String>,
    /// The JSON event payload sent to the webhook endpoint.
    #[serde(default)]
    pub request_body: HashMap<String, serde_json::Value>,
    /// ID of the resource that triggered the webhook.
    #[serde(default)]
    pub resource_id: String,
    /// The endpoint's JSON response. A non-JSON response is stored as `{ error, raw_body }` with the first 100 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body: Option<HashMap<String, serde_json::Value>>,
    /// HTTP response code received from the webhook endpoint.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub response_code: f64,
    /// When the webhook was sent, as an ISO 8601 timestamp.
    #[serde(default)]
    pub sent_at: String,
    /// Whether the endpoint acknowledged this attempt with a 2xx response.
    #[serde(default)]
    pub success: bool,
    /// Total time taken to send the webhook request, in seconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_time: f64,
}

impl WebhookDelivery {
    pub fn builder() -> WebhookDeliveryBuilder {
        <WebhookDeliveryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookDeliveryBuilder {
    event: Option<String>,
    id: Option<String>,
    replayed_from: Option<String>,
    request_body: Option<HashMap<String, serde_json::Value>>,
    resource_id: Option<String>,
    response_body: Option<HashMap<String, serde_json::Value>>,
    response_code: Option<f64>,
    sent_at: Option<String>,
    success: Option<bool>,
    total_time: Option<f64>,
}

impl WebhookDeliveryBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn replayed_from(mut self, value: impl Into<String>) -> Self {
        self.replayed_from = Some(value.into());
        self
    }

    pub fn request_body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.request_body = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn response_body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.response_body = Some(value);
        self
    }

    pub fn response_code(mut self, value: f64) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn sent_at(mut self, value: impl Into<String>) -> Self {
        self.sent_at = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn total_time(mut self, value: f64) -> Self {
        self.total_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhookDelivery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookDeliveryBuilder::id)
    /// - [`request_body`](WebhookDeliveryBuilder::request_body)
    /// - [`resource_id`](WebhookDeliveryBuilder::resource_id)
    /// - [`response_code`](WebhookDeliveryBuilder::response_code)
    /// - [`sent_at`](WebhookDeliveryBuilder::sent_at)
    /// - [`success`](WebhookDeliveryBuilder::success)
    /// - [`total_time`](WebhookDeliveryBuilder::total_time)
    pub fn build(self) -> Result<WebhookDelivery, BuildError> {
        Ok(WebhookDelivery {
            event: self.event,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            replayed_from: self.replayed_from,
            request_body: self
                .request_body
                .ok_or_else(|| BuildError::missing_field("request_body"))?,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            response_body: self.response_body,
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            sent_at: self
                .sent_at
                .ok_or_else(|| BuildError::missing_field("sent_at"))?,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            total_time: self
                .total_time
                .ok_or_else(|| BuildError::missing_field("total_time"))?,
        })
    }
}
