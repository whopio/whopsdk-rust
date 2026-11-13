pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessagesListQueryRequest {
    /// Filter by public channel ID. An experience ID is also accepted for experience chat.
    #[serde(default)]
    pub channel_id: String,
    /// Return replies to this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replying_to_message_id: Option<String>,
    /// Filter by message pin state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// Include redacted deletion entries for cache synchronization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    /// Return messages created strictly after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Return messages created strictly before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Return messages changed at or after this ISO 8601 timestamp in ascending change order. Cannot be combined with history filters, around anchors, order, or direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<String>,
    /// Filter messages strictly before this message in channel_id by creation time and message ID. Deleted anchors are accepted; missing or other-channel IDs return 404. Cannot be combined with around anchors or updated_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_message_id: Option<String>,
    /// Filter messages strictly after this message in channel_id by creation time and message ID. Deleted anchors are accepted; missing or other-channel IDs return 404. Cannot be combined with around anchors or updated_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_message_id: Option<String>,
    /// Center the first page around this live message in the selected channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub around_message_id: Option<String>,
    /// Center the first page around this ISO 8601 timestamp in the selected channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub around_timestamp: Option<String>,
    /// History sort field. Defaults to created_at. Cannot be supplied with updated_after or around anchors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListMessagesRequestOrder>,
    /// History sort direction. Defaults to desc. Cannot be supplied with updated_after or around anchors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListMessagesRequestDirection>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl MessagesListQueryRequest {
    pub fn builder() -> MessagesListQueryRequestBuilder {
        <MessagesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessagesListQueryRequestBuilder {
    channel_id: Option<String>,
    replying_to_message_id: Option<String>,
    pinned: Option<bool>,
    include_deleted: Option<bool>,
    created_after: Option<String>,
    created_before: Option<String>,
    updated_after: Option<String>,
    before_message_id: Option<String>,
    after_message_id: Option<String>,
    around_message_id: Option<String>,
    around_timestamp: Option<String>,
    order: Option<ListMessagesRequestOrder>,
    direction: Option<ListMessagesRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl MessagesListQueryRequestBuilder {
    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn replying_to_message_id(mut self, value: impl Into<String>) -> Self {
        self.replying_to_message_id = Some(value.into());
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn include_deleted(mut self, value: bool) -> Self {
        self.include_deleted = Some(value);
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    pub fn before_message_id(mut self, value: impl Into<String>) -> Self {
        self.before_message_id = Some(value.into());
        self
    }

    pub fn after_message_id(mut self, value: impl Into<String>) -> Self {
        self.after_message_id = Some(value.into());
        self
    }

    pub fn around_message_id(mut self, value: impl Into<String>) -> Self {
        self.around_message_id = Some(value.into());
        self
    }

    pub fn around_timestamp(mut self, value: impl Into<String>) -> Self {
        self.around_timestamp = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListMessagesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListMessagesRequestDirection) -> Self {
        self.direction = Some(value);
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

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessagesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](MessagesListQueryRequestBuilder::channel_id)
    pub fn build(self) -> Result<MessagesListQueryRequest, BuildError> {
        Ok(MessagesListQueryRequest {
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            replying_to_message_id: self.replying_to_message_id,
            pinned: self.pinned,
            include_deleted: self.include_deleted,
            created_after: self.created_after,
            created_before: self.created_before,
            updated_after: self.updated_after,
            before_message_id: self.before_message_id,
            after_message_id: self.after_message_id,
            around_message_id: self.around_message_id,
            around_timestamp: self.around_timestamp,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
