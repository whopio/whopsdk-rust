pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffiliatesOverridesListQueryRequest {
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
    pub override_type: Option<AffiliateOverrideRoles>,
}

impl AffiliatesOverridesListQueryRequest {
    pub fn builder() -> AffiliatesOverridesListQueryRequestBuilder {
        <AffiliatesOverridesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffiliatesOverridesListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    override_type: Option<AffiliateOverrideRoles>,
}

impl AffiliatesOverridesListQueryRequestBuilder {
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

    pub fn override_type(mut self, value: AffiliateOverrideRoles) -> Self {
        self.override_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AffiliatesOverridesListQueryRequest`].
    pub fn build(self) -> Result<AffiliatesOverridesListQueryRequest, BuildError> {
        Ok(AffiliatesOverridesListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            override_type: self.override_type,
        })
    }
}
