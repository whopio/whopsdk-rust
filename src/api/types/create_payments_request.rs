pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaymentsRequest {
    /// The account to charge for, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Whether to capture a card payment immediately. Defaults to true. Pass false to place an authorization hold that must be captured in full within five days via the capture endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,
    /// A confirmation token describing a payment method the buyer just supplied. Provide this instead of `member_id` and `payment_method_id`; the buyer is resolved from the token's billing email, or from `email`. The buyer may still have a step to complete — poll the payment's status for what to do next.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_token: Option<String>,
    /// Overrides the buyer email carried on the confirmation token, resolving or creating the user the payment belongs to. Ignored unless `confirmation_token` is provided, and when the token was created by a signed-in buyer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The member to charge, prefixed `mber_`. Required with `payment_method_id` unless `confirmation_token` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Custom metadata to attach to the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// The stored payment method to charge, prefixed `payt_`. It must belong to the member. Required unless `confirmation_token` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
    /// The plan to charge for, prefixed `plan_`. It must belong to the account.
    #[serde(default)]
    pub plan_id: String,
    /// An active promo code to apply, prefixed `promo_`. It must belong to the account and be valid for the plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code_id: Option<String>,
    /// Where the buyer continues after completing an off-site step. An absolute https URL without credentials, at most 2,048 characters. Ignored unless `confirmation_token` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

impl CreatePaymentsRequest {
    pub fn builder() -> CreatePaymentsRequestBuilder {
        <CreatePaymentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestBuilder {
    account_id: Option<String>,
    capture: Option<bool>,
    confirmation_token: Option<String>,
    email: Option<String>,
    member_id: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
    payment_method_id: Option<String>,
    plan_id: Option<String>,
    promo_code_id: Option<String>,
    return_url: Option<String>,
}

impl CreatePaymentsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn capture(mut self, value: bool) -> Self {
        self.capture = Some(value);
        self
    }

    pub fn confirmation_token(mut self, value: impl Into<String>) -> Self {
        self.confirmation_token = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn promo_code_id(mut self, value: impl Into<String>) -> Self {
        self.promo_code_id = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreatePaymentsRequestBuilder::account_id)
    /// - [`plan_id`](CreatePaymentsRequestBuilder::plan_id)
    pub fn build(self) -> Result<CreatePaymentsRequest, BuildError> {
        Ok(CreatePaymentsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            capture: self.capture,
            confirmation_token: self.confirmation_token,
            email: self.email,
            member_id: self.member_id,
            metadata: self.metadata,
            payment_method_id: self.payment_method_id,
            plan_id: self
                .plan_id
                .ok_or_else(|| BuildError::missing_field("plan_id"))?,
            promo_code_id: self.promo_code_id,
            return_url: self.return_url,
        })
    }
}
