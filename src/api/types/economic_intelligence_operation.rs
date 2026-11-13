pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EconomicIntelligenceOperation {
    /// Concise description of the operation and affected resource, or null for older recommendations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// API operation or assistant tool name.
    #[serde(default)]
    pub tool_name: String,
}

impl EconomicIntelligenceOperation {
    pub fn builder() -> EconomicIntelligenceOperationBuilder {
        <EconomicIntelligenceOperationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EconomicIntelligenceOperationBuilder {
    description: Option<String>,
    tool_name: Option<String>,
}

impl EconomicIntelligenceOperationBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EconomicIntelligenceOperation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_name`](EconomicIntelligenceOperationBuilder::tool_name)
    pub fn build(self) -> Result<EconomicIntelligenceOperation, BuildError> {
        Ok(EconomicIntelligenceOperation {
            description: self.description,
            tool_name: self
                .tool_name
                .ok_or_else(|| BuildError::missing_field("tool_name"))?,
        })
    }
}
