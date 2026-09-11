pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunEconomicIntelligenceRequest {
    /// Account ID, prefixed `biz_`. Defaults to the API key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// What the owner wants, in their own words. Up to 1000 characters.
    #[serde(default)]
    pub input: String,
}

impl RunEconomicIntelligenceRequest {
    pub fn builder() -> RunEconomicIntelligenceRequestBuilder {
        <RunEconomicIntelligenceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunEconomicIntelligenceRequestBuilder {
    account_id: Option<String>,
    input: Option<String>,
}

impl RunEconomicIntelligenceRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunEconomicIntelligenceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](RunEconomicIntelligenceRequestBuilder::input)
    pub fn build(self) -> Result<RunEconomicIntelligenceRequest, BuildError> {
        Ok(RunEconomicIntelligenceRequest {
            account_id: self.account_id,
            input: self
                .input
                .ok_or_else(|| BuildError::missing_field("input"))?,
        })
    }
}
