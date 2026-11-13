pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSetupIntentsResponse {
    #[serde(default)]
    pub data: Vec<SetupIntent>,
    #[serde(default)]
    pub page_info: ListSetupIntentsResponsePageInfo,
}

impl ListSetupIntentsResponse {
    pub fn builder() -> ListSetupIntentsResponseBuilder {
        <ListSetupIntentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSetupIntentsResponseBuilder {
    data: Option<Vec<SetupIntent>>,
    page_info: Option<ListSetupIntentsResponsePageInfo>,
}

impl ListSetupIntentsResponseBuilder {
    pub fn data(mut self, value: Vec<SetupIntent>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListSetupIntentsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSetupIntentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSetupIntentsResponseBuilder::data)
    /// - [`page_info`](ListSetupIntentsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListSetupIntentsResponse, BuildError> {
        Ok(ListSetupIntentsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
