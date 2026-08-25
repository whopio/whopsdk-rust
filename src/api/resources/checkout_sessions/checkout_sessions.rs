use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CheckoutSessionsClient {
    pub http_client: HttpClient,
}

impl CheckoutSessionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Opens a checkout session. No credentials required. Pass exactly one of `items`, `checkout_configuration`, or `link`. The response includes `client_secret` once; later calls authenticate with it.
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
    ///         .checkout_sessions
    ///         .create(
    ///             &CreateCheckoutSessionsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCheckoutSessionsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CheckoutSession, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "checkout_sessions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
