pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListReactionsMessagesResponse {
    #[serde(default)]
    pub data: Vec<ChatReaction>,
    #[serde(default)]
    pub page_info: ListReactionsMessagesResponsePageInfo,
}

impl ListReactionsMessagesResponse {
    pub fn builder() -> ListReactionsMessagesResponseBuilder {
        <ListReactionsMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReactionsMessagesResponseBuilder {
    data: Option<Vec<ChatReaction>>,
    page_info: Option<ListReactionsMessagesResponsePageInfo>,
}

impl ListReactionsMessagesResponseBuilder {
    pub fn data(mut self, value: Vec<ChatReaction>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListReactionsMessagesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReactionsMessagesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListReactionsMessagesResponseBuilder::data)
    /// - [`page_info`](ListReactionsMessagesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListReactionsMessagesResponse, BuildError> {
        Ok(ListReactionsMessagesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
