pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListApiKeysResponse {
    #[serde(default)]
    pub data: Vec<ApiKey>,
    #[serde(default)]
    pub page_info: ListApiKeysResponsePageInfo,
}

impl ListApiKeysResponse {
    pub fn builder() -> ListApiKeysResponseBuilder {
        <ListApiKeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListApiKeysResponseBuilder {
    data: Option<Vec<ApiKey>>,
    page_info: Option<ListApiKeysResponsePageInfo>,
}

impl ListApiKeysResponseBuilder {
    pub fn data(mut self, value: Vec<ApiKey>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListApiKeysResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListApiKeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListApiKeysResponseBuilder::data)
    /// - [`page_info`](ListApiKeysResponseBuilder::page_info)
    pub fn build(self) -> Result<ListApiKeysResponse, BuildError> {
        Ok(ListApiKeysResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
