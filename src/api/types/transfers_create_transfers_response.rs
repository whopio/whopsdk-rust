pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "object")]
#[non_exhaustive]
pub enum CreateTransfersResponse {
    #[serde(rename = "transfer")]
    #[non_exhaustive]
    Transfer {
        #[serde(skip_serializing_if = "Option::is_none")]
        amount: Option<Money>,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
        #[serde(skip_serializing_if = "Option::is_none")]
        destination: Option<CreateTransfersResponseTransferDestination>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        failed_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_code: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        failure_reason: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        fee: Option<Money>,
        #[serde(default)]
        id: String,
        #[serde(default)]
        metadata: HashMap<String, serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        status_changed_at: Option<DateTime<FixedOffset>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset::option")]
        succeeded_at: Option<DateTime<FixedOffset>>,
        #[serde(default)]
        tracking_url: String,
    },

    #[serde(rename = "claim_link")]
    #[non_exhaustive]
    ClaimLink {
        #[serde(default)]
        amount: String,
        #[serde(default)]
        claim_url: String,
        #[serde(default)]
        claimable: bool,
        #[serde(default)]
        code: String,
        #[serde(default)]
        #[serde(with = "crate::core::flexible_datetime::offset")]
        created_at: DateTime<FixedOffset>,
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
        redeemed_count: i64,
        #[serde(default)]
        remaining_claims: i64,
        sender: CreateTransfersResponseClaimLinkSender,
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
        created_at: DateTime<FixedOffset>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        status: CreateTransfersResponseTransferStatus,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount: None,
            created_at,
            destination: None,
            failed_at: None,
            failure_code: None,
            failure_reason: None,
            fee: None,
            id,
            metadata,
            notes: None,
            origin: None,
            status,
            status_changed_at: None,
            succeeded_at: None,
            tracking_url,
        }
    }

    pub fn claim_link(
        amount: String,
        claim_url: String,
        claimable: bool,
        code: String,
        created_at: DateTime<FixedOffset>,
        currency: String,
        id: String,
        redeemable_count: i64,
        redeemed_count: i64,
        remaining_claims: i64,
        sender: CreateTransfersResponseClaimLinkSender,
        source: CreateTransfersResponseClaimLinkSource,
        status: CreateTransfersResponseClaimLinkStatus,
    ) -> Self {
        Self::ClaimLink {
            amount,
            claim_url,
            claimable,
            code,
            created_at,
            currency,
            expires_at: None,
            id,
            redeemable_count,
            redeemed_count,
            remaining_claims,
            sender,
            source,
            status,
        }
    }

    pub fn transfer_with_amount(
        amount: Money,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount: Some(amount),
            created_at,
            destination,
            failed_at,
            failure_code,
            failure_reason,
            fee,
            id,
            metadata,
            notes,
            origin,
            status,
            status_changed_at,
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_destination(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: CreateTransfersResponseTransferDestination,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination: Some(destination),
            failed_at,
            failure_code,
            failure_reason,
            fee,
            id,
            metadata,
            notes,
            origin,
            status,
            status_changed_at,
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_failed_at(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: DateTime<FixedOffset>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination,
            failed_at: Some(failed_at),
            failure_code,
            failure_reason,
            fee,
            id,
            metadata,
            notes,
            origin,
            status,
            status_changed_at,
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_failure_code(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: String,
        failure_reason: Option<String>,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination,
            failed_at,
            failure_code: Some(failure_code),
            failure_reason,
            fee,
            id,
            metadata,
            notes,
            origin,
            status,
            status_changed_at,
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_failure_reason(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: String,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination,
            failed_at,
            failure_code,
            failure_reason: Some(failure_reason),
            fee,
            id,
            metadata,
            notes,
            origin,
            status,
            status_changed_at,
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_fee(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee: Money,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination,
            failed_at,
            failure_code,
            failure_reason,
            fee: Some(fee),
            id,
            metadata,
            notes,
            origin,
            status,
            status_changed_at,
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_notes(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: String,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination,
            failed_at,
            failure_code,
            failure_reason,
            fee,
            id,
            metadata,
            notes: Some(notes),
            origin,
            status,
            status_changed_at,
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_origin(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: CreateTransfersResponseTransferOrigin,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination,
            failed_at,
            failure_code,
            failure_reason,
            fee,
            id,
            metadata,
            notes,
            origin: Some(origin),
            status,
            status_changed_at,
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_status_changed_at(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: DateTime<FixedOffset>,
        succeeded_at: Option<DateTime<FixedOffset>>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination,
            failed_at,
            failure_code,
            failure_reason,
            fee,
            id,
            metadata,
            notes,
            origin,
            status,
            status_changed_at: Some(status_changed_at),
            succeeded_at,
            tracking_url,
        }
    }

    pub fn transfer_with_succeeded_at(
        amount: Option<Money>,
        created_at: DateTime<FixedOffset>,
        destination: Option<CreateTransfersResponseTransferDestination>,
        failed_at: Option<DateTime<FixedOffset>>,
        failure_code: Option<String>,
        failure_reason: Option<String>,
        fee: Option<Money>,
        id: String,
        metadata: HashMap<String, serde_json::Value>,
        notes: Option<String>,
        origin: Option<CreateTransfersResponseTransferOrigin>,
        status: CreateTransfersResponseTransferStatus,
        status_changed_at: Option<DateTime<FixedOffset>>,
        succeeded_at: DateTime<FixedOffset>,
        tracking_url: String,
    ) -> Self {
        Self::Transfer {
            amount,
            created_at,
            destination,
            failed_at,
            failure_code,
            failure_reason,
            fee,
            id,
            metadata,
            notes,
            origin,
            status,
            status_changed_at,
            succeeded_at: Some(succeeded_at),
            tracking_url,
        }
    }

    pub fn claim_link_with_expires_at(
        amount: String,
        claim_url: String,
        claimable: bool,
        code: String,
        created_at: DateTime<FixedOffset>,
        currency: String,
        expires_at: DateTime<FixedOffset>,
        id: String,
        redeemable_count: i64,
        redeemed_count: i64,
        remaining_claims: i64,
        sender: CreateTransfersResponseClaimLinkSender,
        source: CreateTransfersResponseClaimLinkSource,
        status: CreateTransfersResponseClaimLinkStatus,
    ) -> Self {
        Self::ClaimLink {
            amount,
            claim_url,
            claimable,
            code,
            created_at,
            currency,
            expires_at: Some(expires_at),
            id,
            redeemable_count,
            redeemed_count,
            remaining_claims,
            sender,
            source,
            status,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
