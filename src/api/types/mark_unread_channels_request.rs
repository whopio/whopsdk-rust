pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkUnreadChannelsRequest {}

impl MarkUnreadChannelsRequest {
    pub fn builder() -> MarkUnreadChannelsRequestBuilder {
        <MarkUnreadChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkUnreadChannelsRequestBuilder {}

impl MarkUnreadChannelsRequestBuilder {
    /// Consumes the builder and constructs a [`MarkUnreadChannelsRequest`].
    pub fn build(self) -> Result<MarkUnreadChannelsRequest, BuildError> {
        Ok(MarkUnreadChannelsRequest {})
    }
}
