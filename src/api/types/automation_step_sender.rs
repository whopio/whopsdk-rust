pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "object")]
#[non_exhaustive]
pub enum AutomationStepSender {
    #[serde(rename = "email_address")]
    #[non_exhaustive]
    EmailAddress {
        #[serde(default)]
        address: String,
        #[serde(default)]
        id: String,
    },

    #[serde(rename = "user")]
    #[non_exhaustive]
    User {
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(default)]
        username: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl AutomationStepSender {
    pub fn email_address(address: String, id: String) -> Self {
        Self::EmailAddress { address, id }
    }

    pub fn user(id: String, username: String) -> Self {
        Self::User {
            id,
            name: None,
            username,
        }
    }

    pub fn user_with_name(id: String, name: String, username: String) -> Self {
        Self::User {
            id,
            name: Some(name),
            username,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
