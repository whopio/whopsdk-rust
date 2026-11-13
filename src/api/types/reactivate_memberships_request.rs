pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReactivateMembershipsRequest {
    /// Days of access from now (1-1095), which sets `current_period_end`. Omit to keep the original `current_period_end`; required once it has passed. Ignored for lifetime memberships.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}

impl ReactivateMembershipsRequest {
    pub fn builder() -> ReactivateMembershipsRequestBuilder {
        <ReactivateMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReactivateMembershipsRequestBuilder {
    days: Option<i64>,
}

impl ReactivateMembershipsRequestBuilder {
    pub fn days(mut self, value: i64) -> Self {
        self.days = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReactivateMembershipsRequest`].
    pub fn build(self) -> Result<ReactivateMembershipsRequest, BuildError> {
        Ok(ReactivateMembershipsRequest { days: self.days })
    }
}
