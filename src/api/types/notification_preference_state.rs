pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationPreferenceState {
    /// What the user is now notified about in this scope, or `null` if the preference was cleared and the scope inherits its default again.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<NotificationPreferenceStateLevel>,
    /// Which kind of preference was written: `experience_notification_preference` for an experience level, `notification_preference` for a topic override.
    pub object: NotificationPreferenceStateObject,
    /// The scope that was written, resolved. `null` on a dimension means the preference is not narrowed there.
    #[serde(default)]
    pub scope: NotificationPreferenceScope,
}

impl NotificationPreferenceState {
    pub fn builder() -> NotificationPreferenceStateBuilder {
        <NotificationPreferenceStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationPreferenceStateBuilder {
    level: Option<NotificationPreferenceStateLevel>,
    object: Option<NotificationPreferenceStateObject>,
    scope: Option<NotificationPreferenceScope>,
}

impl NotificationPreferenceStateBuilder {
    pub fn level(mut self, value: NotificationPreferenceStateLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn object(mut self, value: NotificationPreferenceStateObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn scope(mut self, value: NotificationPreferenceScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotificationPreferenceState`].
    /// This method will fail if any of the following fields are not set:
    /// - [`object`](NotificationPreferenceStateBuilder::object)
    /// - [`scope`](NotificationPreferenceStateBuilder::scope)
    pub fn build(self) -> Result<NotificationPreferenceState, BuildError> {
        Ok(NotificationPreferenceState {
            level: self.level,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
