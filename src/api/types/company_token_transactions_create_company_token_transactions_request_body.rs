pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "transaction_type")]
#[non_exhaustive]
pub enum CreateCompanyTokenTransactionsRequestBody {
    #[serde(rename = "transfer")]
    #[non_exhaustive]
    Transfer {
        #[serde(default)]
        account_id: String,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(default)]
        destination_user_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        idempotency_key: Option<String>,
        #[serde(default)]
        user_id: String,
    },

    #[serde(rename = "add")]
    #[non_exhaustive]
    Add {
        #[serde(default)]
        account_id: String,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        idempotency_key: Option<String>,
        #[serde(default)]
        user_id: String,
    },

    #[serde(rename = "subtract")]
    #[non_exhaustive]
    Subtract {
        #[serde(default)]
        account_id: String,
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        idempotency_key: Option<String>,
        #[serde(default)]
        user_id: String,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateCompanyTokenTransactionsRequestBody {
    pub fn transfer(
        account_id: String,
        amount: f64,
        destination_user_id: String,
        user_id: String,
    ) -> Self {
        Self::Transfer {
            account_id,
            amount,
            description: None,
            destination_user_id,
            idempotency_key: None,
            user_id,
        }
    }

    pub fn add(account_id: String, amount: f64, user_id: String) -> Self {
        Self::Add {
            account_id,
            amount,
            description: None,
            idempotency_key: None,
            user_id,
        }
    }

    pub fn subtract(account_id: String, amount: f64, user_id: String) -> Self {
        Self::Subtract {
            account_id,
            amount,
            description: None,
            idempotency_key: None,
            user_id,
        }
    }

    pub fn transfer_with_description(
        account_id: String,
        amount: f64,
        description: String,
        destination_user_id: String,
        idempotency_key: Option<String>,
        user_id: String,
    ) -> Self {
        Self::Transfer {
            account_id,
            amount,
            description: Some(description),
            destination_user_id,
            idempotency_key,
            user_id,
        }
    }

    pub fn transfer_with_idempotency_key(
        account_id: String,
        amount: f64,
        description: Option<String>,
        destination_user_id: String,
        idempotency_key: String,
        user_id: String,
    ) -> Self {
        Self::Transfer {
            account_id,
            amount,
            description,
            destination_user_id,
            idempotency_key: Some(idempotency_key),
            user_id,
        }
    }

    pub fn add_with_description(
        account_id: String,
        amount: f64,
        description: String,
        idempotency_key: Option<String>,
        user_id: String,
    ) -> Self {
        Self::Add {
            account_id,
            amount,
            description: Some(description),
            idempotency_key,
            user_id,
        }
    }

    pub fn add_with_idempotency_key(
        account_id: String,
        amount: f64,
        description: Option<String>,
        idempotency_key: String,
        user_id: String,
    ) -> Self {
        Self::Add {
            account_id,
            amount,
            description,
            idempotency_key: Some(idempotency_key),
            user_id,
        }
    }

    pub fn subtract_with_description(
        account_id: String,
        amount: f64,
        description: String,
        idempotency_key: Option<String>,
        user_id: String,
    ) -> Self {
        Self::Subtract {
            account_id,
            amount,
            description: Some(description),
            idempotency_key,
            user_id,
        }
    }

    pub fn subtract_with_idempotency_key(
        account_id: String,
        amount: f64,
        description: Option<String>,
        idempotency_key: String,
        user_id: String,
    ) -> Self {
        Self::Subtract {
            account_id,
            amount,
            description,
            idempotency_key: Some(idempotency_key),
            user_id,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
