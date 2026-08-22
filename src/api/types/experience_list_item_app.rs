pub use crate::prelude::*;

/// The app that powers this experience, defining its interface and behavior.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperienceListItemApp {
    /// The icon image for this app, displayed on the app store, product pages, checkout, and as the default icon for experiences using this app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ExperienceListItemAppIcon>,
    /// The unique identifier for the app.
    #[serde(default)]
    pub id: String,
    /// The display name of this app shown on the app store and in experience navigation. Maximum 30 characters.
    #[serde(default)]
    pub name: String,
}

impl ExperienceListItemApp {
    pub fn builder() -> ExperienceListItemAppBuilder {
        <ExperienceListItemAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceListItemAppBuilder {
    icon: Option<ExperienceListItemAppIcon>,
    id: Option<String>,
    name: Option<String>,
}

impl ExperienceListItemAppBuilder {
    pub fn icon(mut self, value: ExperienceListItemAppIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExperienceListItemApp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExperienceListItemAppBuilder::id)
    /// - [`name`](ExperienceListItemAppBuilder::name)
    pub fn build(self) -> Result<ExperienceListItemApp, BuildError> {
        Ok(ExperienceListItemApp {
            icon: self.icon,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
