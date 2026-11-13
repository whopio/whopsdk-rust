pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAppInstallationsResponse {
    #[serde(default)]
    pub data: Vec<AppInstallation>,
    #[serde(default)]
    pub page_info: ListAppInstallationsResponsePageInfo,
}

impl ListAppInstallationsResponse {
    pub fn builder() -> ListAppInstallationsResponseBuilder {
        <ListAppInstallationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAppInstallationsResponseBuilder {
    data: Option<Vec<AppInstallation>>,
    page_info: Option<ListAppInstallationsResponsePageInfo>,
}

impl ListAppInstallationsResponseBuilder {
    pub fn data(mut self, value: Vec<AppInstallation>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListAppInstallationsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAppInstallationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListAppInstallationsResponseBuilder::data)
    /// - [`page_info`](ListAppInstallationsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListAppInstallationsResponse, BuildError> {
        Ok(ListAppInstallationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
