pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPermissionsResponse {
    #[serde(default)]
    pub data: Vec<ListPermissionsResponseDataItem>,
}

impl ListPermissionsResponse {
    pub fn builder() -> ListPermissionsResponseBuilder {
        <ListPermissionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPermissionsResponseBuilder {
    data: Option<Vec<ListPermissionsResponseDataItem>>,
}

impl ListPermissionsResponseBuilder {
    pub fn data(mut self, value: Vec<ListPermissionsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPermissionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPermissionsResponseBuilder::data)
    pub fn build(self) -> Result<ListPermissionsResponse, BuildError> {
        Ok(ListPermissionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
