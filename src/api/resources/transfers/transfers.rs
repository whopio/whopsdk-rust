use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TransfersClient {
    pub http_client: HttpClient,
}

impl TransfersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists an account's transfers.
    ///
    /// # Arguments
    ///
    /// * `origin_id` - Filter to transfers sent from this account. Provide this or destination_id.
    /// * `destination_id` - Filter to transfers received by this account. Provide this or origin_id.
    /// * `order` - Sort column. Defaults to created_at.
    /// * `direction` - Sort direction. Defaults to desc.
    /// * `created_before` - Only transfers created strictly before this ISO 8601 timestamp.
    /// * `created_after` - Only transfers created strictly after this ISO 8601 timestamp.
    /// * `first` - Number of transfers to return from the start of the window.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - Number of transfers to return from the end of the window.
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
    ///         .transfers
    ///         .list(
    ///             &TransfersListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &TransfersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTransfersResponse, ApiError> {
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
                "transfers",
                None,
                QueryBuilder::new()
                    .string("origin_id", request.origin_id.clone())
                    .string("destination_id", request.destination_id.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Moves money between accounts, or into a claim link anyone with the URL can redeem.
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
    ///         .transfers
    ///         .create(
    ///             &CreateTransfersRequest {
    ///                 amount: 25.0,
    ///                 origin_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 currency: None,
    ///                 destination_id: None,
    ///                 expires_at: None,
    ///                 idempotence_key: None,
    ///                 metadata: None,
    ///                 notes: None,
    ///                 redeemable_count: None,
    ///                 r#type: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateTransfersRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateTransfersResponse, ApiError> {
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
                "transfers",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the people and accounts you can send money to.
    ///
    /// # Arguments
    ///
    /// * `origin_id` - The account sending the money: a company account ID (`biz_`), or a user ID (`user_`) for that user's own personal balance.
    /// * `query` - Search anyone on Whop by name or username, plus your own accounts by name or ID. An exact business ID (`biz_`) returns that business first. Omit it to get the team around the balance, the people you follow, and your own accounts. The list is the same whether the balance belongs to a company or to you. Searching from a `biz_` origin additionally requires the member:basic:read scope. A credential scoped to a single company is the exception to the search itself: it only ever sees that company's own people. Complete email addresses return no matches.
    /// * `first` - Number of recipients per page. Search queries preserve the dashboard's 20-result maximum.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
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
    ///         .transfers
    ///         .list_recipients(
    ///             &ListRecipientsQueryRequest {
    ///                 origin_id: "origin_id".to_string(),
    ///                 query: None,
    ///                 first: None,
    ///                 after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_recipients(
        &self,
        request: &ListRecipientsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListRecipientsTransfersResponse, ApiError> {
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
                "transfers/recipients",
                None,
                QueryBuilder::new()
                    .string("origin_id", request.origin_id.clone())
                    .structured_query("query", request.query.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a single transfer.
    ///
    /// # Arguments
    ///
    /// * `id` - The transfer ID.
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
    ///     client.transfers.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveTransfersResponse, ApiError> {
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
                &format!("transfers/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
