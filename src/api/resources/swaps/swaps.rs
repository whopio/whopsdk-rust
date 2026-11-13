use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SwapsClient {
    pub http_client: HttpClient,
}

impl SwapsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the completed or pending swaps for an account or user — currently only the most recent one.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Business account whose swaps to list, prefixed `biz_`. Provide this or `user_id`.
    /// * `user_id` - The caller's own user ID, prefixed `user_`, to list swaps in their personal account. Provide this or `account_id`.
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
    ///         .swaps
    ///         .list(
    ///             &SwapsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SwapsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSwapsResponse, ApiError> {
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
                "swaps",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Swaps one token for another in an account or user's wallet, or converts between their fiat balances. Crypto swaps finish in the background — retrieve the swap to follow its status.
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
    ///         .swaps
    ///         .create(
    ///             &CreateSwapsRequest {
    ///                 from_token: "usd".to_string(),
    ///                 to_token: "cad".to_string(),
    ///                 account_id: None,
    ///                 amount: None,
    ///                 from_chain: None,
    ///                 slippage_bps: None,
    ///                 to_amount: None,
    ///                 to_chain: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateSwapsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateSwapsResponse, ApiError> {
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
                "swaps",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Previews the price of a swap before you create it. A fiat pair quotes the same rate creating the swap fills at; name the account with `account_id` or `user_id` to quote the rate for that account. No funds move, nothing is saved, and no authentication is required unless you name an account.
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
    ///         .swaps
    ///         .create_quote(
    ///             &CreateQuoteSwapsRequest {
    ///                 amount: "100".to_string(),
    ///                 from_token: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
    ///                 to_token: "0x1b64b9025eebb9a6239575df9ea4b9ac46d4d193".to_string(),
    ///                 account_id: None,
    ///                 from_address: None,
    ///                 from_chain: None,
    ///                 metadata: None,
    ///                 slippage_bps: None,
    ///                 to_address: None,
    ///                 to_chain: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_quote(
        &self,
        request: &CreateQuoteSwapsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateQuoteSwapsResponse, ApiError> {
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
                "swaps/quote",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a swap and its status. Poll it after creating a crypto swap, which finishes in the background.
    ///
    /// # Arguments
    ///
    /// * `id` - Swap ID returned from POST /swaps.
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
    ///     client.swaps.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveSwapsResponse, ApiError> {
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
                &format!("swaps/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
