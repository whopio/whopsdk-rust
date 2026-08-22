pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPasskeysResponse {
    #[serde(default)]
    pub data: Vec<Passkey>,
    #[serde(default)]
    pub page_info: ListPasskeysResponsePageInfo,
}

impl ListPasskeysResponse {
    pub fn builder() -> ListPasskeysResponseBuilder {
        <ListPasskeysResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPasskeysResponseBuilder {
    data: Option<Vec<Passkey>>,
    page_info: Option<ListPasskeysResponsePageInfo>,
}

impl ListPasskeysResponseBuilder {
    pub fn data(mut self, value: Vec<Passkey>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListPasskeysResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPasskeysResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListPasskeysResponseBuilder::data)
    /// - [`page_info`](ListPasskeysResponseBuilder::page_info)
    pub fn build(self) -> Result<ListPasskeysResponse, BuildError> {
        Ok(ListPasskeysResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
