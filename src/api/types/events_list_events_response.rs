pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEventsResponse {
    #[serde(default)]
    pub data: Vec<ListEventsResponseDataItem>,
    #[serde(default)]
    pub page_info: ListEventsResponsePageInfo,
}

impl ListEventsResponse {
    pub fn builder() -> ListEventsResponseBuilder {
        <ListEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseBuilder {
    data: Option<Vec<ListEventsResponseDataItem>>,
    page_info: Option<ListEventsResponsePageInfo>,
}

impl ListEventsResponseBuilder {
    pub fn data(mut self, value: Vec<ListEventsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListEventsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListEventsResponseBuilder::data)
    /// - [`page_info`](ListEventsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListEventsResponse, BuildError> {
        Ok(ListEventsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
