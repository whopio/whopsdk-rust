pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListApiLogsResponse {
    #[serde(default)]
    pub data: Vec<ListApiLogsResponseDataItem>,
    #[serde(default)]
    pub page_info: ListApiLogsResponsePageInfo,
}

impl ListApiLogsResponse {
    pub fn builder() -> ListApiLogsResponseBuilder {
        <ListApiLogsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListApiLogsResponseBuilder {
    data: Option<Vec<ListApiLogsResponseDataItem>>,
    page_info: Option<ListApiLogsResponsePageInfo>,
}

impl ListApiLogsResponseBuilder {
    pub fn data(mut self, value: Vec<ListApiLogsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListApiLogsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListApiLogsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListApiLogsResponseBuilder::data)
    /// - [`page_info`](ListApiLogsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListApiLogsResponse, BuildError> {
        Ok(ListApiLogsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
