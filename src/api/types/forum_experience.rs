pub use crate::prelude::*;

/// The parent experience that this forum belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForumExperience {
    /// The unique identifier for the experience.
    #[serde(default)]
    pub id: String,
    /// The display name of this experience shown to users in the product navigation. Maximum 255 characters.
    #[serde(default)]
    pub name: String,
}

impl ForumExperience {
    pub fn builder() -> ForumExperienceBuilder {
        <ForumExperienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForumExperienceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ForumExperienceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ForumExperience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ForumExperienceBuilder::id)
    /// - [`name`](ForumExperienceBuilder::name)
    pub fn build(self) -> Result<ForumExperience, BuildError> {
        Ok(ForumExperience {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
