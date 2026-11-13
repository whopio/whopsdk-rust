pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AutomationStepConfig {
    AutomationStepWaitConfig(AutomationStepWaitConfig),

    AutomationStepEmailConfig(AutomationStepEmailConfig),

    AutomationStepMessageConfig(AutomationStepMessageConfig),
}

impl AutomationStepConfig {
    pub fn is_automation_step_wait_config(&self) -> bool {
        matches!(self, Self::AutomationStepWaitConfig(_))
    }

    pub fn is_automation_step_email_config(&self) -> bool {
        matches!(self, Self::AutomationStepEmailConfig(_))
    }

    pub fn is_automation_step_message_config(&self) -> bool {
        matches!(self, Self::AutomationStepMessageConfig(_))
    }

    pub fn as_automation_step_wait_config(&self) -> Option<&AutomationStepWaitConfig> {
        match self {
            Self::AutomationStepWaitConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_automation_step_wait_config(self) -> Option<AutomationStepWaitConfig> {
        match self {
            Self::AutomationStepWaitConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_automation_step_email_config(&self) -> Option<&AutomationStepEmailConfig> {
        match self {
            Self::AutomationStepEmailConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_automation_step_email_config(self) -> Option<AutomationStepEmailConfig> {
        match self {
            Self::AutomationStepEmailConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_automation_step_message_config(&self) -> Option<&AutomationStepMessageConfig> {
        match self {
            Self::AutomationStepMessageConfig(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_automation_step_message_config(self) -> Option<AutomationStepMessageConfig> {
        match self {
            Self::AutomationStepMessageConfig(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AutomationStepConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AutomationStepWaitConfig(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AutomationStepEmailConfig(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::AutomationStepMessageConfig(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
