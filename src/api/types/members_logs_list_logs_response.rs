pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLogsResponse {
    #[serde(default)]
    pub data: Vec<ListLogsResponseDataItem>,
    #[serde(default)]
    pub page_info: ListLogsResponsePageInfo,
}

impl ListLogsResponse {
    pub fn builder() -> ListLogsResponseBuilder {
        <ListLogsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLogsResponseBuilder {
    data: Option<Vec<ListLogsResponseDataItem>>,
    page_info: Option<ListLogsResponsePageInfo>,
}

impl ListLogsResponseBuilder {
    pub fn data(mut self, value: Vec<ListLogsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListLogsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLogsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListLogsResponseBuilder::data)
    /// - [`page_info`](ListLogsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListLogsResponse, BuildError> {
        Ok(ListLogsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
