pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetNotificationsRequestPreferencesItem {
    /// What the user is notified about in this scope. `mentions` is only valid for an experience level. `null` clears the preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<SetNotificationsRequestPreferencesItemLevel>,
    /// What the preference applies to. `null` on a dimension means the preference is not narrowed there.
    #[serde(default)]
    pub scope: SetNotificationsRequestPreferencesItemScope,
}

impl SetNotificationsRequestPreferencesItem {
    pub fn builder() -> SetNotificationsRequestPreferencesItemBuilder {
        <SetNotificationsRequestPreferencesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetNotificationsRequestPreferencesItemBuilder {
    level: Option<SetNotificationsRequestPreferencesItemLevel>,
    scope: Option<SetNotificationsRequestPreferencesItemScope>,
}

impl SetNotificationsRequestPreferencesItemBuilder {
    pub fn level(mut self, value: SetNotificationsRequestPreferencesItemLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn scope(mut self, value: SetNotificationsRequestPreferencesItemScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetNotificationsRequestPreferencesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`scope`](SetNotificationsRequestPreferencesItemBuilder::scope)
    pub fn build(self) -> Result<SetNotificationsRequestPreferencesItem, BuildError> {
        Ok(SetNotificationsRequestPreferencesItem {
            level: self.level,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
