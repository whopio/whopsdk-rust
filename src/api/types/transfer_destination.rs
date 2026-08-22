pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "typename")]
#[non_exhaustive]
pub enum TransferDestination {
    #[non_exhaustive]
    User {
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(default)]
        username: String,
    },

    #[non_exhaustive]
    Company {
        #[serde(default)]
        id: String,
        #[serde(default)]
        route: String,
        #[serde(default)]
        title: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl TransferDestination {
    pub fn user(id: String, username: String) -> Self {
        Self::User {
            id,
            name: None,
            username,
        }
    }

    pub fn company(id: String, route: String, title: String) -> Self {
        Self::Company { id, route, title }
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
