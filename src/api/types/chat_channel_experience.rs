pub use crate::prelude::*;

/// The experience this chat feed is attached to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChatChannelExperience {
    /// The unique identifier for the experience.
    #[serde(default)]
    pub id: String,
    /// The display name of this experience shown to users in the product navigation. Maximum 255 characters.
    #[serde(default)]
    pub name: String,
}

impl ChatChannelExperience {
    pub fn builder() -> ChatChannelExperienceBuilder {
        <ChatChannelExperienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatChannelExperienceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ChatChannelExperienceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatChannelExperience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChatChannelExperienceBuilder::id)
    /// - [`name`](ChatChannelExperienceBuilder::name)
    pub fn build(self) -> Result<ChatChannelExperience, BuildError> {
        Ok(ChatChannelExperience {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
