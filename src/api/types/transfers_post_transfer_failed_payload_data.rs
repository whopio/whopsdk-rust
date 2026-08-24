pub use crate::prelude::*;

/// A transfer of credit between two ledger accounts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostTransferFailedPayloadData {
    /// Transfer amount.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// When the transfer was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The user who initiated the transfer, such as the team member who sent a manual payout. Null if the creator is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<PostTransferFailedPayloadDataCreatedByUser>,
    /// Transfer currency.
    #[serde(default)]
    pub currency: String,
    /// Account or user receiving funds.
    pub destination: PostTransferFailedPayloadDataDestination,
    /// Destination ledger account ID.
    #[serde(default)]
    pub destination_ledger_account_id: String,
    /// When the transfer failed, as an ISO 8601 timestamp. Null unless the transfer has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<DateTime<FixedOffset>>,
    /// Machine-readable code for why the transfer failed. Null unless the transfer has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// Human-readable explanation of why the transfer failed. Null unless the transfer has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// Fee charged for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    /// Transfer ID.
    #[serde(default)]
    pub id: String,
    /// Custom metadata attached to the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Transfer note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The object type. Discriminates the create response from a send or a claim link.
    pub object: PostTransferFailedPayloadDataObject,
    /// Account or user sending funds.
    pub origin: PostTransferFailedPayloadDataOrigin,
    /// Source ledger account ID.
    #[serde(default)]
    pub origin_ledger_account_id: String,
    /// Transfer status. `processing` means the on-chain leg is still executing — poll the transfer until it resolves to `succeeded` or `failed`. A `failed` transfer may be retried under the same ID and later resolve to `succeeded`.
    pub status: PostTransferFailedPayloadDataStatus,
}

impl PostTransferFailedPayloadData {
    pub fn builder() -> PostTransferFailedPayloadDataBuilder {
        <PostTransferFailedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostTransferFailedPayloadDataBuilder {
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by_user: Option<PostTransferFailedPayloadDataCreatedByUser>,
    currency: Option<String>,
    destination: Option<PostTransferFailedPayloadDataDestination>,
    destination_ledger_account_id: Option<String>,
    failed_at: Option<DateTime<FixedOffset>>,
    failure_code: Option<String>,
    failure_reason: Option<String>,
    fee_amount: Option<f64>,
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    notes: Option<String>,
    object: Option<PostTransferFailedPayloadDataObject>,
    origin: Option<PostTransferFailedPayloadDataOrigin>,
    origin_ledger_account_id: Option<String>,
    status: Option<PostTransferFailedPayloadDataStatus>,
}

impl PostTransferFailedPayloadDataBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_user(mut self, value: PostTransferFailedPayloadDataCreatedByUser) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn destination(mut self, value: PostTransferFailedPayloadDataDestination) -> Self {
        self.destination = Some(value);
        self
    }

    pub fn destination_ledger_account_id(mut self, value: impl Into<String>) -> Self {
        self.destination_ledger_account_id = Some(value.into());
        self
    }

    pub fn failed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.failed_at = Some(value);
        self
    }

    pub fn failure_code(mut self, value: impl Into<String>) -> Self {
        self.failure_code = Some(value.into());
        self
    }

    pub fn failure_reason(mut self, value: impl Into<String>) -> Self {
        self.failure_reason = Some(value.into());
        self
    }

    pub fn fee_amount(mut self, value: f64) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn object(mut self, value: PostTransferFailedPayloadDataObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn origin(mut self, value: PostTransferFailedPayloadDataOrigin) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn origin_ledger_account_id(mut self, value: impl Into<String>) -> Self {
        self.origin_ledger_account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: PostTransferFailedPayloadDataStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostTransferFailedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PostTransferFailedPayloadDataBuilder::amount)
    /// - [`created_at`](PostTransferFailedPayloadDataBuilder::created_at)
    /// - [`currency`](PostTransferFailedPayloadDataBuilder::currency)
    /// - [`destination`](PostTransferFailedPayloadDataBuilder::destination)
    /// - [`destination_ledger_account_id`](PostTransferFailedPayloadDataBuilder::destination_ledger_account_id)
    /// - [`id`](PostTransferFailedPayloadDataBuilder::id)
    /// - [`object`](PostTransferFailedPayloadDataBuilder::object)
    /// - [`origin`](PostTransferFailedPayloadDataBuilder::origin)
    /// - [`origin_ledger_account_id`](PostTransferFailedPayloadDataBuilder::origin_ledger_account_id)
    /// - [`status`](PostTransferFailedPayloadDataBuilder::status)
    pub fn build(self) -> Result<PostTransferFailedPayloadData, BuildError> {
        Ok(PostTransferFailedPayloadData {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by_user: self.created_by_user,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
            destination_ledger_account_id: self
                .destination_ledger_account_id
                .ok_or_else(|| BuildError::missing_field("destination_ledger_account_id"))?,
            failed_at: self.failed_at,
            failure_code: self.failure_code,
            failure_reason: self.failure_reason,
            fee_amount: self.fee_amount,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
            notes: self.notes,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            origin: self
                .origin
                .ok_or_else(|| BuildError::missing_field("origin"))?,
            origin_ledger_account_id: self
                .origin_ledger_account_id
                .ok_or_else(|| BuildError::missing_field("origin_ledger_account_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
