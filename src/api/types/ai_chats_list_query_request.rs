pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AiChatsListQueryRequest {
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
    /// When true, returns only chats with an active cron schedule
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_active_crons: Option<bool>,
}

impl AiChatsListQueryRequest {
    pub fn builder() -> AiChatsListQueryRequestBuilder {
        <AiChatsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AiChatsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    only_active_crons: Option<bool>,
}

impl AiChatsListQueryRequestBuilder {
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

    pub fn only_active_crons(mut self, value: bool) -> Self {
        self.only_active_crons = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AiChatsListQueryRequest`].
    pub fn build(self) -> Result<AiChatsListQueryRequest, BuildError> {
        Ok(AiChatsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            only_active_crons: self.only_active_crons,
        })
    }
}
