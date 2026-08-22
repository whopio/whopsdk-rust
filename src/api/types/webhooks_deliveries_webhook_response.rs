pub use crate::prelude::*;

/// The connection type for WebhookLog.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeliveriesWebhookResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<DeliveriesWebhookResponseDataItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: DeliveriesWebhookResponsePageInfo,
}

impl DeliveriesWebhookResponse {
    pub fn builder() -> DeliveriesWebhookResponseBuilder {
        <DeliveriesWebhookResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesWebhookResponseBuilder {
    data: Option<Vec<DeliveriesWebhookResponseDataItem>>,
    page_info: Option<DeliveriesWebhookResponsePageInfo>,
}

impl DeliveriesWebhookResponseBuilder {
    pub fn data(mut self, value: Vec<DeliveriesWebhookResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: DeliveriesWebhookResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesWebhookResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](DeliveriesWebhookResponseBuilder::data)
    /// - [`page_info`](DeliveriesWebhookResponseBuilder::page_info)
    pub fn build(self) -> Result<DeliveriesWebhookResponse, BuildError> {
        Ok(DeliveriesWebhookResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
