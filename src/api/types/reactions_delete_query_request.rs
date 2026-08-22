pub use crate::prelude::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReactionsDeleteQueryRequest {
    /// The emoji to remove, in shortcode or unicode format. For example, ':heart:' or a unicode emoji. Required when the id refers to a message or post instead of a reaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}

impl ReactionsDeleteQueryRequest {
    pub fn builder() -> ReactionsDeleteQueryRequestBuilder {
        <ReactionsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReactionsDeleteQueryRequestBuilder {
    emoji: Option<String>,
}

impl ReactionsDeleteQueryRequestBuilder {
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReactionsDeleteQueryRequest`].
    pub fn build(self) -> Result<ReactionsDeleteQueryRequest, BuildError> {
        Ok(ReactionsDeleteQueryRequest { emoji: self.emoji })
    }
}
