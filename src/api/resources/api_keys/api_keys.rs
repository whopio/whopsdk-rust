use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient {
    pub http_client: HttpClient,
}

impl ApiKeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the API keys of an account or app, newest first. Responses never include the full secret — only its obfuscated form.
    ///
    /// # Arguments
    ///
    /// * `resource_id` - The account (`biz_`) or app (`app_`) tag to list API keys for.
    /// * `resource_type` - The type of resource that owns the API keys.
    /// * `created_before` - Only return API keys created before this ISO 8601 timestamp.
    /// * `created_after` - Only return API keys created after this ISO 8601 timestamp.
    /// * `first` - The number of API keys to return (default 20, max 100).
    /// * `after` - A cursor; returns API keys after this position.
    /// * `last` - The number of API keys to return from the end of the range.
    /// * `before` - A cursor; returns API keys before this position.
    /// * `order` - The field to sort API keys by.
    /// * `direction` - Sort direction.
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
    ///         .api_keys
    ///         .list(
    ///             &APIKeysListQueryRequest {
    ///                 resource_id: "resource_id".to_string(),
    ///                 resource_type: ListAPIKeysRequestResourceType::Account,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 order: None,
    ///                 direction: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ApiKeysListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListApiKeysResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "api_keys",
                None,
                QueryBuilder::new()
                    .string("resource_id", request.resource_id.clone())
                    .serialize("resource_type", Some(request.resource_type.clone()))
                    .serialize("created_before", request.created_before.clone())
                    .serialize("created_after", request.created_after.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an API key for an account or app. The response is the only place the full `secret_key` is returned — store it immediately. Requires a user session; API keys cannot manage API keys.
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
    ///         .api_keys
    ///         .create(
    ///             &CreateAPIKeysRequest {
    ///                 name: "Shine Time Booking (production)".to_string(),
    ///                 permissions: CreateAPIKeysRequestPermissions {
    ///                     ..Default::default()
    ///                 },
    ///                 resource_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 resource_type: CreateAPIKeysRequestResourceType::Account,
    ///                 api_version_date: None,
    ///                 expires_at: None,
    ///                 ip_allowlist: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateApiKeysRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKey, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "api_keys",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the catalog of permission actions that can be granted to users, apps, and API keys — the source for the dashboard's permission pickers. Small and returned in full on one page.
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
    ///     client.api_keys.list_permissions(None).await;
    /// }
    /// ```
    pub async fn list_permissions(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ListPermissionsApiKeysResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "api_keys/permissions", None, None, options)
            .await
    }

    /// Retrieves an API key with its effective permission grants. The full secret is never returned — rotate the key if it was lost.
    ///
    /// # Arguments
    ///
    /// * `id` - API key ID, prefixed `apik_`.
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
    ///     client.api_keys.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApiKey, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("api_keys/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently revokes an API key; requests using its secret stop authenticating immediately. Default and agent-backend keys cannot be deleted.
    ///
    /// # Arguments
    ///
    /// * `id` - API key ID, prefixed `apik_`.
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
    ///     client.api_keys.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteApiKeysResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("api_keys/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an API key's name, permissions, API version, expiration, or IP allowlist. Fields that are omitted keep their current value; default keys cannot be modified.
    ///
    /// # Arguments
    ///
    /// * `id` - API key ID, prefixed `apik_`.
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
    ///         .api_keys
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAPIKeysRequest {
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
        request: &UpdateApiKeysRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKey, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("api_keys/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Rotates the API key's secret, invalidating the previous secret immediately. The response is the only place the new `secret_key` is returned.
    ///
    /// # Arguments
    ///
    /// * `id` - API key ID, prefixed `apik_`.
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
    ///     client.api_keys.rotate(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn rotate(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApiKey, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("api_keys/{}/rotate", id),
                None,
                None,
                options,
            )
            .await
    }
}
