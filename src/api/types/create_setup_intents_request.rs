pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateSetupIntentsRequest {
    /// The account to save the payment method for, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// A confirmation token describing a payment method the buyer just supplied, collected by the payment elements in setup mode. Provide this or `payment_method_id`, not both. The buyer is resolved from the token's billing email, or from `email`, and may still have a step to complete — poll Retrieve setup status for what to do next.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_token: Option<String>,
    /// The currency the saved payment method will be used with, as a lowercase ISO 4217 code. Controls which currency-specific payment methods are available. Defaults to `usd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Overrides the buyer email carried on the confirmation token, resolving or creating the user the method belongs to. Ignored unless `confirmation_token` is provided, and when the token was created by a signed-in buyer or the caller is the buyer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Custom metadata to attach to the setup intent. Returned on the setup intent and its webhooks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// Who the saved payment method belongs to. Pass `account_id` to save it on the account itself, which needs `payment:charge` on the account. Pass your own user ID, prefixed `user_`, to save it on the authenticated user. Omit it to save it on the buyer: the caller when it is the buyer, otherwise the user the confirmation token names. Not available with `payment_method_id`, which keeps its current owner, or with `purpose`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// An existing payment method to re-verify and save, prefixed `payt_`. Provide this or `confirmation_token`, not both. Not available to a buyer credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// What the saved payment method will pay for. Set to `ads_billing` when saving a card to pay for Whop Ads on `account_id`: the card is verified by Whop Ads, the merchant that charges it, which helps minimize security declines on ad payments, and it requires `ad_campaign:create` on `account_id`. Omit it for any other payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CreateSetupIntentsRequestPurpose>,
    /// Where the buyer continues after completing an off-site step. An absolute https URL without credentials, at most 2,048 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// 3D Secure behavior for supported on-session card payments. `mandate_challenge` requires a 3DS challenge before payment processing; `mandate_if_required` mandates a challenge only when the payment processor requires it; `frictionless_if_required` uses the regular frictionless 3DS flow. Payments of $1,000 or more use `mandate_if_required` unless `mandate_challenge` is selected. Risk and authentication recovery requirements can override the preference. Applies to this setup only, and only while the buyer is present: ignored with `payment_method_id`, which re-verifies off session. Not available with `purpose: ads_billing`. Defaults to `frictionless_if_required`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds_level: Option<CreateSetupIntentsRequestThreeDsLevel>,
}

impl CreateSetupIntentsRequest {
    pub fn builder() -> CreateSetupIntentsRequestBuilder {
        <CreateSetupIntentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSetupIntentsRequestBuilder {
    account_id: Option<String>,
    confirmation_token: Option<String>,
    currency: Option<String>,
    email: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
    owner_id: Option<String>,
    payment_method_id: Option<String>,
    purpose: Option<CreateSetupIntentsRequestPurpose>,
    return_url: Option<String>,
    three_ds_level: Option<CreateSetupIntentsRequestThreeDsLevel>,
}

impl CreateSetupIntentsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn confirmation_token(mut self, value: impl Into<String>) -> Self {
        self.confirmation_token = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: CreateSetupIntentsRequestPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn three_ds_level(mut self, value: CreateSetupIntentsRequestThreeDsLevel) -> Self {
        self.three_ds_level = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSetupIntentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateSetupIntentsRequestBuilder::account_id)
    pub fn build(self) -> Result<CreateSetupIntentsRequest, BuildError> {
        Ok(CreateSetupIntentsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            confirmation_token: self.confirmation_token,
            currency: self.currency,
            email: self.email,
            metadata: self.metadata,
            owner_id: self.owner_id,
            payment_method_id: self.payment_method_id,
            purpose: self.purpose,
            return_url: self.return_url,
            three_ds_level: self.three_ds_level,
        })
    }
}
