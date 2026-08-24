pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAppsResponse {
    #[serde(default)]
    pub data: Vec<AppListItem>,
    #[serde(default)]
    pub page_info: ListAppsResponsePageInfo,
}

impl ListAppsResponse {
    pub fn builder() -> ListAppsResponseBuilder {
        <ListAppsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAppsResponseBuilder {
    data: Option<Vec<AppListItem>>,
    page_info: Option<ListAppsResponsePageInfo>,
}

impl ListAppsResponseBuilder {
    pub fn data(mut self, value: Vec<AppListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAppsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAppsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAppsResponseBuilder::data)
    /// - [`page_info`](ListAppsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAppsResponse, BuildError> {
        Ok(ListAppsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
