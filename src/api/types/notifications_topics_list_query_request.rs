pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationsTopicsListQueryRequest {
    /// Only return topics of this scope: `user` (member notifications) or `account_team` (team notifications).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_type: Option<ListTopicsRequestTopicType>,
    /// The number of topics to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns topics after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl NotificationsTopicsListQueryRequest {
    pub fn builder() -> NotificationsTopicsListQueryRequestBuilder {
        <NotificationsTopicsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationsTopicsListQueryRequestBuilder {
    topic_type: Option<ListTopicsRequestTopicType>,
    first: Option<i64>,
    after: Option<String>,
}

impl NotificationsTopicsListQueryRequestBuilder {
    pub fn topic_type(mut self, value: ListTopicsRequestTopicType) -> Self {
        self.topic_type = Some(value);
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

    /// Consumes the builder and constructs a [`NotificationsTopicsListQueryRequest`].
    pub fn build(self) -> Result<NotificationsTopicsListQueryRequest, BuildError> {
        Ok(NotificationsTopicsListQueryRequest {
            topic_type: self.topic_type,
            first: self.first,
            after: self.after,
        })
    }
}
