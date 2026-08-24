pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCheckoutConfigurationsResponse {
    #[serde(default)]
    pub data: Vec<ListCheckoutConfigurationsResponseDataItem>,
    #[serde(default)]
    pub page_info: ListCheckoutConfigurationsResponsePageInfo,
}

impl ListCheckoutConfigurationsResponse {
    pub fn builder() -> ListCheckoutConfigurationsResponseBuilder {
        <ListCheckoutConfigurationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCheckoutConfigurationsResponseBuilder {
    data: Option<Vec<ListCheckoutConfigurationsResponseDataItem>>,
    page_info: Option<ListCheckoutConfigurationsResponsePageInfo>,
}

impl ListCheckoutConfigurationsResponseBuilder {
    pub fn data(mut self, value: Vec<ListCheckoutConfigurationsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListCheckoutConfigurationsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCheckoutConfigurationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCheckoutConfigurationsResponseBuilder::data)
    /// - [`page_info`](ListCheckoutConfigurationsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCheckoutConfigurationsResponse, BuildError> {
        Ok(ListCheckoutConfigurationsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
