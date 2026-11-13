pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateChannelMembersRequest {
    pub notification_preference: UpdateChannelMembersRequestNotificationPreference,
}

impl UpdateChannelMembersRequest {
    pub fn builder() -> UpdateChannelMembersRequestBuilder {
        <UpdateChannelMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateChannelMembersRequestBuilder {
    notification_preference: Option<UpdateChannelMembersRequestNotificationPreference>,
}

impl UpdateChannelMembersRequestBuilder {
    pub fn notification_preference(
        mut self,
        value: UpdateChannelMembersRequestNotificationPreference,
    ) -> Self {
        self.notification_preference = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateChannelMembersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`notification_preference`](UpdateChannelMembersRequestBuilder::notification_preference)
    pub fn build(self) -> Result<UpdateChannelMembersRequest, BuildError> {
        Ok(UpdateChannelMembersRequest {
            notification_preference: self
                .notification_preference
                .ok_or_else(|| BuildError::missing_field("notification_preference"))?,
        })
    }
}
