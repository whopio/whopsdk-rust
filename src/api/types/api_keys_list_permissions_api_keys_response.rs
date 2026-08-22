pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPermissionsApiKeysResponse {
    #[serde(default)]
    pub data: Vec<Permission>,
    #[serde(default)]
    pub page_info: ListPermissionsApiKeysResponsePageInfo,
}

impl ListPermissionsApiKeysResponse {
    pub fn builder() -> ListPermissionsApiKeysResponseBuilder {
        <ListPermissionsApiKeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPermissionsApiKeysResponseBuilder {
    data: Option<Vec<Permission>>,
    page_info: Option<ListPermissionsApiKeysResponsePageInfo>,
}

impl ListPermissionsApiKeysResponseBuilder {
    pub fn data(mut self, value: Vec<Permission>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPermissionsApiKeysResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPermissionsApiKeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPermissionsApiKeysResponseBuilder::data)
    /// - [`page_info`](ListPermissionsApiKeysResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPermissionsApiKeysResponse, BuildError> {
        Ok(ListPermissionsApiKeysResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
