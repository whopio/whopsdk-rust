pub use crate::prelude::*;

/// A transfer between Whop accounts or users.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrieveTransfersResponse {
    /// Amount credited to the recipient. Null while the transfer has not succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// When the transfer was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Business account or user receiving the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<RetrieveTransfersResponseDestination>,
    /// Recorded failure time. Null unless the transfer has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub failed_at: Option<DateTime<FixedOffset>>,
    /// Machine-readable failure code. Null unless the transfer has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    /// Failure explanation. Null unless the transfer has failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// All applicable transfer fees, including platform markup. Null when the authoritative total is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Money>,
    /// Transfer ID.
    #[serde(default)]
    pub id: String,
    /// Custom metadata attached to the transfer.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Transfer note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The object type.
    pub object: RetrieveTransfersResponseObject,
    /// Business account or user sending the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<RetrieveTransfersResponseOrigin>,
    /// Current transfer status. A transfer can be created `processing` and settle a few seconds later; poll Retrieve Transfer or listen for the `transfer.completed` and `transfer.failed` webhooks instead of treating the create response as final. `succeeded` and `failed` are final.
    pub status: RetrieveTransfersResponseStatus,
    /// Recorded time of the current transition. Null when the historical transition time is unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub status_changed_at: Option<DateTime<FixedOffset>>,
    /// Recorded success time. Null when unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub succeeded_at: Option<DateTime<FixedOffset>>,
    /// Tracking page for the transfer. Viewing it requires sign-in and access to the transfer.
    #[serde(default)]
    pub tracking_url: String,
}

impl RetrieveTransfersResponse {
    pub fn builder() -> RetrieveTransfersResponseBuilder {
        <RetrieveTransfersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveTransfersResponseBuilder {
    amount: Option<Money>,
    created_at: Option<DateTime<FixedOffset>>,
    destination: Option<RetrieveTransfersResponseDestination>,
    failed_at: Option<DateTime<FixedOffset>>,
    failure_code: Option<String>,
    failure_reason: Option<String>,
    fee: Option<Money>,
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    notes: Option<String>,
    object: Option<RetrieveTransfersResponseObject>,
    origin: Option<RetrieveTransfersResponseOrigin>,
    status: Option<RetrieveTransfersResponseStatus>,
    status_changed_at: Option<DateTime<FixedOffset>>,
    succeeded_at: Option<DateTime<FixedOffset>>,
    tracking_url: Option<String>,
}

impl RetrieveTransfersResponseBuilder {
    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn destination(mut self, value: RetrieveTransfersResponseDestination) -> Self {
        self.destination = Some(value);
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

    pub fn fee(mut self, value: Money) -> Self {
        self.fee = Some(value);
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

    pub fn object(mut self, value: RetrieveTransfersResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn origin(mut self, value: RetrieveTransfersResponseOrigin) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn status(mut self, value: RetrieveTransfersResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_changed_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.status_changed_at = Some(value);
        self
    }

    pub fn succeeded_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.succeeded_at = Some(value);
        self
    }

    pub fn tracking_url(mut self, value: impl Into<String>) -> Self {
        self.tracking_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveTransfersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](RetrieveTransfersResponseBuilder::created_at)
    /// - [`id`](RetrieveTransfersResponseBuilder::id)
    /// - [`metadata`](RetrieveTransfersResponseBuilder::metadata)
    /// - [`object`](RetrieveTransfersResponseBuilder::object)
    /// - [`status`](RetrieveTransfersResponseBuilder::status)
    /// - [`tracking_url`](RetrieveTransfersResponseBuilder::tracking_url)
    pub fn build(self) -> Result<RetrieveTransfersResponse, BuildError> {
        Ok(RetrieveTransfersResponse {
            amount: self.amount,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            destination: self.destination,
            failed_at: self.failed_at,
            failure_code: self.failure_code,
            failure_reason: self.failure_reason,
            fee: self.fee,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            notes: self.notes,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            origin: self.origin,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            status_changed_at: self.status_changed_at,
            succeeded_at: self.succeeded_at,
            tracking_url: self
                .tracking_url
                .ok_or_else(|| BuildError::missing_field("tracking_url"))?,
        })
    }
}
