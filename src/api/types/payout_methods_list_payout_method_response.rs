pub use crate::prelude::*;

/// The connection type for PayoutToken.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPayoutMethodResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<PayoutMethodListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListPayoutMethodResponse {
    pub fn builder() -> ListPayoutMethodResponseBuilder {
        <ListPayoutMethodResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPayoutMethodResponseBuilder {
    data: Option<Vec<PayoutMethodListItem>>,
    page_info: Option<PageInfo>,
}

impl ListPayoutMethodResponseBuilder {
    pub fn data(mut self, value: Vec<PayoutMethodListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPayoutMethodResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPayoutMethodResponseBuilder::data)
    /// - [`page_info`](ListPayoutMethodResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPayoutMethodResponse, BuildError> {
        Ok(ListPayoutMethodResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
