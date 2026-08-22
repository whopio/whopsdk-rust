pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "object")]
#[non_exhaustive]
pub enum CreateTransfersResponse {
    #[serde(rename = "transfer")]
    #[non_exhaustive]
    Transfer {
        #[serde(default)]
        #[serde(with = "crate::core::number_serializers")]
        amount: f64,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(skip_serializing_if = "Option::is_none")]
        created_by_user: Option<CreateTransfersResponseTransferCreatedByUser>,
        #[serde(default)]
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        #[serde(default)]
        destination_ledger_account_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        failed_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_reason: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fee_amount: Option<f64>,
        #[serde(default)]
        id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        metadata: Option<HashMap<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
        origin: CreateTransfersResponseTransferOrigin,
        #[serde(default)]
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    },

    #[serde(rename = "send")]
    #[non_exhaustive]
    Send {
        #[serde(default)]
        amount: String,
        #[serde(default)]
        currency: String,
        #[serde(default)]
        destination: CreateTransfersResponseSendDestination,
        #[serde(default)]
        source: CreateTransfersResponseSendSource,
        #[serde(default)]
        tx_hash: String,
    },

    #[serde(rename = "claim_link")]
    #[non_exhaustive]
    ClaimLink {
        #[serde(default)]
        amount: String,
        #[serde(default)]
        claim_url: String,
        #[serde(default)]
        currency: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        expires_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        id: String,
        #[serde(default)]
        redeemable_count: i64,
        #[serde(default)]
        source: CreateTransfersResponseClaimLinkSource,
        status: CreateTransfersResponseClaimLinkStatus,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateTransfersResponse {
    pub fn transfer(
        amount: f64,
        created_at: DateTime<FixedOffset>,
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        destination_ledger_account_id: String,
        id: String,
        origin: CreateTransfersResponseTransferOrigin,
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            created_by_user: None,
            currency,
            destination,
            destination_ledger_account_id,
            failed_at: None,
            failure_code: None,
            failure_reason: None,
            fee_amount: None,
            id,
            metadata: None,
            notes: None,
            origin,
            origin_ledger_account_id,
            status,
        }
    }

    pub fn send(
        amount: String,
        currency: String,
        destination: CreateTransfersResponseSendDestination,
        source: CreateTransfersResponseSendSource,
        tx_hash: String,
    ) -> Self {
        Self::Send {
            amount,
            currency,
            destination,
            source,
            tx_hash,
        }
    }

    pub fn claim_link(
        amount: String,
        claim_url: String,
        currency: String,
        id: String,
        redeemable_count: i64,
        source: CreateTransfersResponseClaimLinkSource,
        status: CreateTransfersResponseClaimLinkStatus,
    ) -> Self {
        Self::ClaimLink {
            amount,
            claim_url,
            currency,
            expires_at: None,
            id,
            redeemable_count,
            source,
            status,
        }
    }

    pub fn transfer_with_created_by_user(
        amount: f64,
        created_at: DateTime<FixedOffset>,
        created_by_user: CreateTransfersResponseTransferCreatedByUser,
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        destination_ledger_account_id: String,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee_amount: Option<f64>,
        id: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: Option<String>,
        origin: CreateTransfersResponseTransferOrigin,
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            created_by_user: Some(created_by_user),
            currency,
            destination,
            destination_ledger_account_id,
            failed_at,
            failure_code,
            failure_reason,
            fee_amount,
            id,
            metadata,
            notes,
            origin,
            origin_ledger_account_id,
            status,
        }
    }

    pub fn transfer_with_failed_at(
        amount: f64,
        created_at: DateTime<FixedOffset>,
        created_by_user: Option<CreateTransfersResponseTransferCreatedByUser>,
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        destination_ledger_account_id: String,
        failed_at: DateTime<FixedOffset>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee_amount: Option<f64>,
        id: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: Option<String>,
        origin: CreateTransfersResponseTransferOrigin,
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            created_by_user,
            currency,
            destination,
            destination_ledger_account_id,
            failed_at: Some(failed_at),
            failure_code,
            failure_reason,
            fee_amount,
            id,
            metadata,
            notes,
            origin,
            origin_ledger_account_id,
            status,
        }
    }

    pub fn transfer_with_failure_code(
        amount: f64,
        created_at: DateTime<FixedOffset>,
        created_by_user: Option<CreateTransfersResponseTransferCreatedByUser>,
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        destination_ledger_account_id: String,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: String,
        failure_reason: Option<String>,
        fee_amount: Option<f64>,
        id: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: Option<String>,
        origin: CreateTransfersResponseTransferOrigin,
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            created_by_user,
            currency,
            destination,
            destination_ledger_account_id,
            failed_at,
            failure_code: Some(failure_code),
            failure_reason,
            fee_amount,
            id,
            metadata,
            notes,
            origin,
            origin_ledger_account_id,
            status,
        }
    }

    pub fn transfer_with_failure_reason(
        amount: f64,
        created_at: DateTime<FixedOffset>,
        created_by_user: Option<CreateTransfersResponseTransferCreatedByUser>,
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        destination_ledger_account_id: String,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: String,
        fee_amount: Option<f64>,
        id: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: Option<String>,
        origin: CreateTransfersResponseTransferOrigin,
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            created_by_user,
            currency,
            destination,
            destination_ledger_account_id,
            failed_at,
            failure_code,
            failure_reason: Some(failure_reason),
            fee_amount,
            id,
            metadata,
            notes,
            origin,
            origin_ledger_account_id,
            status,
        }
    }

    pub fn transfer_with_fee_amount(
        amount: f64,
        created_at: DateTime<FixedOffset>,
        created_by_user: Option<CreateTransfersResponseTransferCreatedByUser>,
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        destination_ledger_account_id: String,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee_amount: f64,
        id: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: Option<String>,
        origin: CreateTransfersResponseTransferOrigin,
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            created_by_user,
            currency,
            destination,
            destination_ledger_account_id,
            failed_at,
            failure_code,
            failure_reason,
            fee_amount: Some(fee_amount),
            id,
            metadata,
            notes,
            origin,
            origin_ledger_account_id,
            status,
        }
    }

    pub fn transfer_with_metadata(
        amount: f64,
        created_at: DateTime<FixedOffset>,
        created_by_user: Option<CreateTransfersResponseTransferCreatedByUser>,
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        destination_ledger_account_id: String,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee_amount: Option<f64>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: CreateTransfersResponseTransferOrigin,
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            created_by_user,
            currency,
            destination,
            destination_ledger_account_id,
            failed_at,
            failure_code,
            failure_reason,
            fee_amount,
            id,
            metadata: Some(metadata),
            notes,
            origin,
            origin_ledger_account_id,
            status,
        }
    }

    pub fn transfer_with_notes(
        amount: f64,
        created_at: DateTime<FixedOffset>,
        created_by_user: Option<CreateTransfersResponseTransferCreatedByUser>,
        currency: String,
        destination: CreateTransfersResponseTransferDestination,
        destination_ledger_account_id: String,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee_amount: Option<f64>,
        id: String,
        metadata: Option<HashMap<String, serde_json::Value>>,
        notes: String,
        origin: CreateTransfersResponseTransferOrigin,
        origin_ledger_account_id: String,
        status: CreateTransfersResponseTransferStatus,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            created_by_user,
            currency,
            destination,
            destination_ledger_account_id,
            failed_at,
            failure_code,
            failure_reason,
            fee_amount,
            id,
            metadata,
            notes: Some(notes),
            origin,
            origin_ledger_account_id,
            status,
        }
    }

    pub fn claim_link_with_expires_at(
        amount: String,
        claim_url: String,
        currency: String,
        expires_at: DateTime<FixedOffset>,
        id: String,
        redeemable_count: i64,
        source: CreateTransfersResponseClaimLinkSource,
        status: CreateTransfersResponseClaimLinkStatus,
    ) -> Self {
        Self::ClaimLink {
            amount,
            claim_url,
            currency,
            expires_at: Some(expires_at),
            id,
            redeemable_count,
            source,
            status,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
