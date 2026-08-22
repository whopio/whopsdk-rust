pub use crate::prelude::*;

/// What the preference applies to. `null` on a dimension means the preference is not narrowed there.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetNotificationsRequestPreferencesItemScope {
    /// Account to scope the preference to (member notifications), `biz_` tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Delivery channel the preference applies to. Required when setting a topic override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<SetNotificationsRequestPreferencesItemScopeChannel>,
    /// Experience to scope the preference to (`exp_` tag). Requires `account_id` when a `topic_id` is also given.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// Account whose team notifications the preference is scoped to, `biz_` tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_account_id: Option<String>,
    /// Notification topic to scope the preference to, `topic_` tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

impl SetNotificationsRequestPreferencesItemScope {
    pub fn builder() -> SetNotificationsRequestPreferencesItemScopeBuilder {
        <SetNotificationsRequestPreferencesItemScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetNotificationsRequestPreferencesItemScopeBuilder {
    account_id: Option<String>,
    channel: Option<SetNotificationsRequestPreferencesItemScopeChannel>,
    experience_id: Option<String>,
    team_account_id: Option<String>,
    topic_id: Option<String>,
}

impl SetNotificationsRequestPreferencesItemScopeBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn channel(mut self, value: SetNotificationsRequestPreferencesItemScopeChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn team_account_id(mut self, value: impl Into<String>) -> Self {
        self.team_account_id = Some(value.into());
        self
    }

    pub fn topic_id(mut self, value: impl Into<String>) -> Self {
        self.topic_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetNotificationsRequestPreferencesItemScope`].
    pub fn build(self) -> Result<SetNotificationsRequestPreferencesItemScope, BuildError> {
        Ok(SetNotificationsRequestPreferencesItemScope {
            account_id: self.account_id,
            channel: self.channel,
            experience_id: self.experience_id,
            team_account_id: self.team_account_id,
            topic_id: self.topic_id,
        })
    }
}
