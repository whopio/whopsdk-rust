pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMessagesResponse {
    #[serde(default)]
    pub data: Vec<ChatMessage>,
    #[serde(default)]
    pub page_info: ListMessagesResponsePageInfo,
}

impl ListMessagesResponse {
    pub fn builder() -> ListMessagesResponseBuilder {
        <ListMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMessagesResponseBuilder {
    data: Option<Vec<ChatMessage>>,
    page_info: Option<ListMessagesResponsePageInfo>,
}

impl ListMessagesResponseBuilder {
    pub fn data(mut self, value: Vec<ChatMessage>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListMessagesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMessagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListMessagesResponseBuilder::data)
    /// - [`page_info`](ListMessagesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListMessagesResponse, BuildError> {
        Ok(ListMessagesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
