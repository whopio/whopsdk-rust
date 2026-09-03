use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ConfirmationTokensClient {
    pub http_client: HttpClient,
}

impl ConfirmationTokensClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Mints a single-use, short-lived confirmation token from what the buyer entered on your collection surface: the payment method payload, billing details, and attested save consent. Public and rate-limited — the account_id in the body scopes the token but does not authenticate. Confirm it with POST /payments from your server.
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
    ///         .confirmation_tokens
    ///         .create(
    ///             &CreateConfirmationTokensRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 billing_details: Some(CreateConfirmationTokensRequestBillingDetails {
    ///                     address: Some(HashMap::from([
    ///                         ("city".to_string(), serde_json::json!("Austin")),
    ///                         ("country".to_string(), serde_json::json!("US")),
    ///                         ("line1".to_string(), serde_json::json!("123 Main St")),
    ///                         ("postal_code".to_string(), serde_json::json!("78701")),
    ///                     ])),
    ///                     email: "buyer@example.com".to_string(),
    ///                     name: Some("Buyer Name".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 payment_method: CreateConfirmationTokensRequestPaymentMethod {
    ///                     apple_pay: None,
    ///                     balance: None,
    ///                     bank_debit: None,
    ///                     card: Some(CreateConfirmationTokensRequestPaymentMethodCard {
    ///                         brand: Some("visa".to_string()),
    ///                         last4: Some("4242".to_string()),
    ///                         token_intent: Some("bt_ti_123".to_string()),
    ///                         ..Default::default()
    ///                     }),
    ///                     category: CreateConfirmationTokensRequestPaymentMethodCategory::Card,
    ///                     google_pay: None,
    ///                     payer_document: None,
    ///                     saved: None,
    ///                     r#type: Some("card".to_string()),
    ///                 },
    ///                 setup_future_usage: Some(
    ///                     CreateConfirmationTokensRequestSetupFutureUsage::OffSession,
    ///                 ),
    ///                 browser_info: None,
    ///                 return_url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateConfirmationTokensRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConfirmationToken, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "confirmation_tokens",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a token's display-safe preview — never the underlying payment credential. Public and rate-limited: the account_id query param must match the account the token was minted for.
    ///
    /// # Arguments
    ///
    /// * `id` - Confirmation token ID, prefixed `ctok_`.
    /// * `account_id` - The account (biz_) the token was minted for.
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
    ///         .confirmation_tokens
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &ConfirmationTokensRetrieveQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        request: &ConfirmationTokensRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConfirmationToken, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("confirmation_tokens/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
