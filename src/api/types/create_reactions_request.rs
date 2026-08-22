pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateReactionsRequest {
    /// The emoji to react with, in shortcode or unicode format. For example, ':heart:' or a unicode emoji. Ignored in forums where reactions are always likes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// The unique identifier of a poll option to vote for. Only valid when the target message or post contains a poll.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_option_id: Option<String>,
    /// The unique identifier of the message or forum post to react to.
    #[serde(default)]
    pub resource_id: String,
}

impl CreateReactionsRequest {
    pub fn builder() -> CreateReactionsRequestBuilder {
        <CreateReactionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateReactionsRequestBuilder {
    emoji: Option<String>,
    poll_option_id: Option<String>,
    resource_id: Option<String>,
}

impl CreateReactionsRequestBuilder {
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    pub fn poll_option_id(mut self, value: impl Into<String>) -> Self {
        self.poll_option_id = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateReactionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_id`](CreateReactionsRequestBuilder::resource_id)
    pub fn build(self) -> Result<CreateReactionsRequest, BuildError> {
        Ok(CreateReactionsRequest {
            emoji: self.emoji,
            poll_option_id: self.poll_option_id,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
        })
    }
}
