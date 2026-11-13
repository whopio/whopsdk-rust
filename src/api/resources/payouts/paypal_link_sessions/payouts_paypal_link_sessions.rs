use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PaypalLinkSessionsClient {
    pub http_client: HttpClient,
}

impl PaypalLinkSessionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Starts a Log in with PayPal session so PayPal itself can confirm the account holder owns a verified PayPal account. Send the holder to `authorize_url`; after they log in, PayPal returns them to Whop, and Whop redirects the browser to your `return_url` with a `paypal_link_token` query parameter to pass to `POST /payouts/methods`. If the login fails or is canceled, `return_url` receives an `error` query parameter (`paypal_login_canceled` or `paypal_login_failed`) instead. The payout method saved this way pays the email on the PayPal account, and its holder name must match the name on the account's payout setup. PayPal login is rolling out per account: until it is enabled for the account this answers `403 forbidden`, and `supports_paypal_login` on `GET /payouts/supported_methods` is false.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use whop_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Whop::new(config).expect("Failed to build client");
    ///     client
    ///         .payouts
    ///         .paypal_link_sessions
    ///         .create(
    ///             &CreatePaypalLinkSessionsRequest {
    ///                 return_url: "https://example.com/settings/payouts".to_string(),
    ///                 supported_payout_method_id: "podst_xxxxxxxxxxxxxx".to_string(),
    ///                 account_id: None,
    ///                 destination_currency: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePaypalLinkSessionsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePaypalLinkSessionsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                "payouts/paypal_link_sessions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
