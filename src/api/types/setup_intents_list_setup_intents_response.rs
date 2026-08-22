pub use crate::prelude::*;

/// The connection type for SetupIntent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSetupIntentsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<SetupIntentListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListSetupIntentsResponse {
    pub fn builder() -> ListSetupIntentsResponseBuilder {
        <ListSetupIntentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSetupIntentsResponseBuilder {
    data: Option<Vec<SetupIntentListItem>>,
    page_info: Option<PageInfo>,
}

impl ListSetupIntentsResponseBuilder {
    pub fn data(mut self, value: Vec<SetupIntentListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
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
