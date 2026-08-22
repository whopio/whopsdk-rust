pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListExportsResponse {
    #[serde(default)]
    pub data: Vec<Export>,
    #[serde(default)]
    pub page_info: ListExportsResponsePageInfo,
}

impl ListExportsResponse {
    pub fn builder() -> ListExportsResponseBuilder {
        <ListExportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExportsResponseBuilder {
    data: Option<Vec<Export>>,
    page_info: Option<ListExportsResponsePageInfo>,
}

impl ListExportsResponseBuilder {
    pub fn data(mut self, value: Vec<Export>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListExportsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListExportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListExportsResponseBuilder::data)
    /// - [`page_info`](ListExportsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListExportsResponse, BuildError> {
        Ok(ListExportsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
