pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationPreferenceScope {
    /// Account the preference is scoped to (member notifications), prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Delivery channel the preference applies to. `null` applies to every channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<NotificationPreferenceScopeChannel>,
    /// Experience the preference is scoped to, prefixed `exp_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// Account whose team notifications the preference is scoped to, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_account_id: Option<String>,
    /// Notification topic the preference is scoped to, prefixed `topic_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

impl NotificationPreferenceScope {
    pub fn builder() -> NotificationPreferenceScopeBuilder {
        <NotificationPreferenceScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationPreferenceScopeBuilder {
    account_id: Option<String>,
    channel: Option<NotificationPreferenceScopeChannel>,
    experience_id: Option<String>,
    team_account_id: Option<String>,
    topic_id: Option<String>,
}

impl NotificationPreferenceScopeBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn channel(mut self, value: NotificationPreferenceScopeChannel) -> Self {
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

    /// Consumes the builder and constructs a [`NotificationPreferenceScope`].
    pub fn build(self) -> Result<NotificationPreferenceScope, BuildError> {
        Ok(NotificationPreferenceScope {
            account_id: self.account_id,
            channel: self.channel,
            experience_id: self.experience_id,
            team_account_id: self.team_account_id,
            topic_id: self.topic_id,
        })
    }
}
