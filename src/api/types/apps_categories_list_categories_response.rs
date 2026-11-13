pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCategoriesResponse {
    #[serde(default)]
    pub data: Vec<AppCategory>,
    #[serde(default)]
    pub page_info: ListCategoriesResponsePageInfo,
}

impl ListCategoriesResponse {
    pub fn builder() -> ListCategoriesResponseBuilder {
        <ListCategoriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCategoriesResponseBuilder {
    data: Option<Vec<AppCategory>>,
    page_info: Option<ListCategoriesResponsePageInfo>,
}

impl ListCategoriesResponseBuilder {
    pub fn data(mut self, value: Vec<AppCategory>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListCategoriesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCategoriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCategoriesResponseBuilder::data)
    /// - [`page_info`](ListCategoriesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCategoriesResponse, BuildError> {
        Ok(ListCategoriesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
