pub use crate::prelude::*;

/// One entry per requested breakdown, keyed by the field it groups on. A field you did not ask for is absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SummaryDisputesResponseGroups {
    /// How many of the matching disputes are in each currency, keyed by three-letter ISO code. Only currencies with at least one dispute are present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<HashMap<String, i64>>,
    /// How many of the matching disputes are in each status. Every status is present, including those with a count of zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SummaryDisputesResponseGroupsStatus>,
}

impl SummaryDisputesResponseGroups {
    pub fn builder() -> SummaryDisputesResponseGroupsBuilder {
        <SummaryDisputesResponseGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryDisputesResponseGroupsBuilder {
    currency: Option<HashMap<String, i64>>,
    status: Option<SummaryDisputesResponseGroupsStatus>,
}

impl SummaryDisputesResponseGroupsBuilder {
    pub fn currency(mut self, value: HashMap<String, i64>) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn status(mut self, value: SummaryDisputesResponseGroupsStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryDisputesResponseGroups`].
    pub fn build(self) -> Result<SummaryDisputesResponseGroups, BuildError> {
        Ok(SummaryDisputesResponseGroups {
            currency: self.currency,
            status: self.status,
        })
    }
}
