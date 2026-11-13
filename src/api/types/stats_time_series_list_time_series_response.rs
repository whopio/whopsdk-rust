pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTimeSeriesResponse {
    /// The available metrics.
    #[serde(default)]
    pub data: Vec<ListTimeSeriesResponseDataItem>,
}

impl ListTimeSeriesResponse {
    pub fn builder() -> ListTimeSeriesResponseBuilder {
        <ListTimeSeriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTimeSeriesResponseBuilder {
    data: Option<Vec<ListTimeSeriesResponseDataItem>>,
}

impl ListTimeSeriesResponseBuilder {
    pub fn data(mut self, value: Vec<ListTimeSeriesResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTimeSeriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListTimeSeriesResponseBuilder::data)
    pub fn build(self) -> Result<ListTimeSeriesResponse, BuildError> {
        Ok(ListTimeSeriesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
