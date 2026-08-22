pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChatChannelsListQueryRequest {
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
    /// The unique identifier of the company to list chat channels for.
    #[serde(default)]
    pub company_id: String,
    /// The unique identifier of a product to filter by. When set, only chat channels connected to this product are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

impl ChatChannelsListQueryRequest {
    pub fn builder() -> ChatChannelsListQueryRequestBuilder {
        <ChatChannelsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatChannelsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    company_id: Option<String>,
    product_id: Option<String>,
}

impl ChatChannelsListQueryRequestBuilder {
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

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatChannelsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](ChatChannelsListQueryRequestBuilder::company_id)
    pub fn build(self) -> Result<ChatChannelsListQueryRequest, BuildError> {
        Ok(ChatChannelsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            product_id: self.product_id,
        })
    }
}
