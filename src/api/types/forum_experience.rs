pub use crate::prelude::*;

/// The parent experience that this forum belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForumExperience {
    /// The unique identifier for the experience.
    #[serde(default)]
    pub id: String,
    /// Whether this experience is publicly visible to all users, including those without a membership.
    #[serde(default)]
    pub is_public: bool,
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
    is_public: Option<bool>,
    name: Option<String>,
}

impl ForumExperienceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_public(mut self, value: bool) -> Self {
        self.is_public = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ForumExperience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ForumExperienceBuilder::id)
    /// - [`is_public`](ForumExperienceBuilder::is_public)
    /// - [`name`](ForumExperienceBuilder::name)
    pub fn build(self) -> Result<ForumExperience, BuildError> {
        Ok(ForumExperience {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_public: self
                .is_public
                .ok_or_else(|| BuildError::missing_field("is_public"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
