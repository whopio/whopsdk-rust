pub use crate::prelude::*;

/// The connection type for PublicWithdrawal.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListWithdrawalsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<WithdrawalListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListWithdrawalsResponse {
    pub fn builder() -> ListWithdrawalsResponseBuilder {
        <ListWithdrawalsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWithdrawalsResponseBuilder {
    data: Option<Vec<WithdrawalListItem>>,
    page_info: Option<PageInfo>,
}

impl ListWithdrawalsResponseBuilder {
    pub fn data(mut self, value: Vec<WithdrawalListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWithdrawalsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListWithdrawalsResponseBuilder::data)
    /// - [`page_info`](ListWithdrawalsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListWithdrawalsResponse, BuildError> {
        Ok(ListWithdrawalsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
