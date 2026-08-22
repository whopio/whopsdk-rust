pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTopicsResponse {
    #[serde(default)]
    pub data: Vec<NotificationTopic>,
    #[serde(default)]
    pub page_info: ListTopicsResponsePageInfo,
}

impl ListTopicsResponse {
    pub fn builder() -> ListTopicsResponseBuilder {
        <ListTopicsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTopicsResponseBuilder {
    data: Option<Vec<NotificationTopic>>,
    page_info: Option<ListTopicsResponsePageInfo>,
}

impl ListTopicsResponseBuilder {
    pub fn data(mut self, value: Vec<NotificationTopic>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListTopicsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTopicsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListTopicsResponseBuilder::data)
    /// - [`page_info`](ListTopicsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListTopicsResponse, BuildError> {
        Ok(ListTopicsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
