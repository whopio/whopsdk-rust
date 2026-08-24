pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListStatsResponse {
    /// The available metrics.
    #[serde(default)]
    pub data: Vec<ListStatsResponseDataItem>,
}

impl ListStatsResponse {
    pub fn builder() -> ListStatsResponseBuilder {
        <ListStatsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStatsResponseBuilder {
    data: Option<Vec<ListStatsResponseDataItem>>,
}

impl ListStatsResponseBuilder {
    pub fn data(mut self, value: Vec<ListStatsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListStatsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListStatsResponseBuilder::data)
    pub fn build(self) -> Result<ListStatsResponse, BuildError> {
        Ok(ListStatsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
