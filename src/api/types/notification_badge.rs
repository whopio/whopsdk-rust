pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationBadge {
    /// Account the experience belongs to, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Experience the badge counts, prefixed `exp_`.
    #[serde(default)]
    pub experience_id: String,
    /// Whether the caller has unread notifications in this experience.
    #[serde(default)]
    pub has_unread: bool,
    /// Number of unread important (mention) notifications in this experience.
    #[serde(default)]
    pub important_count: i64,
    /// When the caller last viewed the experience, as an ISO 8601 timestamp. `null` when never viewed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_viewed_at: Option<String>,
}

impl NotificationBadge {
    pub fn builder() -> NotificationBadgeBuilder {
        <NotificationBadgeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationBadgeBuilder {
    account_id: Option<String>,
    experience_id: Option<String>,
    has_unread: Option<bool>,
    important_count: Option<i64>,
    last_viewed_at: Option<String>,
}

impl NotificationBadgeBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn has_unread(mut self, value: bool) -> Self {
        self.has_unread = Some(value);
        self
    }

    pub fn important_count(mut self, value: i64) -> Self {
        self.important_count = Some(value);
        self
    }

    pub fn last_viewed_at(mut self, value: impl Into<String>) -> Self {
        self.last_viewed_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotificationBadge`].
    /// This method will fail if any of the following fields are not set:
    /// - [`experience_id`](NotificationBadgeBuilder::experience_id)
    /// - [`has_unread`](NotificationBadgeBuilder::has_unread)
    /// - [`important_count`](NotificationBadgeBuilder::important_count)
    pub fn build(self) -> Result<NotificationBadge, BuildError> {
        Ok(NotificationBadge {
            account_id: self.account_id,
            experience_id: self
                .experience_id
                .ok_or_else(|| BuildError::missing_field("experience_id"))?,
            has_unread: self
                .has_unread
                .ok_or_else(|| BuildError::missing_field("has_unread"))?,
            important_count: self
                .important_count
                .ok_or_else(|| BuildError::missing_field("important_count"))?,
            last_viewed_at: self.last_viewed_at,
        })
    }
}
