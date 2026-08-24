pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "typename")]
#[non_exhaustive]
pub enum PostTransferCompletedPayloadDataDestination {
    #[non_exhaustive]
    Company {
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        route: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    #[non_exhaustive]
    User {
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PostTransferCompletedPayloadDataDestination {
    pub fn company(id: String) -> Self {
        Self::Company {
            id,
            route: None,
            title: None,
        }
    }

    pub fn user(id: String) -> Self {
        Self::User {
            id,
            name: None,
            username: None,
        }
    }

    pub fn company_with_route(id: String, route: String, title: Option<String>) -> Self {
        Self::Company {
            id,
            route: Some(route),
            title,
        }
    }

    pub fn company_with_title(id: String, route: Option<String>, title: String) -> Self {
        Self::Company {
            id,
            route,
            title: Some(title),
        }
    }

    pub fn user_with_name(id: String, name: String, username: Option<String>) -> Self {
        Self::User {
            id,
            name: Some(name),
            username,
        }
    }

    pub fn user_with_username(id: String, name: Option<String>, username: String) -> Self {
        Self::User {
            id,
            name,
            username: Some(username),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
