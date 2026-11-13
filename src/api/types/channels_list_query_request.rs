pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChannelsListQueryRequest {
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
    /// Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total_count: Option<bool>,
    /// Channel kinds to return. Combine `private` and `support`, or pass `community` alone. `private` includes one-to-one and group direct messages.
    #[serde(default)]
    pub kind: Vec<Option<ListChannelsRequestKindItem>>,
    /// Which support channels to return. `customer` returns support chats you opened, including when `account_id` is set. `staff` returns the support inbox of the account in `account_id`, and requires permission to read that account's support chats. Applies only to `support` channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_role: Option<ListChannelsRequestSupportRole>,
    /// Filter channels by their owning account, prefixed `biz_`. Applies to private, community and support channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Return only channels whose name contains this text. Private and support channels also match a member's username or name. `%` and `_` match literally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only support channels with this resolution. Requires `kind` to be `support` alone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListChannelsRequestStatus>,
    /// `true` returns unresolved support channels whose last message came from the customer; `false` returns the other support channels. Other channel kinds are returned either way.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awaiting_reply: Option<bool>,
    /// `true` returns channels with messages you have not read; `false` returns the rest. Requires authenticating as a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
    /// `true` returns private and support channels you have pinned; `false` returns those you have not. Requires authenticating as a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// `true` returns community channels you have opened; `false` returns those you never have. Requires authenticating as a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewed: Option<bool>,
    /// `true` returns your pending direct-message requests; `false` returns your accepted private channels. Requires `private` in `kind` and authenticating as a user. With `support` also in `kind`, `false` keeps support channels and `true` returns only pending requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
    /// The field to sort by. Defaults to `last_viewed_at` for community channels and `last_message_at` for private and support channels. Ties sort in a stable order. Cannot be supplied with updated_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListChannelsRequestOrder>,
    /// Include the current members of private and support channels, including you, with their public profiles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_members: Option<bool>,
    /// Include the most recent message you can see, with its author.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_last_message: Option<bool>,
    /// The sort direction. Defaults to `desc`. Cannot be supplied with updated_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListChannelsRequestDirection>,
    /// Include redacted entries for deleted channels and your removed memberships. Use deleted_at to remove these channels from your cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    /// Return channels where the channel or your membership changed at or after this ISO 8601 timestamp. Cannot be more than one hour old, including on subsequent pages. For older changes, reload channels without updated_after. Results use ascending synchronized_at order. Cannot be combined with order, direction, query, status, awaiting_reply, unread, pinned, viewed, requested, created_before, or created_after.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_after: Option<String>,
    /// Return only channels created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Return only channels created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
}

impl ChannelsListQueryRequest {
    pub fn builder() -> ChannelsListQueryRequestBuilder {
        <ChannelsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelsListQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    include_total_count: Option<bool>,
    kind: Option<Vec<Option<ListChannelsRequestKindItem>>>,
    support_role: Option<ListChannelsRequestSupportRole>,
    account_id: Option<String>,
    query: Option<String>,
    status: Option<ListChannelsRequestStatus>,
    awaiting_reply: Option<bool>,
    unread: Option<bool>,
    pinned: Option<bool>,
    viewed: Option<bool>,
    requested: Option<bool>,
    order: Option<ListChannelsRequestOrder>,
    include_members: Option<bool>,
    include_last_message: Option<bool>,
    direction: Option<ListChannelsRequestDirection>,
    include_deleted: Option<bool>,
    updated_after: Option<String>,
    created_before: Option<String>,
    created_after: Option<String>,
}

impl ChannelsListQueryRequestBuilder {
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

    pub fn include_total_count(mut self, value: bool) -> Self {
        self.include_total_count = Some(value);
        self
    }

    pub fn kind(mut self, value: Vec<Option<ListChannelsRequestKindItem>>) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn support_role(mut self, value: ListChannelsRequestSupportRole) -> Self {
        self.support_role = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListChannelsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn awaiting_reply(mut self, value: bool) -> Self {
        self.awaiting_reply = Some(value);
        self
    }

    pub fn unread(mut self, value: bool) -> Self {
        self.unread = Some(value);
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn viewed(mut self, value: bool) -> Self {
        self.viewed = Some(value);
        self
    }

    pub fn requested(mut self, value: bool) -> Self {
        self.requested = Some(value);
        self
    }

    pub fn order(mut self, value: ListChannelsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn include_members(mut self, value: bool) -> Self {
        self.include_members = Some(value);
        self
    }

    pub fn include_last_message(mut self, value: bool) -> Self {
        self.include_last_message = Some(value);
        self
    }

    pub fn direction(mut self, value: ListChannelsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn include_deleted(mut self, value: bool) -> Self {
        self.include_deleted = Some(value);
        self
    }

    pub fn updated_after(mut self, value: impl Into<String>) -> Self {
        self.updated_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChannelsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ChannelsListQueryRequestBuilder::kind)
    pub fn build(self) -> Result<ChannelsListQueryRequest, BuildError> {
        Ok(ChannelsListQueryRequest {
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            include_total_count: self.include_total_count,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            support_role: self.support_role,
            account_id: self.account_id,
            query: self.query,
            status: self.status,
            awaiting_reply: self.awaiting_reply,
            unread: self.unread,
            pinned: self.pinned,
            viewed: self.viewed,
            requested: self.requested,
            order: self.order,
            include_members: self.include_members,
            include_last_message: self.include_last_message,
            direction: self.direction,
            include_deleted: self.include_deleted,
            updated_after: self.updated_after,
            created_before: self.created_before,
            created_after: self.created_after,
        })
    }
}
