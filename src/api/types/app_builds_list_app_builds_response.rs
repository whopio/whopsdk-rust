pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAppBuildsResponse {
    #[serde(default)]
    pub data: Vec<AppBuild>,
    #[serde(default)]
    pub page_info: ListAppBuildsResponsePageInfo,
}

impl ListAppBuildsResponse {
    pub fn builder() -> ListAppBuildsResponseBuilder {
        <ListAppBuildsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAppBuildsResponseBuilder {
    data: Option<Vec<AppBuild>>,
    page_info: Option<ListAppBuildsResponsePageInfo>,
}

impl ListAppBuildsResponseBuilder {
    pub fn data(mut self, value: Vec<AppBuild>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAppBuildsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAppBuildsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAppBuildsResponseBuilder::data)
    /// - [`page_info`](ListAppBuildsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAppBuildsResponse, BuildError> {
        Ok(ListAppBuildsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
