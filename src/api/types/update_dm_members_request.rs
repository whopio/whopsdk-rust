pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDmMembersRequest {
    /// The notification setting for this member, controlling how they receive alerts for new messages in this channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_preference: Option<DmsFeedMemberNotificationPreferences>,
    /// The membership status for this member in the DM channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DmsFeedMemberStatuses>,
}

impl UpdateDmMembersRequest {
    pub fn builder() -> UpdateDmMembersRequestBuilder {
        <UpdateDmMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDmMembersRequestBuilder {
    notification_preference: Option<DmsFeedMemberNotificationPreferences>,
    status: Option<DmsFeedMemberStatuses>,
}

impl UpdateDmMembersRequestBuilder {
    pub fn notification_preference(mut self, value: DmsFeedMemberNotificationPreferences) -> Self {
        self.notification_preference = Some(value);
        self
    }

    pub fn status(mut self, value: DmsFeedMemberStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDmMembersRequest`].
    pub fn build(self) -> Result<UpdateDmMembersRequest, BuildError> {
        Ok(UpdateDmMembersRequest {
            notification_preference: self.notification_preference,
            status: self.status,
        })
    }
}
