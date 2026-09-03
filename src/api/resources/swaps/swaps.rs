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

    /// Retrieve the account's completed or pending swaps — currently just the latest one.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Business or user account ID (biz_* / user_*).
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
    ///                 account_id: "account_id".to_string(),
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
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "swaps",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Swaps one token for another from the account's wallet, or converts between fiat currencies in the account's ledger at the mid-market rate. Crypto swaps finish in the background — check the swap for its status.
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
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 from_token: "usd".to_string(),
    ///                 to_token: "cad".to_string(),
    ///                 amount: None,
    ///                 from_chain: None,
    ///                 slippage_bps: None,
    ///                 to_amount: None,
    ///                 to_chain: None,
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
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "swaps",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Previews the price of a swap. Fiat pairs quote the in-ledger mid-market conversion — the same rate creating the swap fills at. No funds move and nothing is saved.
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
    ///                 from_address: None,
    ///                 from_chain: None,
    ///                 metadata: None,
    ///                 slippage_bps: None,
    ///                 to_address: None,
    ///                 to_chain: None,
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
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "swaps/quote",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a single swap and its status.
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
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::GET, &format!("swaps/{}", id), None, None, options)
            .await
    }
}
