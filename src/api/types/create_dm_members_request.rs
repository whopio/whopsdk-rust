pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDmMembersRequest {
    /// The unique identifier of the DM channel to add the new member to.
    #[serde(default)]
    pub channel_id: String,
    /// The unique identifier of the user to add to the DM channel. For example, 'user_xxxxx'.
    #[serde(default)]
    pub user_id: String,
}

impl CreateDmMembersRequest {
    pub fn builder() -> CreateDmMembersRequestBuilder {
        <CreateDmMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDmMembersRequestBuilder {
    channel_id: Option<String>,
    user_id: Option<String>,
}

impl CreateDmMembersRequestBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateDmMembersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](CreateDmMembersRequestBuilder::channel_id)
    /// - [`user_id`](CreateDmMembersRequestBuilder::user_id)
    pub fn build(self) -> Result<CreateDmMembersRequest, BuildError> {
        Ok(CreateDmMembersRequest {
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
