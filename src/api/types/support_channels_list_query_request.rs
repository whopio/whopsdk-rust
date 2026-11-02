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
    pub fn build(self) -> Result<SupportChannelsListQueryRequest, BuildError> {
        Ok(SupportChannelsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            view: self.view,
            open: self.open,
            direction: self.direction,
            order: self.order,
            account_id: self.account_id,
        })
    }
}
