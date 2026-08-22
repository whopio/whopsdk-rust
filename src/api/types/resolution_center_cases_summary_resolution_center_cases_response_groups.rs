pub use crate::prelude::*;

/// One entry per requested breakdown, keyed by the field it groups on. A field you did not ask for is absent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SummaryResolutionCenterCasesResponseGroups {
    /// How many of the matching cases ended each way. Every outcome is present, including those with a count of zero; open cases are counted in none of them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<SummaryResolutionCenterCasesResponseGroupsOutcome>,
    /// How many of the matching cases were opened for each reason. Every reason is present, including those with a count of zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<SummaryResolutionCenterCasesResponseGroupsReason>,
    /// How many of the matching cases are in each status. Every status is present, including those with a count of zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SummaryResolutionCenterCasesResponseGroupsStatus>,
}

impl SummaryResolutionCenterCasesResponseGroups {
    pub fn builder() -> SummaryResolutionCenterCasesResponseGroupsBuilder {
        <SummaryResolutionCenterCasesResponseGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryResolutionCenterCasesResponseGroupsBuilder {
    outcome: Option<SummaryResolutionCenterCasesResponseGroupsOutcome>,
    reason: Option<SummaryResolutionCenterCasesResponseGroupsReason>,
    status: Option<SummaryResolutionCenterCasesResponseGroupsStatus>,
}

impl SummaryResolutionCenterCasesResponseGroupsBuilder {
    pub fn outcome(mut self, value: SummaryResolutionCenterCasesResponseGroupsOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn reason(mut self, value: SummaryResolutionCenterCasesResponseGroupsReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn status(mut self, value: SummaryResolutionCenterCasesResponseGroupsStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryResolutionCenterCasesResponseGroups`].
    pub fn build(self) -> Result<SummaryResolutionCenterCasesResponseGroups, BuildError> {
        Ok(SummaryResolutionCenterCasesResponseGroups {
            outcome: self.outcome,
            reason: self.reason,
            status: self.status,
        })
    }
}
