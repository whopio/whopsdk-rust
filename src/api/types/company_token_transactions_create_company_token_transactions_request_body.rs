pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "transaction_type")]
#[non_exhaustive]
pub enum CreateCompanyTokenTransactionsRequestBody {
    #[serde(rename = "transfer")]
    #[non_exhaustive]
    Transfer {
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(default)]
        company_id: String,
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
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(default)]
        company_id: String,
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
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(default)]
        company_id: String,
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
        amount: f64,
        company_id: String,
        destination_user_id: String,
        user_id: String,
    ) -> Self {
        Self::Transfer {
            amount,
            company_id,
            description: None,
            destination_user_id,
            idempotency_key: None,
            user_id,
        }
    }

    pub fn add(amount: f64, company_id: String, user_id: String) -> Self {
        Self::Add {
            amount,
            company_id,
            description: None,
            idempotency_key: None,
            user_id,
        }
    }

    pub fn subtract(amount: f64, company_id: String, user_id: String) -> Self {
        Self::Subtract {
            amount,
            company_id,
            description: None,
            idempotency_key: None,
            user_id,
        }
    }

    pub fn transfer_with_description(
        amount: f64,
        company_id: String,
        description: String,
        destination_user_id: String,
        idempotency_key: Option<String>,
        user_id: String,
    ) -> Self {
        Self::Transfer {
            amount,
            company_id,
            description: Some(description),
            destination_user_id,
            idempotency_key,
            user_id,
        }
    }

    pub fn transfer_with_idempotency_key(
        amount: f64,
        company_id: String,
        description: Option<String>,
        destination_user_id: String,
        idempotency_key: String,
        user_id: String,
    ) -> Self {
        Self::Transfer {
            amount,
            company_id,
            description,
            destination_user_id,
            idempotency_key: Some(idempotency_key),
            user_id,
        }
    }

    pub fn add_with_description(
        amount: f64,
        company_id: String,
        description: String,
        idempotency_key: Option<String>,
        user_id: String,
    ) -> Self {
        Self::Add {
            amount,
            company_id,
            description: Some(description),
            idempotency_key,
            user_id,
        }
    }

    pub fn add_with_idempotency_key(
        amount: f64,
        company_id: String,
        description: Option<String>,
        idempotency_key: String,
        user_id: String,
    ) -> Self {
        Self::Add {
            amount,
            company_id,
            description,
            idempotency_key: Some(idempotency_key),
            user_id,
        }
    }

    pub fn subtract_with_description(
        amount: f64,
        company_id: String,
        description: String,
        idempotency_key: Option<String>,
        user_id: String,
    ) -> Self {
        Self::Subtract {
            amount,
            company_id,
            description: Some(description),
            idempotency_key,
            user_id,
        }
    }

    pub fn subtract_with_idempotency_key(
        amount: f64,
        company_id: String,
        description: Option<String>,
        idempotency_key: String,
        user_id: String,
    ) -> Self {
        Self::Subtract {
            amount,
            company_id,
            description,
            idempotency_key: Some(idempotency_key),
            user_id,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
