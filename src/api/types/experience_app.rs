pub use crate::prelude::*;

/// The app that powers this experience, defining its interface and behavior.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperienceApp {
    /// The icon image for this app, displayed on the app store, product pages, checkout, and as the default icon for experiences using this app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ExperienceAppIcon>,
    /// The unique identifier for the app.
    #[serde(default)]
    pub id: String,
    /// The display name of this app shown on the app store and in experience navigation. Maximum 30 characters.
    #[serde(default)]
    pub name: String,
}

impl ExperienceApp {
    pub fn builder() -> ExperienceAppBuilder {
        <ExperienceAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceAppBuilder {
    icon: Option<ExperienceAppIcon>,
    id: Option<String>,
    name: Option<String>,
}

impl ExperienceAppBuilder {
    pub fn icon(mut self, value: ExperienceAppIcon) -> Self {
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

    /// Consumes the builder and constructs a [`ExperienceApp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExperienceAppBuilder::id)
    /// - [`name`](ExperienceAppBuilder::name)
    pub fn build(self) -> Result<ExperienceApp, BuildError> {
        Ok(ExperienceApp {
            icon: self.icon,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
