pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhooksResponse {
    #[serde(default)]
    pub data: Vec<WebhookListItem>,
    #[serde(default)]
    pub page_info: ListWebhooksResponsePageInfo,
}

impl ListWebhooksResponse {
    pub fn builder() -> ListWebhooksResponseBuilder {
        <ListWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhooksResponseBuilder {
    data: Option<Vec<WebhookListItem>>,
    page_info: Option<ListWebhooksResponsePageInfo>,
}

impl ListWebhooksResponseBuilder {
    pub fn data(mut self, value: Vec<WebhookListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListWebhooksResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhooksResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListWebhooksResponseBuilder::data)
    /// - [`page_info`](ListWebhooksResponseBuilder::page_info)
    pub fn build(self) -> Result<ListWebhooksResponse, BuildError> {
        Ok(ListWebhooksResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
