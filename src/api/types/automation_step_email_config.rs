pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AutomationStepEmailConfig {
    /// The mailbox the email comes from, prefixed `emaddr_`: an email address on the account whose domain can send email. Naming one needs what `/email_addresses` asks to use it. `null` sends from the account's name through Whop.
    pub sender_id: Option<String>,
}

impl AutomationStepEmailConfig {
    pub fn builder() -> AutomationStepEmailConfigBuilder {
        <AutomationStepEmailConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationStepEmailConfigBuilder {
    sender_id: Option<String>,
}

impl AutomationStepEmailConfigBuilder {
    pub fn sender_id(mut self, value: impl Into<String>) -> Self {
        self.sender_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationStepEmailConfig`].
    pub fn build(self) -> Result<AutomationStepEmailConfig, BuildError> {
        Ok(AutomationStepEmailConfig {
            sender_id: self.sender_id,
        })
    }
}
