pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateChannelMembersRequest {
    /// The channel to add the user to.
    #[serde(default)]
    pub channel_id: String,
    /// The user to add.
    #[serde(default)]
    pub user_id: String,
}

impl CreateChannelMembersRequest {
    pub fn builder() -> CreateChannelMembersRequestBuilder {
        <CreateChannelMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateChannelMembersRequestBuilder {
    channel_id: Option<String>,
    user_id: Option<String>,
}

impl CreateChannelMembersRequestBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateChannelMembersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](CreateChannelMembersRequestBuilder::channel_id)
    /// - [`user_id`](CreateChannelMembersRequestBuilder::user_id)
    pub fn build(self) -> Result<CreateChannelMembersRequest, BuildError> {
        Ok(CreateChannelMembersRequest {
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
