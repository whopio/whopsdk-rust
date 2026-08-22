pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct CreateAudiencesResponseData {
    pub data: Vec<Audience>,
}

impl CreateAudiencesResponseData {
    pub fn builder() -> CreateAudiencesResponseDataBuilder {
        <CreateAudiencesResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudiencesResponseDataBuilder {
    data: Option<Vec<Audience>>,
}

impl CreateAudiencesResponseDataBuilder {
    pub fn data(mut self, value: Vec<Audience>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAudiencesResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](CreateAudiencesResponseDataBuilder::data)
    pub fn build(self) -> Result<CreateAudiencesResponseData, BuildError> {
        Ok(CreateAudiencesResponseData {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
