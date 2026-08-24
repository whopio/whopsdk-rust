pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDeliveriesWebhooksResponse {
    #[serde(default)]
    pub data: Vec<WebhookDelivery>,
    #[serde(default)]
    pub page_info: ListDeliveriesWebhooksResponsePageInfo,
}

impl ListDeliveriesWebhooksResponse {
    pub fn builder() -> ListDeliveriesWebhooksResponseBuilder {
        <ListDeliveriesWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDeliveriesWebhooksResponseBuilder {
    data: Option<Vec<WebhookDelivery>>,
    page_info: Option<ListDeliveriesWebhooksResponsePageInfo>,
}

impl ListDeliveriesWebhooksResponseBuilder {
    pub fn data(mut self, value: Vec<WebhookDelivery>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListDeliveriesWebhooksResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDeliveriesWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListDeliveriesWebhooksResponseBuilder::data)
    /// - [`page_info`](ListDeliveriesWebhooksResponseBuilder::page_info)
    pub fn build(self) -> Result<ListDeliveriesWebhooksResponse, BuildError> {
        Ok(ListDeliveriesWebhooksResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
