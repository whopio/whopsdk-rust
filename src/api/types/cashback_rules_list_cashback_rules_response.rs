pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCashbackRulesResponse {
    #[serde(default)]
    pub data: Vec<CashbackRule>,
    #[serde(default)]
    pub page_info: ListCashbackRulesResponsePageInfo,
}

impl ListCashbackRulesResponse {
    pub fn builder() -> ListCashbackRulesResponseBuilder {
        <ListCashbackRulesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCashbackRulesResponseBuilder {
    data: Option<Vec<CashbackRule>>,
    page_info: Option<ListCashbackRulesResponsePageInfo>,
}

impl ListCashbackRulesResponseBuilder {
    pub fn data(mut self, value: Vec<CashbackRule>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListCashbackRulesResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCashbackRulesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCashbackRulesResponseBuilder::data)
    /// - [`page_info`](ListCashbackRulesResponseBuilder::page_info)
    pub fn build(self) -> Result<ListCashbackRulesResponse, BuildError> {
        Ok(ListCashbackRulesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
