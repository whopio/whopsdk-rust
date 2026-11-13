pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReopenChannelsRequest {}

impl ReopenChannelsRequest {
    pub fn builder() -> ReopenChannelsRequestBuilder {
        <ReopenChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReopenChannelsRequestBuilder {}

impl ReopenChannelsRequestBuilder {
    /// Consumes the builder and constructs a [`ReopenChannelsRequest`].
    pub fn build(self) -> Result<ReopenChannelsRequest, BuildError> {
        Ok(ReopenChannelsRequest {})
    }
}
