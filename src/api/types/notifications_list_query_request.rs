pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationsListQueryRequest {
    /// Only return notifications created since the user last viewed their source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
    /// Only return notifications from this experience (`exp_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// Only return team notifications for this account (`biz_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return notifications that mention the user directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<bool>,
    /// The number of notifications to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor (a notification `id` from a previous page); returns notifications older than it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl NotificationsListQueryRequest {
    pub fn builder() -> NotificationsListQueryRequestBuilder {
        <NotificationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationsListQueryRequestBuilder {
    unread: Option<bool>,
    experience_id: Option<String>,
    account_id: Option<String>,
    mentions: Option<bool>,
    first: Option<i64>,
    after: Option<String>,
}

impl NotificationsListQueryRequestBuilder {
    pub fn unread(mut self, value: bool) -> Self {
        self.unread = Some(value);
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn mentions(mut self, value: bool) -> Self {
        self.mentions = Some(value);
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

    /// Consumes the builder and constructs a [`NotificationsListQueryRequest`].
    pub fn build(self) -> Result<NotificationsListQueryRequest, BuildError> {
        Ok(NotificationsListQueryRequest {
            unread: self.unread,
            experience_id: self.experience_id,
            account_id: self.account_id,
            mentions: self.mentions,
            first: self.first,
            after: self.after,
        })
    }
}
