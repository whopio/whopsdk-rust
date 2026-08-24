pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveStatsResponse {
    #[serde(default)]
    pub data: RetrieveStatsResponseData,
}

impl RetrieveStatsResponse {
    pub fn builder() -> RetrieveStatsResponseBuilder {
        <RetrieveStatsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveStatsResponseBuilder {
    data: Option<RetrieveStatsResponseData>,
}

impl RetrieveStatsResponseBuilder {
    pub fn data(mut self, value: RetrieveStatsResponseData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveStatsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](RetrieveStatsResponseBuilder::data)
    pub fn build(self) -> Result<RetrieveStatsResponse, BuildError> {
        Ok(RetrieveStatsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
