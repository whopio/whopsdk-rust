pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPayoutsResponse {
    #[serde(default)]
    pub data: Vec<ListPayoutsResponseDataItem>,
    #[serde(default)]
    pub page_info: ListPayoutsResponsePageInfo,
}

impl ListPayoutsResponse {
    pub fn builder() -> ListPayoutsResponseBuilder {
        <ListPayoutsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPayoutsResponseBuilder {
    data: Option<Vec<ListPayoutsResponseDataItem>>,
    page_info: Option<ListPayoutsResponsePageInfo>,
}

impl ListPayoutsResponseBuilder {
    pub fn data(mut self, value: Vec<ListPayoutsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPayoutsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPayoutsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPayoutsResponseBuilder::data)
    /// - [`page_info`](ListPayoutsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPayoutsResponse, BuildError> {
        Ok(ListPayoutsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
