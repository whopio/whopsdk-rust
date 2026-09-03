pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRefundsResponse {
    #[serde(default)]
    pub data: Vec<Refund>,
    #[serde(default)]
    pub page_info: ListRefundsResponsePageInfo,
}

impl ListRefundsResponse {
    pub fn builder() -> ListRefundsResponseBuilder {
        <ListRefundsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRefundsResponseBuilder {
    data: Option<Vec<Refund>>,
    page_info: Option<ListRefundsResponsePageInfo>,
}

impl ListRefundsResponseBuilder {
    pub fn data(mut self, value: Vec<Refund>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListRefundsResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListRefundsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListRefundsResponseBuilder::data)
    /// - [`page_info`](ListRefundsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListRefundsResponse, BuildError> {
        Ok(ListRefundsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
