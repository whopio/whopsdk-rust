use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ChatChannelsClient {
    pub http_client: HttpClient,
}

impl ChatChannelsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of chat channels within a specific company, with optional filtering by product.
    ///
    /// Required permissions:
    /// - `chat:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list chat channels for.
    /// * `product_id` - The unique identifier of a product to filter by. When set, only chat channels connected to this product are returned.
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
    ///         .chat_channels
    ///         .list(
    ///             &ChatChannelsListQueryRequest {
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
        request: &ChatChannelsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListChatChannelsResponse, ApiError> {
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
                "chat_channels",
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

    /// Retrieves the details of an existing chat channel.
    ///
    /// Required permissions:
    /// - `chat:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the chat channel or experience to retrieve.
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
    ///     client.chat_channels.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ChatChannel, ApiError> {
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
                &format!("chat_channels/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update moderation settings for a chat channel, such as who can post, banned words, and media restrictions.
    ///
    /// Required permissions:
    /// - `chat:moderate`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the chat channel to update. Accepts either an experience ID (e.g. 'exp_xxxxx') or a chat channel ID.
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
    ///         .chat_channels
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateChatChannelsRequest {
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
        request: &UpdateChatChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChatChannel, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("chat_channels/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
