pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationExperienceApp {
    /// Icon image URL. Always present — the default app icon when none is uploaded.
    #[serde(default)]
    pub icon_url: String,
    /// App ID, prefixed `app_`.
    #[serde(default)]
    pub id: String,
}

impl NotificationExperienceApp {
    pub fn builder() -> NotificationExperienceAppBuilder {
        <NotificationExperienceAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationExperienceAppBuilder {
    icon_url: Option<String>,
    id: Option<String>,
}

impl NotificationExperienceAppBuilder {
    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotificationExperienceApp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`icon_url`](NotificationExperienceAppBuilder::icon_url)
    /// - [`id`](NotificationExperienceAppBuilder::id)
    pub fn build(self) -> Result<NotificationExperienceApp, BuildError> {
        Ok(NotificationExperienceApp {
            icon_url: self
                .icon_url
                .ok_or_else(|| BuildError::missing_field("icon_url"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
