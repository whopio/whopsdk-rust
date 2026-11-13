use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct DepositsClient {
    pub http_client: HttpClient,
}

impl DepositsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the deposit methods for an account or user, including crypto addresses, bank transfer instructions, and, for a business, a hosted deposit page. Bitcoin deposits are converted to USDT on Plasma in the destination's wallet. Business destinations require no authentication. Pass `payment_method_id` to also charge a business's saved payment method for `amount` in USD and add it to the business's balance; that requires authentication with the `payment:charge` scope. The charge has no fees or taxes and does not count as revenue.
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
    ///         .deposits
    ///         .create(
    ///             &CreateDepositsRequest {
    ///                 destination: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 amount: None,
    ///                 metadata: None,
    ///                 origin_bic: None,
    ///                 payment_method_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateDepositsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateDepositsResponse, ApiError> {
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
                "deposits",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Sends a deposit back where it came from. If the available balance doesn't cover it, nothing is sent.
    ///
    /// - **Bank deposit:** pass its `vbt_` ID. The deposit fee is returned, so the balance only has to cover the rest. Returns a `topup_reversal` row.
    /// - **Card deposit:** pass its `pay_` ID. It's refunded to the card, and its fees aren't returned. Returns a `payment_refund` row.
    ///
    /// # Arguments
    ///
    /// * `id` - A bank deposit ID, prefixed `vbt_`, or a card deposit's payment ID, prefixed `pay_`.
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
    ///         .deposits
    ///         .reverse(
    ///             &"vbt_xxxxxxxxxxxxxx".to_string(),
    ///             &ReverseDepositsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reverse(
        &self,
        id: &str,
        request: &ReverseDepositsRequest,
        options: Option<RequestOptions>,
    ) -> Result<LedgerActivity, ApiError> {
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
                &format!("deposits/{}/reverse", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
