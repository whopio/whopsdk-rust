pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateChannelsRequest {
    /// Channel icon as a file reference, for community channels and group private channels. Image files only. Group icons can be up to 5 MB. For community channels it sets the experience logo, and `null` restores the app icon. For group private channels it sets the channel's own icon, and `null` clears it. Omit to keep the current icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<UpdateChannelsRequestIcon>,
    /// Moderation settings, accepted only for community channels. Requires chat moderation permission. Omitted settings are preserved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<UpdateChannelsRequestModeration>,
    /// Replacement display name for any channel kind. Send `null` to clear a private or support channel name. Community channels require a name and also rename the experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether app notifications are on for any channel kind. Webhooks fire either way. Omit to preserve the current setting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
}

impl UpdateChannelsRequest {
    pub fn builder() -> UpdateChannelsRequestBuilder {
        <UpdateChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateChannelsRequestBuilder {
    icon: Option<UpdateChannelsRequestIcon>,
    moderation: Option<UpdateChannelsRequestModeration>,
    name: Option<String>,
    notifications_enabled: Option<bool>,
}

impl UpdateChannelsRequestBuilder {
    pub fn icon(mut self, value: UpdateChannelsRequestIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn moderation(mut self, value: UpdateChannelsRequestModeration) -> Self {
        self.moderation = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notifications_enabled(mut self, value: bool) -> Self {
        self.notifications_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateChannelsRequest`].
    pub fn build(self) -> Result<UpdateChannelsRequest, BuildError> {
        Ok(UpdateChannelsRequest {
            icon: self.icon,
            moderation: self.moderation,
            name: self.name,
            notifications_enabled: self.notifications_enabled,
        })
    }
}
