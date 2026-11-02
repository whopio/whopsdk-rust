pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostPayoutMethodCreatedPayloadData {
    /// Masked identifier for the destination, such as the last four digits of a bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_reference: Option<String>,
    /// Lifecycle trust state: `checking` (verification still running), `verified` (bank confirmed ownership or a payout already completed to it), `no_data` (verification unavailable or bank returned no ownership data), `warning` (bank could not confirm the destination's owner), `broken` (payouts failed with a permanent account error), `null` (never checked).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_verification_state: Option<PostPayoutMethodCreatedPayloadDataBankVerificationState>,
    /// When the payout method was added.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Currency payouts are delivered in for this method.
    #[serde(default)]
    pub destination_currency: String,
    /// Estimated arrival times before an amount-specific quote is requested. Null when the method is not currently eligible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_arrival: Option<PostPayoutMethodCreatedPayloadDataEstimatedArrival>,
    /// Configured fee terms for this payout method. Null when the method is not currently eligible. An amount-specific quote remains authoritative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_structure: Option<PostPayoutMethodCreatedPayloadDataFeeStructure>,
    /// Payout method ID.
    #[serde(default)]
    pub id: String,
    /// Name of the bank or institution receiving payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_name: Option<String>,
    /// Whether this method is a copy of one saved on another of the payer's accounts.
    #[serde(default)]
    pub is_clone: bool,
    /// Whether this is the default payout method for the account.
    #[serde(default)]
    pub is_default: bool,
    /// When the most recent completed payout was delivered to this method, as an ISO 8601 timestamp. `null` when nothing has been paid out to it yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_paid_out_at: Option<DateTime<FixedOffset>>,
    /// Whether the payer added this method by signing in to their bank rather than typing account details.
    #[serde(default)]
    pub linked_via_plaid: bool,
    /// Whether the bank sign-in behind this method has expired and must be redone before it counts as linked.
    #[serde(default)]
    pub needs_plaid_reconnect: bool,
    /// User-defined label for the payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub object: PostPayoutMethodCreatedPayloadDataObject,
    /// Display name of the payout rail, such as `ACH Bank Deposit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_name: Option<String>,
    /// Fee and delivery estimate for paying out the requested amount through this method. Null unless an amount was provided, or when the estimate is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<PostPayoutMethodCreatedPayloadDataQuote>,
    /// Lifecycle status: `created` means saved but unused, `active` means a payout succeeded through it, `broken` means a payout failure disabled it; a later successful payout returns it to `active`.
    pub status: PostPayoutMethodCreatedPayloadDataStatus,
    /// Machine-readable code for why the method is `broken` — the newest disabling failure recorded through it, whether a payout error or a pre-payout rejection. `null` unless the method is broken, or when it was disabled without a recorded failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    /// The supported payout method this saved method was created from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_payout_method: Option<PostPayoutMethodCreatedPayloadDataSupportedPayoutMethod>,
    /// Why this method is unavailable: `destination_retired` means the payout provider stopped offering the destination. Whop may automatically remap an eligible method that was not linked through Plaid to a compatible replacement; otherwise, the account owner must re-add it. `null` means no unavailability reason is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<PostPayoutMethodCreatedPayloadDataUnavailableReason>,
}

impl PostPayoutMethodCreatedPayloadData {
    pub fn builder() -> PostPayoutMethodCreatedPayloadDataBuilder {
        <PostPayoutMethodCreatedPayloadDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutMethodCreatedPayloadDataBuilder {
    account_reference: Option<String>,
    bank_verification_state: Option<PostPayoutMethodCreatedPayloadDataBankVerificationState>,
    created_at: Option<DateTime<FixedOffset>>,
    destination_currency: Option<String>,
    estimated_arrival: Option<PostPayoutMethodCreatedPayloadDataEstimatedArrival>,
    fee_structure: Option<PostPayoutMethodCreatedPayloadDataFeeStructure>,
    id: Option<String>,
    institution_name: Option<String>,
    is_clone: Option<bool>,
    is_default: Option<bool>,
    last_paid_out_at: Option<DateTime<FixedOffset>>,
    linked_via_plaid: Option<bool>,
    needs_plaid_reconnect: Option<bool>,
    nickname: Option<String>,
    object: Option<PostPayoutMethodCreatedPayloadDataObject>,
    payer_name: Option<String>,
    quote: Option<PostPayoutMethodCreatedPayloadDataQuote>,
    status: Option<PostPayoutMethodCreatedPayloadDataStatus>,
    status_reason: Option<String>,
    supported_payout_method: Option<PostPayoutMethodCreatedPayloadDataSupportedPayoutMethod>,
    unavailable_reason: Option<PostPayoutMethodCreatedPayloadDataUnavailableReason>,
}

impl PostPayoutMethodCreatedPayloadDataBuilder {
    pub fn account_reference(mut self, value: impl Into<String>) -> Self {
        self.account_reference = Some(value.into());
        self
    }

    pub fn bank_verification_state(
        mut self,
        value: PostPayoutMethodCreatedPayloadDataBankVerificationState,
    ) -> Self {
        self.bank_verification_state = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn destination_currency(mut self, value: impl Into<String>) -> Self {
        self.destination_currency = Some(value.into());
        self
    }

    pub fn estimated_arrival(
        mut self,
        value: PostPayoutMethodCreatedPayloadDataEstimatedArrival,
    ) -> Self {
        self.estimated_arrival = Some(value);
        self
    }

    pub fn fee_structure(mut self, value: PostPayoutMethodCreatedPayloadDataFeeStructure) -> Self {
        self.fee_structure = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn institution_name(mut self, value: impl Into<String>) -> Self {
        self.institution_name = Some(value.into());
        self
    }

    pub fn is_clone(mut self, value: bool) -> Self {
        self.is_clone = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    pub fn last_paid_out_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_paid_out_at = Some(value);
        self
    }

    pub fn linked_via_plaid(mut self, value: bool) -> Self {
        self.linked_via_plaid = Some(value);
        self
    }

    pub fn needs_plaid_reconnect(mut self, value: bool) -> Self {
        self.needs_plaid_reconnect = Some(value);
        self
    }

    pub fn nickname(mut self, value: impl Into<String>) -> Self {
        self.nickname = Some(value.into());
        self
    }

    pub fn object(mut self, value: PostPayoutMethodCreatedPayloadDataObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn payer_name(mut self, value: impl Into<String>) -> Self {
        self.payer_name = Some(value.into());
        self
    }

    pub fn quote(mut self, value: PostPayoutMethodCreatedPayloadDataQuote) -> Self {
        self.quote = Some(value);
        self
    }

    pub fn status(mut self, value: PostPayoutMethodCreatedPayloadDataStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_reason(mut self, value: impl Into<String>) -> Self {
        self.status_reason = Some(value.into());
        self
    }

    pub fn supported_payout_method(
        mut self,
        value: PostPayoutMethodCreatedPayloadDataSupportedPayoutMethod,
    ) -> Self {
        self.supported_payout_method = Some(value);
        self
    }

    pub fn unavailable_reason(
        mut self,
        value: PostPayoutMethodCreatedPayloadDataUnavailableReason,
    ) -> Self {
        self.unavailable_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutMethodCreatedPayloadData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PostPayoutMethodCreatedPayloadDataBuilder::created_at)
    /// - [`destination_currency`](PostPayoutMethodCreatedPayloadDataBuilder::destination_currency)
    /// - [`id`](PostPayoutMethodCreatedPayloadDataBuilder::id)
    /// - [`is_clone`](PostPayoutMethodCreatedPayloadDataBuilder::is_clone)
    /// - [`is_default`](PostPayoutMethodCreatedPayloadDataBuilder::is_default)
    /// - [`linked_via_plaid`](PostPayoutMethodCreatedPayloadDataBuilder::linked_via_plaid)
    /// - [`needs_plaid_reconnect`](PostPayoutMethodCreatedPayloadDataBuilder::needs_plaid_reconnect)
    /// - [`object`](PostPayoutMethodCreatedPayloadDataBuilder::object)
    /// - [`status`](PostPayoutMethodCreatedPayloadDataBuilder::status)
    pub fn build(self) -> Result<PostPayoutMethodCreatedPayloadData, BuildError> {
        Ok(PostPayoutMethodCreatedPayloadData {
            account_reference: self.account_reference,
            bank_verification_state: self.bank_verification_state,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            destination_currency: self
                .destination_currency
                .ok_or_else(|| BuildError::missing_field("destination_currency"))?,
            estimated_arrival: self.estimated_arrival,
            fee_structure: self.fee_structure,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            institution_name: self.institution_name,
            is_clone: self
                .is_clone
                .ok_or_else(|| BuildError::missing_field("is_clone"))?,
            is_default: self
                .is_default
                .ok_or_else(|| BuildError::missing_field("is_default"))?,
            last_paid_out_at: self.last_paid_out_at,
            linked_via_plaid: self
                .linked_via_plaid
                .ok_or_else(|| BuildError::missing_field("linked_via_plaid"))?,
            needs_plaid_reconnect: self
                .needs_plaid_reconnect
                .ok_or_else(|| BuildError::missing_field("needs_plaid_reconnect"))?,
            nickname: self.nickname,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            payer_name: self.payer_name,
            quote: self.quote,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            status_reason: self.status_reason,
            supported_payout_method: self.supported_payout_method,
            unavailable_reason: self.unavailable_reason,
        })
    }
}
