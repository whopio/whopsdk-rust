pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AcceptChannelsRequest {}

impl AcceptChannelsRequest {
    pub fn builder() -> AcceptChannelsRequestBuilder {
        <AcceptChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcceptChannelsRequestBuilder {}

impl AcceptChannelsRequestBuilder {
    /// Consumes the builder and constructs a [`AcceptChannelsRequest`].
    pub fn build(self) -> Result<AcceptChannelsRequest, BuildError> {
        Ok(AcceptChannelsRequest {})
    }
}
