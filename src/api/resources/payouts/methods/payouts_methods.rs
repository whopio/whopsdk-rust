use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MethodsClient {
    pub http_client: HttpClient,
}

impl MethodsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the bank accounts, wallets, and crypto addresses an account or user can pay out to, newest first.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The owning account ID (a biz_ identifier). Provide this or user_id.
    /// * `user_id` - The owning user ID (a user_ identifier). Provide this or account_id.
    /// * `status` - Optional status filter. `created` means saved but unused, `active` means a payout through it succeeded, `broken` means the last payout failed and the method needs fixing.
    /// * `amount` - Optional payout amount in whole currency units, for example `250.00`. When provided, each method includes a quote with the estimated fee, amount received, and delivery date for that amount.
    /// * `currency` - Currency code of the amount, for example `usd`. Only meaningful with amount or include_limits.
    /// * `include_limits` - When true, the response also carries limits — the live per-speed payout caps the account's payout requests are validated against, in the requested currency. Requires the payout:withdrawal:read scope.
    /// * `first` - Number of payout methods to return from the start of the window.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - Number of payout methods to return from the end of the window.
    /// * `before` - Cursor to fetch the page before (from page_info.start_cursor).
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
    ///         .methods
    ///         .list(
    ///             &PayoutsMethodsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PayoutsMethodsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMethodsResponse, ApiError> {
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
                "payouts/methods",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .serialize("status", request.status.clone())
                    .float("amount", request.amount.clone())
                    .string("currency", request.currency.clone())
                    .bool("include_limits", request.include_limits.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Saves a new place an account or user can pay out to. Sensitive details are vaulted in transit and never stored raw.
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
    ///         .methods
    ///         .create(
    ///             &CreateMethodsRequest {
    ///                 supported_payout_method_id: "podst_xxxxxxxxxxxxxx".to_string(),
    ///                 account_id: None,
    ///                 destination_currency: None,
    ///                 fields: None,
    ///                 is_default: None,
    ///                 nickname: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateMethodsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateMethodsResponse, ApiError> {
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
                "payouts/methods",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a saved payout method so it can no longer receive payouts.
    ///
    /// # Arguments
    ///
    /// * `id` - Payout method ID, prefixed `potk_`.
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
    ///     client.payouts.methods.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteMethodsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("payouts/methods/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Changes the label used to identify a saved payout method or makes it the account's default payout method.
    ///
    /// # Arguments
    ///
    /// * `id` - Payout method ID, prefixed `potk_`.
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
    ///         .methods
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateMethodsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateMethodsRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateMethodsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("payouts/methods/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
