pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DuplicateAdGroupsResponse {
    #[serde(default)]
    pub data: Vec<AdGroup>,
}

impl DuplicateAdGroupsResponse {
    pub fn builder() -> DuplicateAdGroupsResponseBuilder {
        <DuplicateAdGroupsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAdGroupsResponseBuilder {
    data: Option<Vec<AdGroup>>,
}

impl DuplicateAdGroupsResponseBuilder {
    pub fn data(mut self, value: Vec<AdGroup>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAdGroupsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](DuplicateAdGroupsResponseBuilder::data)
    pub fn build(self) -> Result<DuplicateAdGroupsResponse, BuildError> {
        Ok(DuplicateAdGroupsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
