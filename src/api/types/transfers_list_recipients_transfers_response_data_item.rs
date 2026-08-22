pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "object")]
#[non_exhaustive]
pub enum ListRecipientsTransfersResponseDataItem {
    #[serde(rename = "user")]
    #[non_exhaustive]
    User {
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        profile_picture_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        username: Option<String>,
    },

    #[serde(rename = "account")]
    #[non_exhaustive]
    Account {
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        logo_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        route: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl ListRecipientsTransfersResponseDataItem {
    pub fn user(id: String) -> Self {
        Self::User {
            id,
            name: None,
            profile_picture_url: None,
            username: None,
        }
    }

    pub fn account(id: String) -> Self {
        Self::Account {
            id,
            logo_url: None,
            route: None,
            title: None,
        }
    }

    pub fn user_with_name(
        id: String,
        name: String,
        profile_picture_url: Option<String>,
        username: Option<String>,
    ) -> Self {
        Self::User {
            id,
            name: Some(name),
            profile_picture_url,
            username,
        }
    }

    pub fn user_with_profile_picture_url(
        id: String,
        name: Option<String>,
        profile_picture_url: String,
        username: Option<String>,
    ) -> Self {
        Self::User {
            id,
            name,
            profile_picture_url: Some(profile_picture_url),
            username,
        }
    }

    pub fn user_with_username(
        id: String,
        name: Option<String>,
        profile_picture_url: Option<String>,
        username: String,
    ) -> Self {
        Self::User {
            id,
            name,
            profile_picture_url,
            username: Some(username),
        }
    }

    pub fn account_with_logo_url(
        id: String,
        logo_url: String,
        route: Option<String>,
        title: Option<String>,
    ) -> Self {
        Self::Account {
            id,
            logo_url: Some(logo_url),
            route,
            title,
        }
    }

    pub fn account_with_route(
        id: String,
        logo_url: Option<String>,
        route: String,
        title: Option<String>,
    ) -> Self {
        Self::Account {
            id,
            logo_url,
            route: Some(route),
            title,
        }
    }

    pub fn account_with_title(
        id: String,
        logo_url: Option<String>,
        route: Option<String>,
        title: String,
    ) -> Self {
        Self::Account {
            id,
            logo_url,
            route,
            title: Some(title),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
