pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SummaryPaymentsResponse {
    /// One entry per requested breakdown, keyed by the field it groups on. A field you did not ask for is absent.
    #[serde(default)]
    pub groups: SummaryPaymentsResponseGroups,
    /// How many payments match the filters.
    #[serde(default)]
    pub total: i64,
}

impl SummaryPaymentsResponse {
    pub fn builder() -> SummaryPaymentsResponseBuilder {
        <SummaryPaymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryPaymentsResponseBuilder {
    groups: Option<SummaryPaymentsResponseGroups>,
    total: Option<i64>,
}

impl SummaryPaymentsResponseBuilder {
    pub fn groups(mut self, value: SummaryPaymentsResponseGroups) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryPaymentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`groups`](SummaryPaymentsResponseBuilder::groups)
    /// - [`total`](SummaryPaymentsResponseBuilder::total)
    pub fn build(self) -> Result<SummaryPaymentsResponse, BuildError> {
        Ok(SummaryPaymentsResponse {
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
