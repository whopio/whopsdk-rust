pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UnreactMessagesRequest {
    /// The emoji to remove, as a Unicode emoji or a supported shortcode.
    #[serde(default)]
    pub emoji: String,
}

impl UnreactMessagesRequest {
    pub fn builder() -> UnreactMessagesRequestBuilder {
        <UnreactMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnreactMessagesRequestBuilder {
    emoji: Option<String>,
}

impl UnreactMessagesRequestBuilder {
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnreactMessagesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`emoji`](UnreactMessagesRequestBuilder::emoji)
    pub fn build(self) -> Result<UnreactMessagesRequest, BuildError> {
        Ok(UnreactMessagesRequest {
            emoji: self
                .emoji
                .ok_or_else(|| BuildError::missing_field("emoji"))?,
        })
    }
}
