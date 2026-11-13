pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "platform")]
#[non_exhaustive]
pub enum CreateSocialAccountsRequestBody {
    #[serde(rename = "facebook")]
    #[non_exhaustive]
    Facebook {
        #[serde(skip_serializing_if = "Option::is_none")]
        account_id: Option<String>,
    },

    #[serde(rename = "tiktok")]
    #[non_exhaustive]
    Tiktok {
        #[serde(skip_serializing_if = "Option::is_none")]
        account_id: Option<String>,
    },

    #[serde(rename = "klaviyo")]
    #[non_exhaustive]
    Klaviyo {
        #[serde(skip_serializing_if = "Option::is_none")]
        account_id: Option<String>,
        #[serde(default)]
        api_key: String,
    },

    #[serde(rename = "aimerce")]
    #[non_exhaustive]
    Aimerce {
        #[serde(skip_serializing_if = "Option::is_none")]
        account_id: Option<String>,
        #[serde(default)]
        api_key: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateSocialAccountsRequestBody {
    pub fn facebook() -> Self {
        Self::Facebook { account_id: None }
    }

    pub fn tiktok() -> Self {
        Self::Tiktok { account_id: None }
    }

    pub fn klaviyo(api_key: String) -> Self {
        Self::Klaviyo {
            account_id: None,
            api_key,
        }
    }

    pub fn aimerce(api_key: String) -> Self {
        Self::Aimerce {
            account_id: None,
            api_key,
        }
    }

    pub fn facebook_with_account_id(account_id: String) -> Self {
        Self::Facebook {
            account_id: Some(account_id),
        }
    }

    pub fn tiktok_with_account_id(account_id: String) -> Self {
        Self::Tiktok {
            account_id: Some(account_id),
        }
    }

    pub fn klaviyo_with_account_id(account_id: String, api_key: String) -> Self {
        Self::Klaviyo {
            account_id: Some(account_id),
            api_key,
        }
    }

    pub fn aimerce_with_account_id(account_id: String, api_key: String) -> Self {
        Self::Aimerce {
            account_id: Some(account_id),
            api_key,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
