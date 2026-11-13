pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AutomationStepMessageConfig {
    /// The team member the message comes from, as a user ID prefixed `user_`. Only that member can name themselves. `null` sends as the account owner.
    pub sender_id: Option<String>,
}

impl AutomationStepMessageConfig {
    pub fn builder() -> AutomationStepMessageConfigBuilder {
        <AutomationStepMessageConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationStepMessageConfigBuilder {
    sender_id: Option<String>,
}

impl AutomationStepMessageConfigBuilder {
    pub fn sender_id(mut self, value: impl Into<String>) -> Self {
        self.sender_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationStepMessageConfig`].
    pub fn build(self) -> Result<AutomationStepMessageConfig, BuildError> {
        Ok(AutomationStepMessageConfig {
            sender_id: self.sender_id,
        })
    }
}
