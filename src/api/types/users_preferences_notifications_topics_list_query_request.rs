pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsersPreferencesNotificationsTopicsListQueryRequest {
    /// Only return preferences for this delivery channel (or not narrowed to a channel).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ListTopicsRequestChannel>,
    /// Only return preferences scoped to this account's member notifications (`biz_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return preferences scoped to this account's team notifications (`biz_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_account_id: Option<String>,
    /// Only return preferences scoped to this experience (`exp_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// Only return preferences scoped to this notification topic (`topic_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    /// The number of preferences to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns preferences after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl UsersPreferencesNotificationsTopicsListQueryRequest {
    pub fn builder() -> UsersPreferencesNotificationsTopicsListQueryRequestBuilder {
        <UsersPreferencesNotificationsTopicsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsersPreferencesNotificationsTopicsListQueryRequestBuilder {
    channel: Option<ListTopicsRequestChannel>,
    account_id: Option<String>,
    team_account_id: Option<String>,
    experience_id: Option<String>,
    topic_id: Option<String>,
    first: Option<i64>,
    after: Option<String>,
}

impl UsersPreferencesNotificationsTopicsListQueryRequestBuilder {
    pub fn channel(mut self, value: ListTopicsRequestChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn team_account_id(mut self, value: impl Into<String>) -> Self {
        self.team_account_id = Some(value.into());
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn topic_id(mut self, value: impl Into<String>) -> Self {
        self.topic_id = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UsersPreferencesNotificationsTopicsListQueryRequest`].
    pub fn build(self) -> Result<UsersPreferencesNotificationsTopicsListQueryRequest, BuildError> {
        Ok(UsersPreferencesNotificationsTopicsListQueryRequest {
            channel: self.channel,
            account_id: self.account_id,
            team_account_id: self.team_account_id,
            experience_id: self.experience_id,
            topic_id: self.topic_id,
            first: self.first,
            after: self.after,
        })
    }
}
