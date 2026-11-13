pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetupIntent {
    /// The account the payment method is saved for, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The checkout configuration this setup was created through, prefixed `ch_`. Null for a setup created through this API rather than a hosted checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_configuration_id: Option<String>,
    /// The credential a buyer's surface presents to poll this setup and set its return URL — hand it to the elements' `handleNextAction`. Only on setups created through this API, and always null in list responses — retrieve the setup intent for it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// When the setup intent was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Setup intent ID, prefixed `sint_`.
    #[serde(default)]
    pub id: String,
    /// Why the setup ended where it did, or `null` when nothing has failed. Present on `canceled` — a buyer who abandoned carries no code, one refused by the provider does. Dropped once the setup succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_setup_error: Option<SetupLastSetupError>,
    /// The buyer's member record on the account, prefixed `mber_`. Null without the member:basic:read permission, unless the caller is the buyer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Your own key-value data attached when the setup intent was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The method behind this setup shaped for display: a buyer-facing name, the standard icon set, and the card's brand, last four, issuer identification number, expiry, and network transaction id when it was a card. Null until a method was collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<PaymentInstrument>,
    /// The saved payment method, prefixed `payt_`, ready to charge with Create Payment. Null until the setup has `succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// The kind of instrument being saved, for example `card` or `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_type: Option<PaymentMethodTypes>,
    /// Where the buyer lands after completing an off-site step, or `null` to leave them where they are.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// How far the setup has got. **A 201 or 200 means we answered, not that the method was saved — always branch on this.** `requires_action` — the buyer has a step outstanding; hand `client_secret` to the elements or poll Retrieve setup status. `processing` — the processor is deciding. `succeeded` — the method is saved, and only this one means saved. `canceled` — abandoned or refused; see `last_setup_error`.
    pub status: SetupIntentStatus,
    /// 3D Secure behavior for supported on-session card payments. `mandate_challenge` requires a 3DS challenge before payment processing; `mandate_if_required` mandates a challenge only when the payment processor requires it; `frictionless_if_required` uses the regular frictionless 3DS flow. Payments of $1,000 or more use `mandate_if_required` unless `mandate_challenge` is selected. Risk and authentication recovery requirements can override the preference. The setting requested for this setup, or `null` when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<SetupIntentThreeDsLevel>,
    /// True when the buyer completed 3D Secure while saving this payment method.
    #[serde(default)]
    pub three_ds_verified: bool,
    /// When the setup intent was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The user saving the payment method. Null when the buyer is a company rather than a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSummary>,
}

impl SetupIntent {
    pub fn builder() -> SetupIntentBuilder {
        <SetupIntentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentBuilder {
    account_id: Option<String>,
    checkout_configuration_id: Option<String>,
    client_secret: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    last_setup_error: Option<SetupLastSetupError>,
    member_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    payment_instrument: Option<PaymentInstrument>,
    payment_method_id: Option<String>,
    payment_method_type: Option<PaymentMethodTypes>,
    return_url: Option<String>,
    status: Option<SetupIntentStatus>,
    three_ds_level: Option<SetupIntentThreeDsLevel>,
    three_ds_verified: Option<bool>,
    updated_at: Option<String>,
    user: Option<UserSummary>,
}

impl SetupIntentBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn checkout_configuration_id(mut self, value: impl Into<String>) -> Self {
        self.checkout_configuration_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_setup_error(mut self, value: SetupLastSetupError) -> Self {
        self.last_setup_error = Some(value);
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn payment_instrument(mut self, value: PaymentInstrument) -> Self {
        self.payment_instrument = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn payment_method_type(mut self, value: PaymentMethodTypes) -> Self {
        self.payment_method_type = Some(value);
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: SetupIntentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn three_ds_level(mut self, value: SetupIntentThreeDsLevel) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    pub fn three_ds_verified(mut self, value: bool) -> Self {
        self.three_ds_verified = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetupIntent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](SetupIntentBuilder::created_at)
    /// - [`id`](SetupIntentBuilder::id)
    /// - [`status`](SetupIntentBuilder::status)
    /// - [`three_ds_verified`](SetupIntentBuilder::three_ds_verified)
    /// - [`updated_at`](SetupIntentBuilder::updated_at)
    pub fn build(self) -> Result<SetupIntent, BuildError> {
        Ok(SetupIntent {
            account_id: self.account_id,
            checkout_configuration_id: self.checkout_configuration_id,
            client_secret: self.client_secret,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_setup_error: self.last_setup_error,
            member_id: self.member_id,
            metadata: self.metadata,
            payment_instrument: self.payment_instrument,
            payment_method_id: self.payment_method_id,
            payment_method_type: self.payment_method_type,
            return_url: self.return_url,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            three_ds_level: self.three_ds_level,
            three_ds_verified: self
                .three_ds_verified
                .ok_or_else(|| BuildError::missing_field("three_ds_verified"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user,
        })
    }
}
