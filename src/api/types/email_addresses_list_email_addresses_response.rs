pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEmailAddressesResponse {
    #[serde(default)]
    pub data: Vec<EmailAddress>,
    #[serde(default)]
    pub page_info: ListEmailAddressesResponsePageInfo,
}

impl ListEmailAddressesResponse {
    pub fn builder() -> ListEmailAddressesResponseBuilder {
        <ListEmailAddressesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEmailAddressesResponseBuilder {
    data: Option<Vec<EmailAddress>>,
    page_info: Option<ListEmailAddressesResponsePageInfo>,
}

impl ListEmailAddressesResponseBuilder {
    pub fn data(mut self, value: Vec<EmailAddress>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListEmailAddressesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEmailAddressesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListEmailAddressesResponseBuilder::data)
    /// - [`page_info`](ListEmailAddressesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListEmailAddressesResponse, BuildError> {
        Ok(ListEmailAddressesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
