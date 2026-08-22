pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationTopicSummary {
    /// Whether notifications for this topic are enabled by default when the user has not set a preference.
    #[serde(default)]
    pub default_preference_value: bool,
    /// Notification topic ID, prefixed `topic_`.
    #[serde(default)]
    pub id: String,
    /// Whether this topic exclusively handles mention-based notifications.
    #[serde(default)]
    pub is_mention: bool,
    /// Scope of the topic: whether it applies to an account, a user, or an account's team.
    pub topic_type: NotificationTopicSummaryTopicType,
}

impl NotificationTopicSummary {
    pub fn builder() -> NotificationTopicSummaryBuilder {
        <NotificationTopicSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationTopicSummaryBuilder {
    default_preference_value: Option<bool>,
    id: Option<String>,
    is_mention: Option<bool>,
    topic_type: Option<NotificationTopicSummaryTopicType>,
}

impl NotificationTopicSummaryBuilder {
    pub fn default_preference_value(mut self, value: bool) -> Self {
        self.default_preference_value = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_mention(mut self, value: bool) -> Self {
        self.is_mention = Some(value);
        self
    }

    pub fn topic_type(mut self, value: NotificationTopicSummaryTopicType) -> Self {
        self.topic_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotificationTopicSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default_preference_value`](NotificationTopicSummaryBuilder::default_preference_value)
    /// - [`id`](NotificationTopicSummaryBuilder::id)
    /// - [`is_mention`](NotificationTopicSummaryBuilder::is_mention)
    /// - [`topic_type`](NotificationTopicSummaryBuilder::topic_type)
    pub fn build(self) -> Result<NotificationTopicSummary, BuildError> {
        Ok(NotificationTopicSummary {
            default_preference_value: self
                .default_preference_value
                .ok_or_else(|| BuildError::missing_field("default_preference_value"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_mention: self
                .is_mention
                .ok_or_else(|| BuildError::missing_field("is_mention"))?,
            topic_type: self
                .topic_type
                .ok_or_else(|| BuildError::missing_field("topic_type"))?,
        })
    }
}
