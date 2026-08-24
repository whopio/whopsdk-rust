pub use crate::prelude::*;

/// Query parameters for listIdentityProfile
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListIdentityProfileQueryRequest {
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
    /// The unique identifier of the company to filter to. When omitted, returns IPs across all ledgers the actor can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_type: Option<IdentityProfileKinds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<IdentityProfileStatuses>,
}

impl ListIdentityProfileQueryRequest {
    pub fn builder() -> ListIdentityProfileQueryRequestBuilder {
        <ListIdentityProfileQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListIdentityProfileQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    company_id: Option<String>,
    profile_type: Option<IdentityProfileKinds>,
    status: Option<IdentityProfileStatuses>,
}

impl ListIdentityProfileQueryRequestBuilder {
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

    pub fn profile_type(mut self, value: IdentityProfileKinds) -> Self {
        self.profile_type = Some(value);
        self
    }

    pub fn status(mut self, value: IdentityProfileStatuses) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListIdentityProfileQueryRequest`].
    pub fn build(self) -> Result<ListIdentityProfileQueryRequest, BuildError> {
        Ok(ListIdentityProfileQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            company_id: self.company_id,
            profile_type: self.profile_type,
            status: self.status,
        })
    }
}
