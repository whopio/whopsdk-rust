pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PauseMembershipsRequest {
    /// ISO 8601 time to automatically resume payment collection. Must be in the future; only supported for memberships billed by Whop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

impl PauseMembershipsRequest {
    pub fn builder() -> PauseMembershipsRequestBuilder {
        <PauseMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PauseMembershipsRequestBuilder {
    until: Option<String>,
}

impl PauseMembershipsRequestBuilder {
    pub fn until(mut self, value: impl Into<String>) -> Self {
        self.until = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PauseMembershipsRequest`].
    pub fn build(self) -> Result<PauseMembershipsRequest, BuildError> {
        Ok(PauseMembershipsRequest { until: self.until })
    }
}
