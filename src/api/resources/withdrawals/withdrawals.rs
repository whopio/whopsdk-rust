use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WithdrawalsClient {
    pub http_client: HttpClient,
}

impl WithdrawalsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of withdrawals for a company, with optional sorting and date filtering.
    ///
    /// Required permissions:
    /// - `payout:withdrawal:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list withdrawals for.
    /// * `created_before` - Only return withdrawals created before this timestamp.
    /// * `created_after` - Only return withdrawals created after this timestamp.
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
    ///         .withdrawals
    ///         .list(
    ///             &WithdrawalsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 after: None,
    ///                 before: None,
    ///                 direction: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WithdrawalsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWithdrawalsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "withdrawals",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .serialize("direction", request.direction.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a withdrawal request for a ledger account
    ///
    /// Required permissions:
    /// - `payout:withdraw_funds`
    /// - `payout:destination:read`
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
    ///         .withdrawals
    ///         .create(
    ///             &CreateWithdrawalsRequest {
    ///                 amount: 6.9,
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 currency: Currencies::Usd,
    ///                 acknowledge_bank_warning: None,
    ///                 idempotency_key: None,
    ///                 payout_method_id: None,
    ///                 platform_covers_fees: None,
    ///                 speed: None,
    ///                 statement_descriptor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateWithdrawalsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Withdrawal, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "withdrawals",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing withdrawal.
    ///
    /// Required permissions:
    /// - `payout:withdrawal:read`
    /// - `payout:destination:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the withdrawal to retrieve.
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
    ///         .withdrawals
    ///         .retrieve(&"wdrl_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Withdrawal, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("withdrawals/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Generates a withdrawal PDF invoice and returns a temporary download URL.
    ///
    /// Required permissions:
    /// - `payout:withdrawal:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the withdrawal to generate a PDF for.
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
    ///         .withdrawals
    ///         .generate_pdf(&"wdrl_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn generate_pdf(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GeneratePdfWithdrawalsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("withdrawals/{}/generate_pdf", id),
                None,
                None,
                options,
            )
            .await
    }
}
