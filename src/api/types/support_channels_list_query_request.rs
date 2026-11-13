pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelsListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view: Option<SupportChannelView>,
    /// Whether to filter by open or resolved support channels. Set to true to only return channels awaiting a response, or false for resolved channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open: Option<bool>,
    /// Only return support channels opened by this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_id: Option<String>,
    /// Only return support channels opened by any of these customers. Maximum 100 IDs per request.
    #[serde(default)]
    pub customer_user_ids: Vec<Option<String>>,
    /// Set to true to only return channels the authenticated user has joined as a participant. Requires a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participated: Option<bool>,
    /// Set to true to return `last_message` and `unread` on each channel. Both are `null` otherwise, which keeps large lists fast.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_last_message: Option<bool>,
    /// Set to true to return `participated` on each channel. It is `null` otherwise, which keeps large lists fast.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_participated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<MessageChannelOrder>,
    /// The unique identifier of the company to list support channels for. Includes channels of child companies. When omitted, returns support channels across all companies the user has access to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl SupportChannelsListQueryRequest {
    pub fn builder() -> SupportChannelsListQueryRequestBuilder {
        <SupportChannelsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    view: Option<SupportChannelView>,
    open: Option<bool>,
    customer_user_id: Option<String>,
    customer_user_ids: Option<Vec<Option<String>>>,
    participated: Option<bool>,
    include_last_message: Option<bool>,
    include_participated: Option<bool>,
    direction: Option<Direction>,
    order: Option<MessageChannelOrder>,
    account_id: Option<String>,
}

impl SupportChannelsListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn view(mut self, value: SupportChannelView) -> Self {
        self.view = Some(value);
        self
    }

    pub fn open(mut self, value: bool) -> Self {
        self.open = Some(value);
        self
    }

    pub fn customer_user_id(mut self, value: impl Into<String>) -> Self {
        self.customer_user_id = Some(value.into());
        self
    }

    pub fn customer_user_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.customer_user_ids = Some(value);
        self
    }

    pub fn participated(mut self, value: bool) -> Self {
        self.participated = Some(value);
        self
    }

    pub fn include_last_message(mut self, value: bool) -> Self {
        self.include_last_message = Some(value);
        self
    }

    pub fn include_participated(mut self, value: bool) -> Self {
        self.include_participated = Some(value);
        self
    }

    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn order(mut self, value: MessageChannelOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_user_ids`](SupportChannelsListQueryRequestBuilder::customer_user_ids)
    pub fn build(self) -> Result<SupportChannelsListQueryRequest, BuildError> {
        Ok(SupportChannelsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            view: self.view,
            open: self.open,
            customer_user_id: self.customer_user_id,
            customer_user_ids: self
                .customer_user_ids
                .ok_or_else(|| BuildError::missing_field("customer_user_ids"))?,
            participated: self.participated,
            include_last_message: self.include_last_message,
            include_participated: self.include_participated,
            direction: self.direction,
            order: self.order,
            account_id: self.account_id,
        })
    }
}
