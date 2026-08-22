use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ForumsClient {
    pub http_client: HttpClient,
}

impl ForumsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of forums within a specific company, with optional filtering by product.
    ///
    /// Required permissions:
    /// - `forum:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list forums for.
    /// * `product_id` - The unique identifier of a product to filter by. When set, only forums connected to this product are returned.
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
    ///         .forums
    ///         .list(
    ///             &ForumsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 product_id: Some("prod_xxxxxxxxxxxxx".to_string()),
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
        request: &ForumsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListForumsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "forums",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .string("product_id", request.product_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the details of an existing forum.
    ///
    /// Required permissions:
    /// - `forum:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the forum or experience to retrieve.
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
    ///     client.forums.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Forum, ApiError> {
        self.http_client
            .execute_request(Method::GET, &format!("forums/{}", id), None, None, options)
            .await
    }

    /// Update moderation and notification settings for a forum, such as who can post, who can comment, and email notification preferences.
    ///
    /// Required permissions:
    /// - `forum:moderate`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the forum to update. Accepts either an experience ID (e.g. 'exp_xxxxx') or a forum ID.
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
    ///         .forums
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateForumsRequest {
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
        request: &UpdateForumsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Forum, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("forums/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
