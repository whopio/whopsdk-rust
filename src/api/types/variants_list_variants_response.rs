pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListVariantsResponse {
    #[serde(default)]
    pub data: Vec<VariantListItem>,
    #[serde(default)]
    pub page_info: ListVariantsResponsePageInfo,
}

impl ListVariantsResponse {
    pub fn builder() -> ListVariantsResponseBuilder {
        <ListVariantsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVariantsResponseBuilder {
    data: Option<Vec<VariantListItem>>,
    page_info: Option<ListVariantsResponsePageInfo>,
}

impl ListVariantsResponseBuilder {
    pub fn data(mut self, value: Vec<VariantListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListVariantsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVariantsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListVariantsResponseBuilder::data)
    /// - [`page_info`](ListVariantsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListVariantsResponse, BuildError> {
        Ok(ListVariantsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
