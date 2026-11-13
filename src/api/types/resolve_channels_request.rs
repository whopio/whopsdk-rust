pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolveChannelsRequest {}

impl ResolveChannelsRequest {
    pub fn builder() -> ResolveChannelsRequestBuilder {
        <ResolveChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolveChannelsRequestBuilder {}

impl ResolveChannelsRequestBuilder {
    /// Consumes the builder and constructs a [`ResolveChannelsRequest`].
    pub fn build(self) -> Result<ResolveChannelsRequest, BuildError> {
        Ok(ResolveChannelsRequest {})
    }
}
