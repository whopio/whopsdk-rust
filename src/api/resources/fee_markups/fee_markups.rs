use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FeeMarkupsClient {
    pub http_client: HttpClient,
}

impl FeeMarkupsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of fee markups configured for a company. If the company is a platform account, returns the platform default markups.
    ///
    /// Required permissions:
    /// - `company:update_child_fees`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list fee markups for. Pass a platform account identifier to retrieve platform default markups.
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
    ///         .fee_markups
    ///         .list(
    ///             &FeeMarkupsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 after: None,
    ///                 before: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &FeeMarkupsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFeeMarkupsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "fee_markups",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create or update a fee markup for a company. If a markup for the specified fee type already exists, it will be updated with the new values.
    ///
    /// Required permissions:
    /// - `company:update_child_fees`
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
    ///         .fee_markups
    ///         .create(
    ///             &CreateFeeMarkupsRequest {
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 fee_type: FeeMarkupTypes::CryptoWithdrawalMarkup,
    ///                 fixed_fee_usd: None,
    ///                 metadata: None,
    ///                 notes: None,
    ///                 percentage_fee: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateFeeMarkupsRequest,
        options: Option<RequestOptions>,
    ) -> Result<FeeMarkup, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "fee_markups",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a fee markup configuration for a company. This removes the custom fee override and reverts to the parent company's default fees.
    ///
    /// Required permissions:
    /// - `company:update_child_fees`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the fee markup to delete.
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
    ///     client.fee_markups.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("fee_markups/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
