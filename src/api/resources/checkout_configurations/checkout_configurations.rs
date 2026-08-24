use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CheckoutConfigurationsClient {
    pub http_client: HttpClient,
}

impl CheckoutConfigurationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists checkout configurations for an account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account ID, prefixed `biz_`.
    /// * `plan_id` - Only return checkout configurations for this plan ID, prefixed `plan_`.
    /// * `created_before` - Only return checkout configurations created before this ISO 8601 timestamp.
    /// * `created_after` - Only return checkout configurations created after this ISO 8601 timestamp.
    /// * `order` - Field used to sort checkout configurations.
    /// * `direction` - Sort direction. Defaults to `desc`.
    /// * `first` - Number of checkout configurations to return.
    /// * `after` - Cursor for the next page of results.
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
    ///         .checkout_configurations
    ///         .list(
    ///             &CheckoutConfigurationsListQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 plan_id: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 first: None,
    ///                 after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CheckoutConfigurationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCheckoutConfigurationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "checkout_configurations",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("plan_id", request.plan_id.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a reusable checkout configuration for an existing or inline plan.
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
    ///         .checkout_configurations
    ///         .create(
    ///             &CreateCheckoutConfigurationsRequest {
    ///                 account_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 plan_id: Some("plan_xxxxxxxxxxxxx".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCheckoutConfigurationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateCheckoutConfigurationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "checkout_configurations",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a checkout configuration by ID. This endpoint is public so a checkout page can load from the configuration URL.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the checkout configuration.
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
    ///         .checkout_configurations
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveCheckoutConfigurationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("checkout_configurations/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a checkout configuration so its checkout URL can no longer be used.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the checkout configuration.
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
    ///         .checkout_configurations
    ///         .delete(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteCheckoutConfigurationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("checkout_configurations/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
