pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserNotificationPreference {
    /// When the preference was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Notification preference ID, prefixed `unpf_`.
    #[serde(default)]
    pub id: String,
    /// What the user is notified about in this scope: `all` or `nothing`.
    pub level: UserNotificationPreferenceLevel,
    /// The type of object. Always `notification_preference`.
    pub object: UserNotificationPreferenceObject,
    /// What the preference applies to. Echo it back to `PATCH /users/me/preferences/notifications` to change this preference.
    #[serde(default)]
    pub scope: NotificationPreferenceScope,
    /// When the preference was last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl UserNotificationPreference {
    pub fn builder() -> UserNotificationPreferenceBuilder {
        <UserNotificationPreferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserNotificationPreferenceBuilder {
    created_at: Option<String>,
    id: Option<String>,
    level: Option<UserNotificationPreferenceLevel>,
    object: Option<UserNotificationPreferenceObject>,
    scope: Option<NotificationPreferenceScope>,
    updated_at: Option<String>,
}

impl UserNotificationPreferenceBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn level(mut self, value: UserNotificationPreferenceLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn object(mut self, value: UserNotificationPreferenceObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn scope(mut self, value: NotificationPreferenceScope) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserNotificationPreference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](UserNotificationPreferenceBuilder::created_at)
    /// - [`id`](UserNotificationPreferenceBuilder::id)
    /// - [`level`](UserNotificationPreferenceBuilder::level)
    /// - [`object`](UserNotificationPreferenceBuilder::object)
    /// - [`scope`](UserNotificationPreferenceBuilder::scope)
    /// - [`updated_at`](UserNotificationPreferenceBuilder::updated_at)
    pub fn build(self) -> Result<UserNotificationPreference, BuildError> {
        Ok(UserNotificationPreference {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            level: self
                .level
                .ok_or_else(|| BuildError::missing_field("level"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
