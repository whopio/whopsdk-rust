pub use crate::prelude::*;

/// A transfer of credit between two ledger accounts.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListTransfersResponseDataItem {
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
    pub created_by_user: Option<ListTransfersResponseDataItemCreatedByUser>,
    /// Transfer currency.
    #[serde(default)]
    pub currency: String,
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
    /// The object type.
    pub object: ListTransfersResponseDataItemObject,
    /// Source ledger account ID.
    #[serde(default)]
    pub origin_ledger_account_id: String,
    /// Transfer status. `processing` means the on-chain leg is still executing — poll the transfer until it resolves to `succeeded` or `failed`. A `failed` transfer may be retried under the same ID and later resolve to `succeeded`.
    pub status: ListTransfersResponseDataItemStatus,
}

impl ListTransfersResponseDataItem {
    pub fn builder() -> ListTransfersResponseDataItemBuilder {
        <ListTransfersResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTransfersResponseDataItemBuilder {
    amount: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by_user: Option<ListTransfersResponseDataItemCreatedByUser>,
    currency: Option<String>,
    destination_ledger_account_id: Option<String>,
    failed_at: Option<DateTime<FixedOffset>>,
    failure_code: Option<String>,
    failure_reason: Option<String>,
    fee_amount: Option<f64>,
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    notes: Option<String>,
    object: Option<ListTransfersResponseDataItemObject>,
    origin_ledger_account_id: Option<String>,
    status: Option<ListTransfersResponseDataItemStatus>,
}

impl ListTransfersResponseDataItemBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by_user(mut self, value: ListTransfersResponseDataItemCreatedByUser) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn object(mut self, value: ListTransfersResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn origin_ledger_account_id(mut self, value: impl Into<String>) -> Self {
        self.origin_ledger_account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListTransfersResponseDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTransfersResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ListTransfersResponseDataItemBuilder::amount)
    /// - [`created_at`](ListTransfersResponseDataItemBuilder::created_at)
    /// - [`currency`](ListTransfersResponseDataItemBuilder::currency)
    /// - [`destination_ledger_account_id`](ListTransfersResponseDataItemBuilder::destination_ledger_account_id)
    /// - [`id`](ListTransfersResponseDataItemBuilder::id)
    /// - [`object`](ListTransfersResponseDataItemBuilder::object)
    /// - [`origin_ledger_account_id`](ListTransfersResponseDataItemBuilder::origin_ledger_account_id)
    /// - [`status`](ListTransfersResponseDataItemBuilder::status)
    pub fn build(self) -> Result<ListTransfersResponseDataItem, BuildError> {
        Ok(ListTransfersResponseDataItem {
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
            origin_ledger_account_id: self
                .origin_ledger_account_id
                .ok_or_else(|| BuildError::missing_field("origin_ledger_account_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
