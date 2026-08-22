pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReactionsListQueryRequest {
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
    /// The unique identifier of the message or forum post to list reactions for.
    #[serde(default)]
    pub resource_id: String,
}

impl ReactionsListQueryRequest {
    pub fn builder() -> ReactionsListQueryRequestBuilder {
        <ReactionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReactionsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    resource_id: Option<String>,
}

impl ReactionsListQueryRequestBuilder {
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

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReactionsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_id`](ReactionsListQueryRequestBuilder::resource_id)
    pub fn build(self) -> Result<ReactionsListQueryRequest, BuildError> {
        Ok(ReactionsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
        })
    }
}
