pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SummaryDisputesResponse {
    /// One entry per requested breakdown, keyed by the field it groups on. A field you did not ask for is absent.
    #[serde(default)]
    pub groups: SummaryDisputesResponseGroups,
    /// How many disputes match the filters.
    #[serde(default)]
    pub total: i64,
}

impl SummaryDisputesResponse {
    pub fn builder() -> SummaryDisputesResponseBuilder {
        <SummaryDisputesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryDisputesResponseBuilder {
    groups: Option<SummaryDisputesResponseGroups>,
    total: Option<i64>,
}

impl SummaryDisputesResponseBuilder {
    pub fn groups(mut self, value: SummaryDisputesResponseGroups) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryDisputesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`groups`](SummaryDisputesResponseBuilder::groups)
    /// - [`total`](SummaryDisputesResponseBuilder::total)
    pub fn build(self) -> Result<SummaryDisputesResponse, BuildError> {
        Ok(SummaryDisputesResponse {
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
