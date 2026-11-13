pub use crate::prelude::*;

/// Channel icon as a file reference, for community channels and private channels with two or more participant IDs. Image files only. Group icons can be up to 5 MB. For community channels it sets the experience logo. Used only when a channel is created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateChannelsRequestIcon {
    /// File ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
}

impl CreateChannelsRequestIcon {
    pub fn builder() -> CreateChannelsRequestIconBuilder {
        <CreateChannelsRequestIconBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateChannelsRequestIconBuilder {
    id: Option<String>,
}

impl CreateChannelsRequestIconBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateChannelsRequestIcon`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateChannelsRequestIconBuilder::id)
    pub fn build(self) -> Result<CreateChannelsRequestIcon, BuildError> {
        Ok(CreateChannelsRequestIcon {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
