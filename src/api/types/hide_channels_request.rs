pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HideChannelsRequest {}

impl HideChannelsRequest {
    pub fn builder() -> HideChannelsRequestBuilder {
        <HideChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HideChannelsRequestBuilder {}

impl HideChannelsRequestBuilder {
    /// Consumes the builder and constructs a [`HideChannelsRequest`].
    pub fn build(self) -> Result<HideChannelsRequest, BuildError> {
        Ok(HideChannelsRequest {})
    }
}
