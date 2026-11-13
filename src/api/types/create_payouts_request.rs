pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePayoutsRequest {
    /// Account to pay out from, prefixed `biz_`. Provide exactly one of `account_id` or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Set to `true` to continue when the destination bank could not confirm the payout method account holder's name, or `false` to have the payout refused in that case so the account holder can correct the name or link their bank first. Omitting the field skips the warning gate — a client that cannot show the warning keeps its pre-gate behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_bank_warning: Option<bool>,
    /// The amount to pay out in the specified currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The currency to pay out. Balances are held per currency and the payout draws only from the balance in this currency, so match the currency the funds arrived in — for example `cad` for an account funded by CAD transfers. When omitted, uses `usd` if that balance can cover a withdrawal, otherwise the account's only other funded currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Key-value data to attach to the payout, echoed on every read and in webhook payloads. At most 50 keys, key names up to 40 characters, string values up to 500 characters. Never store secrets or regulated personal data here — webhook bodies are retained for delivery inspection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Free-form notes to attach to the payout, with a maximum of 255 characters. Omit or pass `null` for no notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// A saved card of the account, prefixed `payt_`, to charge for this payout instead of paying from the balance. Requires a `quote_token` from `POST /payouts/quotes` made with the same card and `amount`. Only for business accounts paying a saved recipient in `usd` at `standard` speed. The payout is returned under the card payment's `pay_` id until it is submitted, then `GET /payouts/{id}` returns the submitted payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// The saved payout method to deliver to (a potk_ identifier).
    #[serde(default)]
    pub payout_method_id: String,
    /// Whether the parent platform covers the payout fee instead of the account being paid out. Omit to use the platform's configured fee coverage policy; pass `false` to opt out of it. `true` is only accepted for accounts that belong to a platform, and requires the platform's policy to cover this payout method's category or a caller authorized to manage the platform's child account fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_covers_fees: Option<bool>,
    /// Required server-signed token from POST /payouts/quotes, matching this payout's owner, method, amount, currency and speed. Submit before expiry. For an uncertain payout, retry with the original token and Idempotency-Key to recover the existing payout.
    #[serde(default)]
    pub quote_token: String,
    /// How fast the funds should arrive. `instant` is only accepted when the account and payout method are eligible; otherwise the payout is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<CreatePayoutsRequestSpeed>,
    /// Payment description sent with the payout, such as an invoice number or what it pays for. Must be 5-80 printable ASCII characters (letters, digits, spaces and punctuation); Whop sends it in upper case. Not all countries and banks show it on the recipient's statement, and it is not the sender name. Set this value when creating the quote; the payout request may omit it but cannot add or change it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// User to pay out from, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreatePayoutsRequest {
    pub fn builder() -> CreatePayoutsRequestBuilder {
        <CreatePayoutsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePayoutsRequestBuilder {
    account_id: Option<String>,
    acknowledge_bank_warning: Option<bool>,
    amount: Option<f64>,
    currency: Option<String>,
    metadata: Option<HashMap<String, String>>,
    notes: Option<String>,
    payment_method_id: Option<String>,
    payout_method_id: Option<String>,
    platform_covers_fees: Option<bool>,
    quote_token: Option<String>,
    speed: Option<CreatePayoutsRequestSpeed>,
    statement_descriptor: Option<String>,
    user_id: Option<String>,
}

impl CreatePayoutsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn acknowledge_bank_warning(mut self, value: bool) -> Self {
        self.acknowledge_bank_warning = Some(value);
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn payout_method_id(mut self, value: impl Into<String>) -> Self {
        self.payout_method_id = Some(value.into());
        self
    }

    pub fn platform_covers_fees(mut self, value: bool) -> Self {
        self.platform_covers_fees = Some(value);
        self
    }

    pub fn quote_token(mut self, value: impl Into<String>) -> Self {
        self.quote_token = Some(value.into());
        self
    }

    pub fn speed(mut self, value: CreatePayoutsRequestSpeed) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn statement_descriptor(mut self, value: impl Into<String>) -> Self {
        self.statement_descriptor = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePayoutsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreatePayoutsRequestBuilder::amount)
    /// - [`payout_method_id`](CreatePayoutsRequestBuilder::payout_method_id)
    /// - [`quote_token`](CreatePayoutsRequestBuilder::quote_token)
    pub fn build(self) -> Result<CreatePayoutsRequest, BuildError> {
        Ok(CreatePayoutsRequest {
            account_id: self.account_id,
            acknowledge_bank_warning: self.acknowledge_bank_warning,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self.currency,
            metadata: self.metadata,
            notes: self.notes,
            payment_method_id: self.payment_method_id,
            payout_method_id: self
                .payout_method_id
                .ok_or_else(|| BuildError::missing_field("payout_method_id"))?,
            platform_covers_fees: self.platform_covers_fees,
            quote_token: self
                .quote_token
                .ok_or_else(|| BuildError::missing_field("quote_token"))?,
            speed: self.speed,
            statement_descriptor: self.statement_descriptor,
            user_id: self.user_id,
        })
    }
}
