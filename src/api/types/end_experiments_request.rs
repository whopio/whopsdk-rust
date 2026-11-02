pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EndExperimentsRequest {
    /// What you learned and why you chose this outcome. Required.
    #[serde(default)]
    pub findings: String,
    /// The treatment that won. Setting it rolls that arm out: every later evaluation returns it to every caller, ignoring targeting and allocation, and exposures stop being recorded. Omit it when control won — an ended experiment with no winning arm evaluates to `control` for everyone, so ending again without one also reverts a rollout recorded by mistake. Not accepted on feature flags, which evaluate to disabled once ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub winning_arm: Option<String>,
    /// Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
    #[serde(skip)]
    pub account_id: Option<String>,
}

impl EndExperimentsRequest {
    pub fn builder() -> EndExperimentsRequestBuilder {
        <EndExperimentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EndExperimentsRequestBuilder {
    findings: Option<String>,
    winning_arm: Option<String>,
    account_id: Option<String>,
}

impl EndExperimentsRequestBuilder {
    pub fn findings(mut self, value: impl Into<String>) -> Self {
        self.findings = Some(value.into());
        self
    }

    pub fn winning_arm(mut self, value: impl Into<String>) -> Self {
        self.winning_arm = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EndExperimentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`findings`](EndExperimentsRequestBuilder::findings)
    pub fn build(self) -> Result<EndExperimentsRequest, BuildError> {
        Ok(EndExperimentsRequest {
            findings: self
                .findings
                .ok_or_else(|| BuildError::missing_field("findings"))?,
            winning_arm: self.winning_arm,
            account_id: self.account_id,
        })
    }
}
