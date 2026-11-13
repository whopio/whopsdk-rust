use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PaymentQuotesClient {
    pub http_client: HttpClient,
}

impl PaymentQuotesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Prices a purchase the way a payment for it will be charged. The body is the `PaymentInput` a payment takes plus where the buyer is, which a seller that collects no tax on the purchase can leave out. The purchase is priced from exactly what you send: no buyer is looked up, so no stored registration or purchase history applies. Pass the quote's `id` as `quote_id` when you create the payment to charge exactly the purchase, promo code, and tax shown here. A quote is priced once and may be consumed by one payment before `expires_at`. Cart entries must name existing variants with `variant_id`.
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
    ///         .payment_quotes
    ///         .create(
    ///             &CreatePaymentQuotesRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 promo_code: None,
    ///                 promo_code_id: None,
    ///                 variant: None,
    ///                 variant_id: None,
    ///                 address: None,
    ///                 ip_address: None,
    ///                 line_items: None,
    ///                 presentment_country: None,
    ///                 presentment_currency: None,
    ///                 shipping_address: None,
    ///                 tax_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePaymentQuotesRequest,
        options: Option<RequestOptions>,
    ) -> Result<PaymentQuote, ApiError> {
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
                "payment_quotes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a payment quote. Use it to check which payment holds the quote, through `payment_id`, and when it expires.
    ///
    /// # Arguments
    ///
    /// * `id` - The payment quote ID, prefixed `pq_`.
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
    ///         .payment_quotes
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PaymentQuote, ApiError> {
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
                &format!("payment_quotes/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
