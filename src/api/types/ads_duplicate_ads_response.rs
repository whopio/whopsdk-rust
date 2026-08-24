pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DuplicateAdsResponse {
    #[serde(default)]
    pub data: Vec<Ad>,
}

impl DuplicateAdsResponse {
    pub fn builder() -> DuplicateAdsResponseBuilder {
        <DuplicateAdsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DuplicateAdsResponseBuilder {
    data: Option<Vec<Ad>>,
}

impl DuplicateAdsResponseBuilder {
    pub fn data(mut self, value: Vec<Ad>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DuplicateAdsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](DuplicateAdsResponseBuilder::data)
    pub fn build(self) -> Result<DuplicateAdsResponse, BuildError> {
        Ok(DuplicateAdsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
