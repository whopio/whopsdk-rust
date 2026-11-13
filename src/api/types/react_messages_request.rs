pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReactMessagesRequest {
    /// A single Unicode emoji or a supported shortcode such as `:heart:`.
    #[serde(default)]
    pub emoji: String,
}

impl ReactMessagesRequest {
    pub fn builder() -> ReactMessagesRequestBuilder {
        <ReactMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReactMessagesRequestBuilder {
    emoji: Option<String>,
}

impl ReactMessagesRequestBuilder {
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReactMessagesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`emoji`](ReactMessagesRequestBuilder::emoji)
    pub fn build(self) -> Result<ReactMessagesRequest, BuildError> {
        Ok(ReactMessagesRequest {
            emoji: self
                .emoji
                .ok_or_else(|| BuildError::missing_field("emoji"))?,
        })
    }
}
