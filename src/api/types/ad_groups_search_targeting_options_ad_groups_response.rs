pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchTargetingOptionsAdGroupsResponse {
    #[serde(default)]
    pub data: Vec<TargetingOption>,
}

impl SearchTargetingOptionsAdGroupsResponse {
    pub fn builder() -> SearchTargetingOptionsAdGroupsResponseBuilder {
        <SearchTargetingOptionsAdGroupsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchTargetingOptionsAdGroupsResponseBuilder {
    data: Option<Vec<TargetingOption>>,
}

impl SearchTargetingOptionsAdGroupsResponseBuilder {
    pub fn data(mut self, value: Vec<TargetingOption>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchTargetingOptionsAdGroupsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](SearchTargetingOptionsAdGroupsResponseBuilder::data)
    pub fn build(self) -> Result<SearchTargetingOptionsAdGroupsResponse, BuildError> {
        Ok(SearchTargetingOptionsAdGroupsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
