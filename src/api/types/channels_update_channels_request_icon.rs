pub use crate::prelude::*;

/// Channel icon as a file reference, for community channels and group private channels. Image files only. Group icons can be up to 5 MB. For community channels it sets the experience logo, and `null` restores the app icon. For group private channels it sets the channel's own icon, and `null` clears it. Omit to keep the current icon.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateChannelsRequestIcon {
    /// File ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
}

impl UpdateChannelsRequestIcon {
    pub fn builder() -> UpdateChannelsRequestIconBuilder {
        <UpdateChannelsRequestIconBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateChannelsRequestIconBuilder {
    id: Option<String>,
}

impl UpdateChannelsRequestIconBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateChannelsRequestIcon`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateChannelsRequestIconBuilder::id)
    pub fn build(self) -> Result<UpdateChannelsRequestIcon, BuildError> {
        Ok(UpdateChannelsRequestIcon {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
