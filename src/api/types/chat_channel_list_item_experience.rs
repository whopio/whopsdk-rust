pub use crate::prelude::*;

/// The experience this chat feed is attached to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChatChannelListItemExperience {
    /// The unique identifier for the experience.
    #[serde(default)]
    pub id: String,
    /// The display name of this experience shown to users in the product navigation. Maximum 255 characters.
    #[serde(default)]
    pub name: String,
}

impl ChatChannelListItemExperience {
    pub fn builder() -> ChatChannelListItemExperienceBuilder {
        <ChatChannelListItemExperienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatChannelListItemExperienceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ChatChannelListItemExperienceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatChannelListItemExperience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChatChannelListItemExperienceBuilder::id)
    /// - [`name`](ChatChannelListItemExperienceBuilder::name)
    pub fn build(self) -> Result<ChatChannelListItemExperience, BuildError> {
        Ok(ChatChannelListItemExperience {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
