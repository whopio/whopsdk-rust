pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaypalLinkSessionsRequest {
    /// The account that will own the PayPal payout method, prefixed `biz_`. Provide this or `user_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Currency the PayPal payout method will deliver payouts in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_currency: Option<String>,
    /// Where to send the holder once the PayPal login finishes. Must be an `https` URL whose host is a domain added to the account, or to its platform, with `POST /domains` and `verification: {}`; the domain doesn't need to finish verifying. Whop's own pages and `https://localhost` are also accepted; they're the only options for a `user_id` session. It receives `paypal_link_token` on success or `error` on failure.
    #[serde(default)]
    pub return_url: String,
    /// The PayPal supported payout method to connect (a `podst_` identifier whose `supports_paypal_login` is true).
    #[serde(default)]
    pub supported_payout_method_id: String,
    /// The user who will own the PayPal payout method, prefixed `user_`. Provide this or `account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreatePaypalLinkSessionsRequest {
    pub fn builder() -> CreatePaypalLinkSessionsRequestBuilder {
        <CreatePaypalLinkSessionsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaypalLinkSessionsRequestBuilder {
    account_id: Option<String>,
    destination_currency: Option<String>,
    return_url: Option<String>,
    supported_payout_method_id: Option<String>,
    user_id: Option<String>,
}

impl CreatePaypalLinkSessionsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn destination_currency(mut self, value: impl Into<String>) -> Self {
        self.destination_currency = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn supported_payout_method_id(mut self, value: impl Into<String>) -> Self {
        self.supported_payout_method_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaypalLinkSessionsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`return_url`](CreatePaypalLinkSessionsRequestBuilder::return_url)
    /// - [`supported_payout_method_id`](CreatePaypalLinkSessionsRequestBuilder::supported_payout_method_id)
    pub fn build(self) -> Result<CreatePaypalLinkSessionsRequest, BuildError> {
        Ok(CreatePaypalLinkSessionsRequest {
            account_id: self.account_id,
            destination_currency: self.destination_currency,
            return_url: self
                .return_url
                .ok_or_else(|| BuildError::missing_field("return_url"))?,
            supported_payout_method_id: self
                .supported_payout_method_id
                .ok_or_else(|| BuildError::missing_field("supported_payout_method_id"))?,
            user_id: self.user_id,
        })
    }
}
