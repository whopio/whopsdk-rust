use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod methods;
pub use methods::MethodsClient;
pub mod supported_methods;
pub use supported_methods::SupportedMethodsClient;
pub struct PayoutsClient {
    pub http_client: HttpClient,
    pub methods: MethodsClient,
    pub supported_methods: SupportedMethodsClient,
}

impl PayoutsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            methods: MethodsClient::new(config.clone())?,
            supported_methods: SupportedMethodsClient::new(config.clone())?,
        })
    }

    /// Lists an account's or user's payouts, newest first.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The owning account ID (a biz_ identifier). Provide this or user_id.
    /// * `user_id` - The owning user ID (a user_ identifier). Provide this or account_id.
    /// * `currency` - Optional currency code filter, for example `usd`.
    /// * `status` - Filter to payouts whose `status` reads this word, matching exactly what this version displays — `reversed` finds settled payouts the bank later returned. Requires Api-Version-Date 2026-08-21 or later.
    /// * `source` - Filter by how the payout was created. Payouts created before source tracking or through internal tooling carry no source and never match.
    /// * `payout_method_id` - Filter to payouts sent to one saved payout method (a pytk_ identifier). An unknown id matches nothing.
    /// * `created_before` - Only payouts created before this ISO 8601 time (exclusive).
    /// * `created_after` - Only payouts created at or after this ISO 8601 time (inclusive).
    /// * `first` - Number of payouts to return from the start of the window.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - Number of payouts to return from the end of the window.
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
    ///         .list(
    ///             &PayoutsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PayoutsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPayoutsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "payouts",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("currency", request.currency.clone())
                    .serialize("status", request.status.clone())
                    .serialize("source", request.source.clone())
                    .string("payout_method_id", request.payout_method_id.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Sends money from an account or user balance to a saved payout method for that owner.
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
    ///         .create(
    ///             &CreatePayoutsRequestBody::Unknown(serde_json::json!({"key":"value"})),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePayoutsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CreatePayoutsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "payouts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Fetches one payout by its `wdrl_` ID, or by the `cofr_` conversion request ID a stablecoin payout carries as `payout_request_id` — both ids answer with the same payout object.
    ///
    /// # Arguments
    ///
    /// * `id` - Payout ID, prefixed `wdrl_` for a payout returned by `GET /payouts` or `cofr_` for the payout request returned by `POST /payouts`.
    /// * `account_id` - Owning account ID, prefixed `biz_`. Provide exactly one of `account_id` or `user_id`.
    /// * `user_id` - Owning user ID, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
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
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &PayoutsRetrieveQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        request: &PayoutsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RetrievePayoutsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("payouts/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Cancels a payout that is still in review and returns the funds, fees included, to the balance. A payout can be canceled while its status is `in_review`. A `requested` payout is still being prepared (its funds may be converting) and answers 409 until it reaches review; from `processing` on, the money is on its way and the answer is 409 with error type `not_cancelable`. Canceling a payout that is already canceled succeeds and returns it unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - Payout ID, prefixed `wdrl_`, or the `cofr_` payout request ID returned by `POST /payouts` — both cancel the same payout.
    /// * `user_id` - Owning user ID, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
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
    ///         .cancel(
    ///             &"id".to_string(),
    ///             &CancelQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        request: &CancelQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CancelPayoutsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("payouts/{}/cancel", id),
                None,
                QueryBuilder::new()
                    .string("user_id", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
