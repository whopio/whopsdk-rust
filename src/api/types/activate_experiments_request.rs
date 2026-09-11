pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ActivateExperimentsRequest {
    /// Reporting window length in days. Omit to use the default. This does not automatically end the experiment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_days: Option<i64>,
    /// Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
    #[serde(skip)]
    pub account_id: Option<String>,
}

impl ActivateExperimentsRequest {
    pub fn builder() -> ActivateExperimentsRequestBuilder {
        <ActivateExperimentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ActivateExperimentsRequestBuilder {
    duration_days: Option<i64>,
    account_id: Option<String>,
}

impl ActivateExperimentsRequestBuilder {
    pub fn duration_days(mut self, value: i64) -> Self {
        self.duration_days = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ActivateExperimentsRequest`].
    pub fn build(self) -> Result<ActivateExperimentsRequest, BuildError> {
        Ok(ActivateExperimentsRequest {
            duration_days: self.duration_days,
            account_id: self.account_id,
        })
    }
}
