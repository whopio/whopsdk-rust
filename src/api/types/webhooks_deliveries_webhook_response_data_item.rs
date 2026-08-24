pub use crate::prelude::*;

/// A webhook log entry containing the request and response details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeliveriesWebhookResponseDataItem {
    /// The request body sent to the webhook endpoint
    #[serde(default)]
    pub request_body: HashMap<String, serde_json::Value>,
    /// The ID of the resource that triggered the webhook
    #[serde(default)]
    pub resource_id: String,
    /// The response body received from the webhook endpoint
    #[serde(default)]
    pub response_body: HashMap<String, serde_json::Value>,
    /// The HTTP response code received from the webhook endpoint
    #[serde(default)]
    pub response_code: i64,
    /// The timestamp when the webhook was sent
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub sent_at: DateTime<FixedOffset>,
    /// The total time taken to send the webhook request in seconds
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_time: f64,
}

impl DeliveriesWebhookResponseDataItem {
    pub fn builder() -> DeliveriesWebhookResponseDataItemBuilder {
        <DeliveriesWebhookResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesWebhookResponseDataItemBuilder {
    request_body: Option<HashMap<String, serde_json::Value>>,
    resource_id: Option<String>,
    response_body: Option<HashMap<String, serde_json::Value>>,
    response_code: Option<i64>,
    sent_at: Option<DateTime<FixedOffset>>,
    total_time: Option<f64>,
}

impl DeliveriesWebhookResponseDataItemBuilder {
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

    pub fn response_code(mut self, value: i64) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn sent_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.sent_at = Some(value);
        self
    }

    pub fn total_time(mut self, value: f64) -> Self {
        self.total_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesWebhookResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`request_body`](DeliveriesWebhookResponseDataItemBuilder::request_body)
    /// - [`resource_id`](DeliveriesWebhookResponseDataItemBuilder::resource_id)
    /// - [`response_body`](DeliveriesWebhookResponseDataItemBuilder::response_body)
    /// - [`response_code`](DeliveriesWebhookResponseDataItemBuilder::response_code)
    /// - [`sent_at`](DeliveriesWebhookResponseDataItemBuilder::sent_at)
    /// - [`total_time`](DeliveriesWebhookResponseDataItemBuilder::total_time)
    pub fn build(self) -> Result<DeliveriesWebhookResponseDataItem, BuildError> {
        Ok(DeliveriesWebhookResponseDataItem {
            request_body: self
                .request_body
                .ok_or_else(|| BuildError::missing_field("request_body"))?,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            response_body: self
                .response_body
                .ok_or_else(|| BuildError::missing_field("response_body"))?,
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            sent_at: self
                .sent_at
                .ok_or_else(|| BuildError::missing_field("sent_at"))?,
            total_time: self
                .total_time
                .ok_or_else(|| BuildError::missing_field("total_time"))?,
        })
    }
}
