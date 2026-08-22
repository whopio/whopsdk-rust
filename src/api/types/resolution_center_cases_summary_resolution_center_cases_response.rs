pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SummaryResolutionCenterCasesResponse {
    /// One entry per requested breakdown, keyed by the field it groups on. A field you did not ask for is absent.
    #[serde(default)]
    pub groups: SummaryResolutionCenterCasesResponseGroups,
    /// How many cases match the filters.
    #[serde(default)]
    pub total: i64,
}

impl SummaryResolutionCenterCasesResponse {
    pub fn builder() -> SummaryResolutionCenterCasesResponseBuilder {
        <SummaryResolutionCenterCasesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryResolutionCenterCasesResponseBuilder {
    groups: Option<SummaryResolutionCenterCasesResponseGroups>,
    total: Option<i64>,
}

impl SummaryResolutionCenterCasesResponseBuilder {
    pub fn groups(mut self, value: SummaryResolutionCenterCasesResponseGroups) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryResolutionCenterCasesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`groups`](SummaryResolutionCenterCasesResponseBuilder::groups)
    /// - [`total`](SummaryResolutionCenterCasesResponseBuilder::total)
    pub fn build(self) -> Result<SummaryResolutionCenterCasesResponse, BuildError> {
        Ok(SummaryResolutionCenterCasesResponse {
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
