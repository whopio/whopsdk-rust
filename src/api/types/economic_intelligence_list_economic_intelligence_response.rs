pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEconomicIntelligenceResponse {
    #[serde(default)]
    pub data: Vec<EconomicIntelligence>,
    #[serde(default)]
    pub page_info: ListEconomicIntelligenceResponsePageInfo,
}

impl ListEconomicIntelligenceResponse {
    pub fn builder() -> ListEconomicIntelligenceResponseBuilder {
        <ListEconomicIntelligenceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEconomicIntelligenceResponseBuilder {
    data: Option<Vec<EconomicIntelligence>>,
    page_info: Option<ListEconomicIntelligenceResponsePageInfo>,
}

impl ListEconomicIntelligenceResponseBuilder {
    pub fn data(mut self, value: Vec<EconomicIntelligence>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListEconomicIntelligenceResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEconomicIntelligenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListEconomicIntelligenceResponseBuilder::data)
    /// - [`page_info`](ListEconomicIntelligenceResponseBuilder::page_info)
    pub fn build(self) -> Result<ListEconomicIntelligenceResponse, BuildError> {
        Ok(ListEconomicIntelligenceResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
