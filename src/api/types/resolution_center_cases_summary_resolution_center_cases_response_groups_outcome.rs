pub use crate::prelude::*;

/// How many of the matching cases ended each way. Every outcome is present, including those with a count of zero; open cases are counted in none of them.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SummaryResolutionCenterCasesResponseGroupsOutcome {
    #[serde(default)]
    pub customer_won: i64,
    #[serde(default)]
    pub merchant_won: i64,
    #[serde(default)]
    pub withdrawn: i64,
}

impl SummaryResolutionCenterCasesResponseGroupsOutcome {
    pub fn builder() -> SummaryResolutionCenterCasesResponseGroupsOutcomeBuilder {
        <SummaryResolutionCenterCasesResponseGroupsOutcomeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryResolutionCenterCasesResponseGroupsOutcomeBuilder {
    customer_won: Option<i64>,
    merchant_won: Option<i64>,
    withdrawn: Option<i64>,
}

impl SummaryResolutionCenterCasesResponseGroupsOutcomeBuilder {
    pub fn customer_won(mut self, value: i64) -> Self {
        self.customer_won = Some(value);
        self
    }

    pub fn merchant_won(mut self, value: i64) -> Self {
        self.merchant_won = Some(value);
        self
    }

    pub fn withdrawn(mut self, value: i64) -> Self {
        self.withdrawn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryResolutionCenterCasesResponseGroupsOutcome`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_won`](SummaryResolutionCenterCasesResponseGroupsOutcomeBuilder::customer_won)
    /// - [`merchant_won`](SummaryResolutionCenterCasesResponseGroupsOutcomeBuilder::merchant_won)
    /// - [`withdrawn`](SummaryResolutionCenterCasesResponseGroupsOutcomeBuilder::withdrawn)
    pub fn build(self) -> Result<SummaryResolutionCenterCasesResponseGroupsOutcome, BuildError> {
        Ok(SummaryResolutionCenterCasesResponseGroupsOutcome {
            customer_won: self
                .customer_won
                .ok_or_else(|| BuildError::missing_field("customer_won"))?,
            merchant_won: self
                .merchant_won
                .ok_or_else(|| BuildError::missing_field("merchant_won"))?,
            withdrawn: self
                .withdrawn
                .ok_or_else(|| BuildError::missing_field("withdrawn"))?,
        })
    }
}
