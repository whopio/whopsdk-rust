pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListRecipientsTransfersResponse {
    #[serde(default)]
    pub data: Vec<ListRecipientsTransfersResponseDataItem>,
    #[serde(default)]
    pub page_info: ListRecipientsTransfersResponsePageInfo,
}

impl ListRecipientsTransfersResponse {
    pub fn builder() -> ListRecipientsTransfersResponseBuilder {
        <ListRecipientsTransfersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRecipientsTransfersResponseBuilder {
    data: Option<Vec<ListRecipientsTransfersResponseDataItem>>,
    page_info: Option<ListRecipientsTransfersResponsePageInfo>,
}

impl ListRecipientsTransfersResponseBuilder {
    pub fn data(mut self, value: Vec<ListRecipientsTransfersResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListRecipientsTransfersResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRecipientsTransfersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListRecipientsTransfersResponseBuilder::data)
    /// - [`page_info`](ListRecipientsTransfersResponseBuilder::page_info)
    pub fn build(self) -> Result<ListRecipientsTransfersResponse, BuildError> {
        Ok(ListRecipientsTransfersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
