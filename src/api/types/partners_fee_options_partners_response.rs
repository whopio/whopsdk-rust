pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeeOptionsPartnersResponse {
    pub object: FeeOptionsPartnersResponseObject,
    /// One entry per income stream a partner earns on.
    #[serde(default)]
    pub streams: Vec<FeeOptionsPartnersResponseStreamsItem>,
}

impl FeeOptionsPartnersResponse {
    pub fn builder() -> FeeOptionsPartnersResponseBuilder {
        <FeeOptionsPartnersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeeOptionsPartnersResponseBuilder {
    object: Option<FeeOptionsPartnersResponseObject>,
    streams: Option<Vec<FeeOptionsPartnersResponseStreamsItem>>,
}

impl FeeOptionsPartnersResponseBuilder {
    pub fn object(mut self, value: FeeOptionsPartnersResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn streams(mut self, value: Vec<FeeOptionsPartnersResponseStreamsItem>) -> Self {
        self.streams = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeeOptionsPartnersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](FeeOptionsPartnersResponseBuilder::object)
    /// - [`streams`](FeeOptionsPartnersResponseBuilder::streams)
    pub fn build(self) -> Result<FeeOptionsPartnersResponse, BuildError> {
        Ok(FeeOptionsPartnersResponse {
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            streams: self
                .streams
                .ok_or_else(|| BuildError::missing_field("streams"))?,
        })
    }
}
