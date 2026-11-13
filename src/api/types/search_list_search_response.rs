pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSearchResponse {
    #[serde(default)]
    pub data: Vec<SearchResult>,
    #[serde(default)]
    pub page_info: ListSearchResponsePageInfo,
}

impl ListSearchResponse {
    pub fn builder() -> ListSearchResponseBuilder {
        <ListSearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSearchResponseBuilder {
    data: Option<Vec<SearchResult>>,
    page_info: Option<ListSearchResponsePageInfo>,
}

impl ListSearchResponseBuilder {
    pub fn data(mut self, value: Vec<SearchResult>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListSearchResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSearchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSearchResponseBuilder::data)
    /// - [`page_info`](ListSearchResponseBuilder::page_info)
    pub fn build(self) -> Result<ListSearchResponse, BuildError> {
        Ok(ListSearchResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
