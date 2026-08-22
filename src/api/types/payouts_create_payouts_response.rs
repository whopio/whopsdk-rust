pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePayoutsResponse {
    /// The payout amount in whole currency units, as a decimal string.
    #[serde(default)]
    pub amount: String,
    /// When the payout was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Payout currency.
    #[serde(default)]
    pub currency: String,
    /// The amount delivered in the destination currency, as a decimal string. Null until the payout settles; appears on the payout in GET /payouts once assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_amount: Option<String>,
    /// Currency the funds are delivered in, taken from the payout method. On a stablecoin payout it follows the settlement payout minted alongside it — the `GET /payouts` row carrying this payout's id as `payout_request_id` — and is `null` only when no settlement payout exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_currency: Option<String>,
    /// Estimated time the funds become available in the destination account. Null until the payout settles.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub estimated_arrival: Option<DateTime<FixedOffset>>,
    /// Exchange rate from the payout currency to the destination currency. Null until the payout settles; appears on the payout in GET /payouts once assigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub exchange_rate: Option<f64>,
    /// Why the payout ended without paying, or why it reversed after settlement. Present on failed, canceled, denied, and reversed payouts; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<CreatePayoutsResponseFailure>,
    /// The fee charged for the payout, in the payout currency, as a decimal string.
    #[serde(default)]
    pub fee_amount: String,
    /// Who bore the payout fee: the account itself, or its parent platform.
    pub fee_paid_by: CreatePayoutsResponseFeePaidBy,
    /// Payout ID, prefixed `wdrl_` — the id POST returns is the id GET /payouts lists. Conversion requests created before this version keep answering under their `cofr_` id.
    #[serde(default)]
    pub id: String,
    /// Whop's markup on the provider fee, in the payout currency, as a decimal string. `"0.0"` when none applies.
    #[serde(default)]
    pub markup_fee: String,
    /// Key-value data attached at creation and echoed on every read. At most 50 keys, key names up to 40 characters, string values up to 500 characters.
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    /// The planned net for the destination, in the payout currency: amount minus fee_amount minus markup_fee when fee_paid_by is `self`; equal to amount when the platform covers the fees. A payout that ends denied, canceled, or failed delivered nothing — most keep the planned figure and `failure` says where the funds are, but a canceled stablecoin payout can report the settled outcome instead: `amount` carries what stayed in the balance, fees are zero because none were charged, and `net_amount` is 0 because nothing was delivered.
    #[serde(default)]
    pub net_amount: String,
    /// Free-form notes attached by the payout creator, or `null` when none were provided. Maximum 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub object: CreatePayoutsResponseObject,
    /// Name of the entity processing the payout. Null until the payout settles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
    /// The saved payout method used. Requires payout:destination:read; null without it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_method: Option<CreatePayoutsResponsePayoutMethod>,
    /// For a stablecoin payout, the id of the conversion request that funds it, prefixed `cofr_`; `null` on fiat payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_request_id: Option<String>,
    /// How the payout was created. `automatic` means a scheduled auto-payout; `null` on payouts created before source tracking or through internal tooling.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<CreatePayoutsResponseSource>,
    /// Payout delivery speed.
    pub speed: CreatePayoutsResponseSpeed,
    /// Current payout status, in the same vocabulary as GET /payouts.
    pub status: CreatePayoutsResponseStatus,
    /// The finest machine phase under `status` — for example `awaiting_provider_acceptance` vs `in_transit` under `processing`, or the stablecoin conversion phase under `requested`. Informational vocabulary: values can be added without a version bump; `status` is the versioned contract.
    #[serde(default)]
    pub status_detail: String,
    /// ACH trace number the recipient's bank can use to locate this payout. Always `null` here — it is assigned when the payout is submitted to the bank, and appears on the payout in GET /payouts once it has been sent; payouts not sent over ACH never get one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_code: Option<String>,
}

impl CreatePayoutsResponse {
    pub fn builder() -> CreatePayoutsResponseBuilder {
        <CreatePayoutsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePayoutsResponseBuilder {
    amount: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<String>,
    destination_amount: Option<String>,
    destination_currency: Option<String>,
    estimated_arrival: Option<DateTime<FixedOffset>>,
    exchange_rate: Option<f64>,
    failure: Option<CreatePayoutsResponseFailure>,
    fee_amount: Option<String>,
    fee_paid_by: Option<CreatePayoutsResponseFeePaidBy>,
    id: Option<String>,
    markup_fee: Option<String>,
    metadata: Option<HashMap<String, String>>,
    net_amount: Option<String>,
    notes: Option<String>,
    object: Option<CreatePayoutsResponseObject>,
    payer_name: Option<String>,
    payout_method: Option<CreatePayoutsResponsePayoutMethod>,
    payout_request_id: Option<String>,
    source: Option<CreatePayoutsResponseSource>,
    speed: Option<CreatePayoutsResponseSpeed>,
    status: Option<CreatePayoutsResponseStatus>,
    status_detail: Option<String>,
    trace_code: Option<String>,
}

impl CreatePayoutsResponseBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn destination_amount(mut self, value: impl Into<String>) -> Self {
        self.destination_amount = Some(value.into());
        self
    }

    pub fn destination_currency(mut self, value: impl Into<String>) -> Self {
        self.destination_currency = Some(value.into());
        self
    }

    pub fn estimated_arrival(mut self, value: DateTime<FixedOffset>) -> Self {
        self.estimated_arrival = Some(value);
        self
    }

    pub fn exchange_rate(mut self, value: f64) -> Self {
        self.exchange_rate = Some(value);
        self
    }

    pub fn failure(mut self, value: CreatePayoutsResponseFailure) -> Self {
        self.failure = Some(value);
        self
    }

    pub fn fee_amount(mut self, value: impl Into<String>) -> Self {
        self.fee_amount = Some(value.into());
        self
    }

    pub fn fee_paid_by(mut self, value: CreatePayoutsResponseFeePaidBy) -> Self {
        self.fee_paid_by = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn markup_fee(mut self, value: impl Into<String>) -> Self {
        self.markup_fee = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn net_amount(mut self, value: impl Into<String>) -> Self {
        self.net_amount = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn object(mut self, value: CreatePayoutsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn payer_name(mut self, value: impl Into<String>) -> Self {
        self.payer_name = Some(value.into());
        self
    }

    pub fn payout_method(mut self, value: CreatePayoutsResponsePayoutMethod) -> Self {
        self.payout_method = Some(value);
        self
    }

    pub fn payout_request_id(mut self, value: impl Into<String>) -> Self {
        self.payout_request_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: CreatePayoutsResponseSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn speed(mut self, value: CreatePayoutsResponseSpeed) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn status(mut self, value: CreatePayoutsResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_detail(mut self, value: impl Into<String>) -> Self {
        self.status_detail = Some(value.into());
        self
    }

    pub fn trace_code(mut self, value: impl Into<String>) -> Self {
        self.trace_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePayoutsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreatePayoutsResponseBuilder::amount)
    /// - [`created_at`](CreatePayoutsResponseBuilder::created_at)
    /// - [`currency`](CreatePayoutsResponseBuilder::currency)
    /// - [`fee_amount`](CreatePayoutsResponseBuilder::fee_amount)
    /// - [`fee_paid_by`](CreatePayoutsResponseBuilder::fee_paid_by)
    /// - [`id`](CreatePayoutsResponseBuilder::id)
    /// - [`markup_fee`](CreatePayoutsResponseBuilder::markup_fee)
    /// - [`metadata`](CreatePayoutsResponseBuilder::metadata)
    /// - [`net_amount`](CreatePayoutsResponseBuilder::net_amount)
    /// - [`object`](CreatePayoutsResponseBuilder::object)
    /// - [`speed`](CreatePayoutsResponseBuilder::speed)
    /// - [`status`](CreatePayoutsResponseBuilder::status)
    /// - [`status_detail`](CreatePayoutsResponseBuilder::status_detail)
    pub fn build(self) -> Result<CreatePayoutsResponse, BuildError> {
        Ok(CreatePayoutsResponse {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            destination_amount: self.destination_amount,
            destination_currency: self.destination_currency,
            estimated_arrival: self.estimated_arrival,
            exchange_rate: self.exchange_rate,
            failure: self.failure,
            fee_amount: self
                .fee_amount
                .ok_or_else(|| BuildError::missing_field("fee_amount"))?,
            fee_paid_by: self
                .fee_paid_by
                .ok_or_else(|| BuildError::missing_field("fee_paid_by"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            markup_fee: self
                .markup_fee
                .ok_or_else(|| BuildError::missing_field("markup_fee"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            net_amount: self
                .net_amount
                .ok_or_else(|| BuildError::missing_field("net_amount"))?,
            notes: self.notes,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payer_name: self.payer_name,
            payout_method: self.payout_method,
            payout_request_id: self.payout_request_id,
            source: self.source,
            speed: self
                .speed
                .ok_or_else(|| BuildError::missing_field("speed"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            status_detail: self
                .status_detail
                .ok_or_else(|| BuildError::missing_field("status_detail"))?,
            trace_code: self.trace_code,
        })
    }
}
