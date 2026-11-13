pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListVerificationsResponse {
    #[serde(default)]
    pub data: Vec<ListVerificationsResponseDataItem>,
    #[serde(default)]
    pub page_info: ListVerificationsResponsePageInfo,
}

impl ListVerificationsResponse {
    pub fn builder() -> ListVerificationsResponseBuilder {
        <ListVerificationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVerificationsResponseBuilder {
    data: Option<Vec<ListVerificationsResponseDataItem>>,
    page_info: Option<ListVerificationsResponsePageInfo>,
}

impl ListVerificationsResponseBuilder {
    pub fn data(mut self, value: Vec<ListVerificationsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListVerificationsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVerificationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListVerificationsResponseBuilder::data)
    /// - [`page_info`](ListVerificationsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListVerificationsResponse, BuildError> {
        Ok(ListVerificationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
