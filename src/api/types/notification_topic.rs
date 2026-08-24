pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationTopic {
    /// Whether notifications for this topic are enabled by default when the user has not set a preference.
    #[serde(default)]
    pub default_preference_value: bool,
    /// Human-readable explanation of what notifications in this topic are about. `null` when no description has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Notification topic ID, prefixed `topic_`. This is the value the notification preference endpoints take as `topic_id`.
    #[serde(default)]
    pub id: String,
    /// Stable, human-readable name for the category, such as `new-follower`. Unlike `id`, it is the same in every environment, which makes it the value to match on in code and to read in logs. Treat it as an opaque string: the set is open and the casing is historical rather than normalized.
    #[serde(default)]
    pub identifier: String,
    /// Whether this topic exclusively handles mention-based notifications.
    #[serde(default)]
    pub is_mention: bool,
    /// Display name shown in notification preference settings.
    #[serde(default)]
    pub name: String,
    /// Scope of the topic: whether it applies to an account, a user, or an account's team.
    pub topic_type: NotificationTopicTopicType,
}

impl NotificationTopic {
    pub fn builder() -> NotificationTopicBuilder {
        <NotificationTopicBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationTopicBuilder {
    default_preference_value: Option<bool>,
    description: Option<String>,
    id: Option<String>,
    identifier: Option<String>,
    is_mention: Option<bool>,
    name: Option<String>,
    topic_type: Option<NotificationTopicTopicType>,
}

impl NotificationTopicBuilder {
    pub fn default_preference_value(mut self, value: bool) -> Self {
        self.default_preference_value = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn is_mention(mut self, value: bool) -> Self {
        self.is_mention = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn topic_type(mut self, value: NotificationTopicTopicType) -> Self {
        self.topic_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotificationTopic`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default_preference_value`](NotificationTopicBuilder::default_preference_value)
    /// - [`id`](NotificationTopicBuilder::id)
    /// - [`identifier`](NotificationTopicBuilder::identifier)
    /// - [`is_mention`](NotificationTopicBuilder::is_mention)
    /// - [`name`](NotificationTopicBuilder::name)
    /// - [`topic_type`](NotificationTopicBuilder::topic_type)
    pub fn build(self) -> Result<NotificationTopic, BuildError> {
        Ok(NotificationTopic {
            default_preference_value: self
                .default_preference_value
                .ok_or_else(|| BuildError::missing_field("default_preference_value"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            identifier: self
                .identifier
                .ok_or_else(|| BuildError::missing_field("identifier"))?,
            is_mention: self
                .is_mention
                .ok_or_else(|| BuildError::missing_field("is_mention"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            topic_type: self
                .topic_type
                .ok_or_else(|| BuildError::missing_field("topic_type"))?,
        })
    }
}
