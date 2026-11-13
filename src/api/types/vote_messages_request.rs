pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoteMessagesRequest {
    /// The option ID from the poll, not its display text.
    #[serde(default)]
    pub poll_option_id: String,
}

impl VoteMessagesRequest {
    pub fn builder() -> VoteMessagesRequestBuilder {
        <VoteMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoteMessagesRequestBuilder {
    poll_option_id: Option<String>,
}

impl VoteMessagesRequestBuilder {
    pub fn poll_option_id(mut self, value: impl Into<String>) -> Self {
        self.poll_option_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoteMessagesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`poll_option_id`](VoteMessagesRequestBuilder::poll_option_id)
    pub fn build(self) -> Result<VoteMessagesRequest, BuildError> {
        Ok(VoteMessagesRequest {
            poll_option_id: self
                .poll_option_id
                .ok_or_else(|| BuildError::missing_field("poll_option_id"))?,
        })
    }
}
