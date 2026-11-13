pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AutomationStepWaitConfig {
    /// Seconds the member waits before the next step, from 1 to 31,536,000 (365 days).
    pub delay_seconds: i64,
}

impl AutomationStepWaitConfig {
    pub fn builder() -> AutomationStepWaitConfigBuilder {
        <AutomationStepWaitConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationStepWaitConfigBuilder {
    delay_seconds: Option<i64>,
}

impl AutomationStepWaitConfigBuilder {
    pub fn delay_seconds(mut self, value: i64) -> Self {
        self.delay_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationStepWaitConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`delay_seconds`](AutomationStepWaitConfigBuilder::delay_seconds)
    pub fn build(self) -> Result<AutomationStepWaitConfig, BuildError> {
        Ok(AutomationStepWaitConfig {
            delay_seconds: self
                .delay_seconds
                .ok_or_else(|| BuildError::missing_field("delay_seconds"))?,
        })
    }
}
