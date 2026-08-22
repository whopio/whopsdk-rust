pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationExperience {
    /// App the experience belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<NotificationExperienceApp>,
    /// Experience ID, prefixed `exp_`.
    #[serde(default)]
    pub id: String,
    /// Display name of the experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl NotificationExperience {
    pub fn builder() -> NotificationExperienceBuilder {
        <NotificationExperienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationExperienceBuilder {
    app: Option<NotificationExperienceApp>,
    id: Option<String>,
    name: Option<String>,
}

impl NotificationExperienceBuilder {
    pub fn app(mut self, value: NotificationExperienceApp) -> Self {
        self.app = Some(value);
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

    /// Consumes the builder and constructs a [`NotificationExperience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](NotificationExperienceBuilder::id)
    pub fn build(self) -> Result<NotificationExperience, BuildError> {
        Ok(NotificationExperience {
            app: self.app,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
        })
    }
}
