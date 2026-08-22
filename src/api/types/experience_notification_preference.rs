pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExperienceNotificationPreference {
    /// What the user is notified about in the experience: `all` notifications, only `mentions`, or `nothing`.
    pub level: ExperienceNotificationPreferenceLevel,
    /// The type of object. Always `experience_notification_preference`.
    pub object: ExperienceNotificationPreferenceObject,
    /// What the preference applies to. Echo it back to `PATCH /users/me/preferences/notifications` to change this preference.
    #[serde(default)]
    pub scope: NotificationPreferenceScope,
}

impl ExperienceNotificationPreference {
    pub fn builder() -> ExperienceNotificationPreferenceBuilder {
        <ExperienceNotificationPreferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceNotificationPreferenceBuilder {
    level: Option<ExperienceNotificationPreferenceLevel>,
    object: Option<ExperienceNotificationPreferenceObject>,
    scope: Option<NotificationPreferenceScope>,
}

impl ExperienceNotificationPreferenceBuilder {
    pub fn level(mut self, value: ExperienceNotificationPreferenceLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn object(mut self, value: ExperienceNotificationPreferenceObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn scope(mut self, value: NotificationPreferenceScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExperienceNotificationPreference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`level`](ExperienceNotificationPreferenceBuilder::level)
    /// - [`object`](ExperienceNotificationPreferenceBuilder::object)
    /// - [`scope`](ExperienceNotificationPreferenceBuilder::scope)
    pub fn build(self) -> Result<ExperienceNotificationPreference, BuildError> {
        Ok(ExperienceNotificationPreference {
            level: self
                .level
                .ok_or_else(|| BuildError::missing_field("level"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
