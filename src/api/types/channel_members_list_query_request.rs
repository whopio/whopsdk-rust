pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChannelMembersListQueryRequest {
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
    /// The channel whose members to list.
    #[serde(default)]
    pub channel_id: String,
    /// Search by name or username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl ChannelMembersListQueryRequest {
    pub fn builder() -> ChannelMembersListQueryRequestBuilder {
        <ChannelMembersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelMembersListQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    channel_id: Option<String>,
    query: Option<String>,
}

impl ChannelMembersListQueryRequestBuilder {
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

    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChannelMembersListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](ChannelMembersListQueryRequestBuilder::channel_id)
    pub fn build(self) -> Result<ChannelMembersListQueryRequest, BuildError> {
        Ok(ChannelMembersListQueryRequest {
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            query: self.query,
        })
    }
}
