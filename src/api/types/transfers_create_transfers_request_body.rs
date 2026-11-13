pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CreateTransfersRequestBody {
    #[serde(rename = "balance")]
    #[non_exhaustive]
    Balance {
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(default)]
        currency: String,
        #[serde(default)]
        destination_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        feed_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        feed_type: Option<CreateTransfersRequestBodyBalanceFeedType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        idempotence_key: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
        #[serde(default)]
        origin_id: String,
    },

    #[serde(rename = "claim_link")]
    #[non_exhaustive]
    ClaimLink {
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        currency: Option<CreateTransfersRequestBodyClaimLinkCurrency>,
        #[serde(skip_serializing_if = "Option::is_none")]
        expires_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        origin_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        redeemable_count: Option<i64>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateTransfersRequestBody {
    pub fn balance(
        amount: f64,
        currency: String,
        destination_id: String,
        origin_id: String,
    ) -> Self {
        Self::Balance {
            amount,
            currency,
            destination_id,
            feed_id: None,
            feed_type: None,
            idempotence_key: None,
            metadata: None,
            notes: None,
            origin_id,
        }
    }

    pub fn claim_link(amount: f64, origin_id: String) -> Self {
        Self::ClaimLink {
            amount,
            currency: None,
            expires_at: None,
            origin_id,
            redeemable_count: None,
        }
    }

    pub fn balance_with_feed_id(
        amount: f64,
        currency: String,
        destination_id: String,
        feed_id: String,
        feed_type: Option<CreateTransfersRequestBodyBalanceFeedType>,
        idempotence_key: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: Option<String>,
        origin_id: String,
    ) -> Self {
        Self::Balance {
            amount,
            currency,
            destination_id,
            feed_id: Some(feed_id),
            feed_type,
            idempotence_key,
            metadata,
            notes,
            origin_id,
        }
    }

    pub fn balance_with_feed_type(
        amount: f64,
        currency: String,
        destination_id: String,
        feed_id: Option<String>,
        feed_type: CreateTransfersRequestBodyBalanceFeedType,
        idempotence_key: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: Option<String>,
        origin_id: String,
    ) -> Self {
        Self::Balance {
            amount,
            currency,
            destination_id,
            feed_id,
            feed_type: Some(feed_type),
            idempotence_key,
            metadata,
            notes,
            origin_id,
        }
    }

    pub fn balance_with_idempotence_key(
        amount: f64,
        currency: String,
        destination_id: String,
        feed_id: Option<String>,
        feed_type: Option<CreateTransfersRequestBodyBalanceFeedType>,
        idempotence_key: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: Option<String>,
        origin_id: String,
    ) -> Self {
        Self::Balance {
            amount,
            currency,
            destination_id,
            feed_id,
            feed_type,
            idempotence_key: Some(idempotence_key),
            metadata,
            notes,
            origin_id,
        }
    }

    pub fn balance_with_metadata(
        amount: f64,
        currency: String,
        destination_id: String,
        feed_id: Option<String>,
        feed_type: Option<CreateTransfersRequestBodyBalanceFeedType>,
        idempotence_key: Option<String>,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin_id: String,
    ) -> Self {
        Self::Balance {
            amount,
            currency,
            destination_id,
            feed_id,
            feed_type,
            idempotence_key,
            metadata: Some(metadata),
            notes,
            origin_id,
        }
    }

    pub fn balance_with_notes(
        amount: f64,
        currency: String,
        destination_id: String,
        feed_id: Option<String>,
        feed_type: Option<CreateTransfersRequestBodyBalanceFeedType>,
        idempotence_key: Option<String>,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: String,
        origin_id: String,
    ) -> Self {
        Self::Balance {
            amount,
            currency,
            destination_id,
            feed_id,
            feed_type,
            idempotence_key,
            metadata,
            notes: Some(notes),
            origin_id,
        }
    }

    pub fn claim_link_with_currency(
        amount: f64,
        currency: CreateTransfersRequestBodyClaimLinkCurrency,
        expires_at: Option<DateTime<FixedOffset>>,
        origin_id: String,
        redeemable_count: Option<i64>,
    ) -> Self {
        Self::ClaimLink {
            amount,
            currency: Some(currency),
            expires_at,
            origin_id,
            redeemable_count,
        }
    }

    pub fn claim_link_with_expires_at(
        amount: f64,
        currency: Option<CreateTransfersRequestBodyClaimLinkCurrency>,
        expires_at: DateTime<FixedOffset>,
        origin_id: String,
        redeemable_count: Option<i64>,
    ) -> Self {
        Self::ClaimLink {
            amount,
            currency,
            expires_at: Some(expires_at),
            origin_id,
            redeemable_count,
        }
    }

    pub fn claim_link_with_redeemable_count(
        amount: f64,
        currency: Option<CreateTransfersRequestBodyClaimLinkCurrency>,
        expires_at: Option<DateTime<FixedOffset>>,
        origin_id: String,
        redeemable_count: i64,
    ) -> Self {
        Self::ClaimLink {
            amount,
            currency,
            expires_at,
            origin_id,
            redeemable_count: Some(redeemable_count),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
