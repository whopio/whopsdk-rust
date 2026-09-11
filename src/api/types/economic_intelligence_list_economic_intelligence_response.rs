pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEconomicIntelligenceResponse {
    #[serde(default)]
    pub data: Vec<EconomicIntelligence>,
    /// Whether a generation is running because the account has no ready recommendations.
    #[serde(default)]
    pub generation_pending: bool,
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
    generation_pending: Option<bool>,
    page_info: Option<ListEconomicIntelligenceResponsePageInfo>,
}

impl ListEconomicIntelligenceResponseBuilder {
    pub fn data(mut self, value: Vec<EconomicIntelligence>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn generation_pending(mut self, value: bool) -> Self {
        self.generation_pending = Some(value);
        self
    }

    pub fn page_info(mut self, value: ListEconomicIntelligenceResponsePageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEconomicIntelligenceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListEconomicIntelligenceResponseBuilder::data)
    /// - [`generation_pending`](ListEconomicIntelligenceResponseBuilder::generation_pending)
    /// - [`page_info`](ListEconomicIntelligenceResponseBuilder::page_info)
    pub fn build(self) -> Result<ListEconomicIntelligenceResponse, BuildError> {
        Ok(ListEconomicIntelligenceResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            generation_pending: self
                .generation_pending
                .ok_or_else(|| BuildError::missing_field("generation_pending"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
