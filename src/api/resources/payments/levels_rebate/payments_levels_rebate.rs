use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LevelsRebateClient {
    pub http_client: HttpClient,
}

impl LevelsRebateClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the part of its account's monthly Whop Levels rebate a card payment earned: the processing fee above its level's rate, once the month its fee posted in has closed. The rebate is paid to the account as one sum each month, so the payment's own amounts and fees never change.
    ///
    /// # Arguments
    ///
    /// * `payment_id` - The payment whose rebate to retrieve, prefixed `pay_`.
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
    ///         .payments
    ///         .levels_rebate
    ///         .retrieve(&"payment_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        payment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentLevelsRebate, ApiError> {
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
                Method::GET,
                &format!("payments/{}/levels_rebate", payment_id),
                None,
                None,
                options,
            )
            .await
    }
}
