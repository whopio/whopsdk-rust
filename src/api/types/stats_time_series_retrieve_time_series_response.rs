pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrieveTimeSeriesResponse {
    #[serde(default)]
    pub data: RetrieveTimeSeriesResponseData,
}

impl RetrieveTimeSeriesResponse {
    pub fn builder() -> RetrieveTimeSeriesResponseBuilder {
        <RetrieveTimeSeriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveTimeSeriesResponseBuilder {
    data: Option<RetrieveTimeSeriesResponseData>,
}

impl RetrieveTimeSeriesResponseBuilder {
    pub fn data(mut self, value: RetrieveTimeSeriesResponseData) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveTimeSeriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](RetrieveTimeSeriesResponseBuilder::data)
    pub fn build(self) -> Result<RetrieveTimeSeriesResponse, BuildError> {
        Ok(RetrieveTimeSeriesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
