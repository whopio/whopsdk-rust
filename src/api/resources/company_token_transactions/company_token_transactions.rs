use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CompanyTokenTransactionsClient {
    pub http_client: HttpClient,
}

impl CompanyTokenTransactionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of token transactions for a user or company, depending on the authenticated actor, with optional filtering by user and transaction type.
    ///
    /// Required permissions:
    /// - `company_token_transaction:read`
    /// - `member:basic:read`
    /// - `company:basic:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `user_id` - Filter transactions to only those involving this specific user.
    /// * `account_id` - The unique identifier of the company to list token transactions for.
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
    ///         .company_token_transactions
    ///         .list(
    ///             &CompanyTokenTransactionsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 user_id: Some("user_xxxxxxxxxxxxx".to_string()),
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 after: None,
    ///                 before: None,
    ///                 transaction_type: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CompanyTokenTransactionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCompanyTokenTransactionsResponse, ApiError> {
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
                "company_token_transactions",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("user_id", request.user_id.clone())
                    .serialize("transaction_type", request.transaction_type.clone())
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a token transaction to add, subtract, or transfer tokens for a member within a company.
    ///
    /// Required permissions:
    /// - `company_token_transaction:create`
    /// - `member:basic:read`
    /// - `company:basic:read`
    ///
    /// # Arguments
    ///
    /// * `request` - Parameters for CreateCompanyTokenTransaction
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
    ///         .company_token_transactions
    ///         .create(
    ///             &CreateCompanyTokenTransactionsRequestBody::Transfer {
    ///                 data: CreateCompanyTokenTransactionsRequestBodyTransfer {
    ///                     account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                     amount: 6.9,
    ///                     destination_user_id: "destination_user_id".to_string(),
    ///                     user_id: "user_xxxxxxxxxxxxx".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCompanyTokenTransactionsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<CompanyTokenTransaction, ApiError> {
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
                "company_token_transactions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing company token transaction.
    ///
    /// Required permissions:
    /// - `company_token_transaction:read`
    /// - `member:basic:read`
    /// - `company:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the token transaction to retrieve.
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
    ///         .company_token_transactions
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CompanyTokenTransaction, ApiError> {
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
                &format!("company_token_transactions/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
