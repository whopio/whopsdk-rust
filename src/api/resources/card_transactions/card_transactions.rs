use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CardTransactionsClient {
    pub http_client: HttpClient,
}

impl CardTransactionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists an account's card transactions, newest first. Defaults to the account the credential belongs to. Covers every card the owner has ever had, including canceled cards and spend that predates a re-application, and team members only see transactions on the cards assigned to them. Pass `transaction_ids` to fetch specific transactions instead of paging for them.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account whose card transactions to list, prefixed `biz_`. Defaults to the credential's account.
    /// * `transaction_ids` - Return only these card transactions, each prefixed `citx_`. Repeat the parameter, or pass one comma-separated value.
    /// * `card_id` - Return only transactions charged to these cards, each prefixed `icrd_`.
    /// * `cardholder_id` - Return only transactions on cards assigned to these users, each prefixed `user_`.
    /// * `status` - Return only transactions with this status.
    /// * `created_after` - Return only transactions authorized at or after this ISO 8601 timestamp.
    /// * `created_before` - Return only transactions authorized at or before this ISO 8601 timestamp.
    /// * `order` - The field to sort by. Defaults to `created_at`.
    /// * `direction` - The sort direction. Defaults to `desc`.
    /// * `first` - The number of card transactions to return.
    /// * `after` - A cursor; returns card transactions after this position.
    /// * `last` - The number of card transactions to return, counting back from the end.
    /// * `before` - A cursor; returns card transactions before this position.
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
    ///         .card_transactions
    ///         .list(
    ///             &CardTransactionsListQueryRequest {
    ///                 transaction_ids: vec![Some("citx_xxxxxxxxxxxxxx".to_string())],
    ///                 card_id: vec![Some("icrd_xxxxxxxxxxxxxx".to_string())],
    ///                 cardholder_id: vec![Some("user_xxxxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 status: None,
    ///                 created_after: None,
    ///                 created_before: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CardTransactionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCardTransactionsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "card_transactions",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string_array("transaction_ids", request.transaction_ids.clone())
                    .string_array("card_id", request.card_id.clone())
                    .string_array("cardholder_id", request.cardholder_id.clone())
                    .serialize("status", request.status.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Fetches a single card transaction by its `citx_` identifier. The owner defaults to the account the credential belongs to.
    ///
    /// # Arguments
    ///
    /// * `id` - The card transaction ID, prefixed `citx_`.
    /// * `account_id` - The account that owns the transaction, prefixed `biz_`. Defaults to the credential's account.
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
    ///         .card_transactions
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &CardTransactionsRetrieveQueryRequest {
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
        request: &CardTransactionsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CardTransaction, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("card_transactions/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
