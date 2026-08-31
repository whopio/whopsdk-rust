pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateConfirmationTokensRequest {
    /// The account (biz_) this token is scoped to — the publishable identity.
    #[serde(default)]
    pub account_id: String,
    /// Billing details collected with the method. `email` is always required; cards additionally require `name` and an address with `line1` and `country`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<CreateConfirmationTokensRequestBillingDetails>,
    /// Screen/runtime facts from the buyer's browser (platform, screen dimensions, language, ...) used for authentication ceremonies. Header-derived fields are captured server-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser_info: Option<HashMap<String, serde_json::Value>>,
    /// The collected method: `type` names the payment method, `category` names the payload shape, and the category-keyed object carries the payload. Wallets are the exception: their payload rides the type key (`apple_pay` / `google_pay`). Send exactly the one payload arm the category selects — extra arms are rejected. Redirect-flow methods (category `redirect`, `bank_transfer`, `voucher`, and redirect wallets like `cashapp`) collect nothing and send no payload arm.
    pub payment_method: CreateConfirmationTokensRequestPaymentMethod,
    /// Where redirect flows send the buyer, carried onto the confirm that consumes this token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// The save-consent state your surface displayed when the buyer confirmed. Confirm may vault only if attested here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreateConfirmationTokensRequestSetupFutureUsage>,
}

impl CreateConfirmationTokensRequest {
    pub fn builder() -> CreateConfirmationTokensRequestBuilder {
        <CreateConfirmationTokensRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestBuilder {
    account_id: Option<String>,
    billing_details: Option<CreateConfirmationTokensRequestBillingDetails>,
    browser_info: Option<HashMap<String, serde_json::Value>>,
    payment_method: Option<CreateConfirmationTokensRequestPaymentMethod>,
    return_url: Option<String>,
    setup_future_usage: Option<CreateConfirmationTokensRequestSetupFutureUsage>,
}

impl CreateConfirmationTokensRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn billing_details(mut self, value: CreateConfirmationTokensRequestBillingDetails) -> Self {
        self.billing_details = Some(value);
        self
    }

    pub fn browser_info(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.browser_info = Some(value);
        self
    }

    pub fn payment_method(mut self, value: CreateConfirmationTokensRequestPaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn setup_future_usage(
        mut self,
        value: CreateConfirmationTokensRequestSetupFutureUsage,
    ) -> Self {
        self.setup_future_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateConfirmationTokensRequestBuilder::account_id)
    /// - [`payment_method`](CreateConfirmationTokensRequestBuilder::payment_method)
    pub fn build(self) -> Result<CreateConfirmationTokensRequest, BuildError> {
        Ok(CreateConfirmationTokensRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            billing_details: self.billing_details,
            browser_info: self.browser_info,
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
            return_url: self.return_url,
            setup_future_usage: self.setup_future_usage,
        })
    }
}
