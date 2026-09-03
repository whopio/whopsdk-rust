use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct MessagesClient {
    pub http_client: HttpClient,
}

impl MessagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of messages within a specific experience chat, DM, or group chat channel, sorted by creation time.
    ///
    /// Required permissions (one of):
    /// - `chat:read`
    /// - `dms:read`
    /// - `support_chat:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `channel_id` - The unique identifier of the channel or experience to list messages for.
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
    ///         .messages
    ///         .list(
    ///             &MessagesListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 channel_id: "channel_id".to_string(),
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
        request: &MessagesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMessagesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "messages",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("channel_id", request.channel_id.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Send a new message in an experience chat, DM, or group chat channel. Supports text content, attachments, polls, and replies.
    ///
    /// Required permissions (one of):
    /// - `chat:message:create`
    /// - `dms:message:manage`
    /// - `livestream:chat:write`
    /// - `support_chat:message:create`
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
    ///         .messages
    ///         .create(
    ///             &CreateMessagesRequest {
    ///                 channel_id: "channel_id".to_string(),
    ///                 content: "content".to_string(),
    ///                 attachments: None,
    ///                 auto_detect_links: None,
    ///                 poll: None,
    ///                 replying_to_message_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Message, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "messages",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing message.
    ///
    /// Required permissions (one of):
    /// - `chat:read`
    /// - `dms:read`
    /// - `livestream:chat:read`
    /// - `support_chat:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the message to retrieve.
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
    ///     client.messages.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Message, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("messages/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently delete a message from an experience chat, DM, or group chat channel. Only the message author or a channel admin can delete a message.
    ///
    /// Required permissions (one of):
    /// - `chat:message:create` and `chat:read`
    /// - `dms:message:manage` and `dms:read`
    /// - `livestream:chat:write` and `livestream:chat:read`
    /// - `support_chat:message:create` and `support_chat:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the message to delete.
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
    ///     client.messages.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("messages/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Edit the content, attachments, or pinned status of an existing message in an experience chat, DM, or group chat channel.
    ///
    /// Required permissions (one of):
    /// - `chat:message:create`
    /// - `dms:message:manage`
    /// - `livestream:chat:write`
    /// - `support_chat:message:create`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the message to update.
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
    ///         .messages
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateMessagesRequest {
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
        request: &UpdateMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Message, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("messages/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
