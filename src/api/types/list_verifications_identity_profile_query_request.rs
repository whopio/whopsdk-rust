pub use crate::prelude::*;

/// Query parameters for listVerificationsIdentityProfile
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVerificationsIdentityProfileQueryRequest {
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
}

impl ListVerificationsIdentityProfileQueryRequest {
    pub fn builder() -> ListVerificationsIdentityProfileQueryRequestBuilder {
        <ListVerificationsIdentityProfileQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVerificationsIdentityProfileQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
}

impl ListVerificationsIdentityProfileQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListVerificationsIdentityProfileQueryRequest`].
    pub fn build(self) -> Result<ListVerificationsIdentityProfileQueryRequest, BuildError> {
        Ok(ListVerificationsIdentityProfileQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
        })
    }
}
