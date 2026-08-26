pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFilesResponse {
    #[serde(default)]
    pub data: Vec<File>,
    #[serde(default)]
    pub page_info: ListFilesResponsePageInfo,
}

impl ListFilesResponse {
    pub fn builder() -> ListFilesResponseBuilder {
        <ListFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFilesResponseBuilder {
    data: Option<Vec<File>>,
    page_info: Option<ListFilesResponsePageInfo>,
}

impl ListFilesResponseBuilder {
    pub fn data(mut self, value: Vec<File>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListFilesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFilesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListFilesResponseBuilder::data)
    /// - [`page_info`](ListFilesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListFilesResponse, BuildError> {
        Ok(ListFilesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
