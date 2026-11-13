use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
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

    /// Lists messages in a channel. Existing public message and channel IDs are preserved. For reconnect synchronization, use inclusive updated_after with include_deleted=true; results use ascending change order. Follow the returned cursors and replace cached messages by ID, including when updated_at is unchanged. The returned updated_at is the stored message update time; reaction changes only affect synchronization ordering. Deleted entries contain identity and timestamps with their content redacted. To jump into history, specify channel_id and one around anchor; results use ascending creation-time order. For older messages in chronological order, use before_message_id with last and direction=asc; for newer messages, use after_message_id with first and direction=asc. Do not supply order or direction with updated_after or an around anchor. Keep message-ID filters when continuing with cursors. For around queries, omit the around anchor and continue with direction=asc.
    ///
    /// # Arguments
    ///
    /// * `channel_id` - Filter by public channel ID. An experience ID is also accepted for experience chat.
    /// * `replying_to_message_id` - Return replies to this message.
    /// * `pinned` - Filter by message pin state.
    /// * `include_deleted` - Include redacted deletion entries for cache synchronization.
    /// * `created_after` - Return messages created strictly after this ISO 8601 timestamp.
    /// * `created_before` - Return messages created strictly before this ISO 8601 timestamp.
    /// * `updated_after` - Return messages changed at or after this ISO 8601 timestamp in ascending change order. Cannot be combined with history filters, around anchors, order, or direction.
    /// * `before_message_id` - Filter messages strictly before this message in channel_id by creation time and message ID. Deleted anchors are accepted; missing or other-channel IDs return 404. Cannot be combined with around anchors or updated_after.
    /// * `after_message_id` - Filter messages strictly after this message in channel_id by creation time and message ID. Deleted anchors are accepted; missing or other-channel IDs return 404. Cannot be combined with around anchors or updated_after.
    /// * `around_message_id` - Center the first page around this live message in the selected channel.
    /// * `around_timestamp` - Center the first page around this ISO 8601 timestamp in the selected channel.
    /// * `order` - History sort field. Defaults to created_at. Cannot be supplied with updated_after or around anchors.
    /// * `direction` - History sort direction. Defaults to desc. Cannot be supplied with updated_after or around anchors.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
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
    ///                 channel_id: "channel_id".to_string(),
    ///                 replying_to_message_id: None,
    ///                 pinned: None,
    ///                 include_deleted: None,
    ///                 created_after: None,
    ///                 created_before: None,
    ///                 updated_after: None,
    ///                 before_message_id: None,
    ///                 after_message_id: None,
    ///                 around_message_id: None,
    ///                 around_timestamp: None,
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
        request: &MessagesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMessagesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                "messages",
                None,
                QueryBuilder::new()
                    .string("channel_id", request.channel_id.clone())
                    .string(
                        "replying_to_message_id",
                        request.replying_to_message_id.clone(),
                    )
                    .bool("pinned", request.pinned.clone())
                    .bool("include_deleted", request.include_deleted.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
                    .string("updated_after", request.updated_after.clone())
                    .string("before_message_id", request.before_message_id.clone())
                    .string("after_message_id", request.after_message_id.clone())
                    .string("around_message_id", request.around_message_id.clone())
                    .string("around_timestamp", request.around_timestamp.clone())
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

    pub async fn list_paginated(
        &self,
        request: &MessagesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("channel_id", request.channel_id.clone())
            .string(
                "replying_to_message_id",
                request.replying_to_message_id.clone(),
            )
            .bool("pinned", request.pinned.clone())
            .bool("include_deleted", request.include_deleted.clone())
            .string("created_after", request.created_after.clone())
            .string("created_before", request.created_before.clone())
            .string("updated_after", request.updated_after.clone())
            .string("before_message_id", request.before_message_id.clone())
            .string("after_message_id", request.after_message_id.clone())
            .string("around_message_id", request.around_message_id.clone())
            .string("around_timestamp", request.around_timestamp.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, cursor_value| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();
                if let Some(cursor) = cursor_value {
                    // Add cursor parameter based on pagination configuration
                    query_params.push(("after".to_string(), cursor));
                }
                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "messages",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction using pagination configuration
                    let items: Vec<serde_json::Value> = response
                        .get("data")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let next_cursor: Option<String> = response
                        .get("page_info")
                        .and_then(|v| v.get("end_cursor"))
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    let has_next_page = next_cursor.is_some();

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with no cursor
        )
    }

    /// Sends a Markdown message as the authenticated user or app agent. Channel access, posting restrictions, moderation, and delivery follow the existing chat services. Use Idempotency-Key to safely retry a send. The response and realtime message use the same message ID. Custom author presentation and embeds are read-only.
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
    ///                 channel_id: "feed_xxxxxxxxxxxxxxxxxxxxxx".to_string(),
    ///                 content: "New **message**".to_string(),
    ///                 attachments: None,
    ///                 auto_detect_links: None,
    ///                 gifs: None,
    ///                 id: None,
    ///                 link_previews: None,
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
    ) -> Result<ChatMessage, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                "messages",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a live message from an accessible channel. Use list synchronization to retrieve deletion entries.
    ///
    /// # Arguments
    ///
    /// * `id` - Existing public message ID.
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
    ) -> Result<ChatMessage, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("messages/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a message as its author or a permitted moderator. Deleted messages remain available as redacted entries through list synchronization.
    ///
    /// # Arguments
    ///
    /// * `id` - Existing public message ID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    pub async fn delete(&self, id: &str, options: Option<RequestOptions>) -> Result<(), ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::DELETE,
                &format!("messages/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the author's message. Use the pin and unpin actions to change its pin state. Omitted fields are preserved; attachment and preview collections replace their previous values.
    ///
    /// # Arguments
    ///
    /// * `id` - Existing public message ID.
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
    ) -> Result<ChatMessage, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::PATCH,
                &format!("messages/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pins a message in its channel. Requires the channel's pin permission.
    ///
    /// # Arguments
    ///
    /// * `id` - The message to pin.
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
    ///     client.messages.pin(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn pin(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ChatMessage, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                &format!("messages/{}/pin", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Add your emoji reaction to a message. Reacting again with the same emoji changes nothing.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the message to react to.
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
    ///         .react(
    ///             &"id".to_string(),
    ///             &ReactMessagesRequest {
    ///                 emoji: "❤️".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn react(
        &self,
        id: &str,
        request: &ReactMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChatMessage, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                &format!("messages/{}/react", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// List who reacted to a message, newest first. Optionally filter by emoji.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the message.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `emoji` - Only return reactions with this emoji, as a Unicode emoji or a supported shortcode.
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
    ///         .list_reactions(
    ///             &"id".to_string(),
    ///             &ListReactionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_reactions(
        &self,
        id: &str,
        request: &ListReactionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListReactionsMessagesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("messages/{}/reactions", id),
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .string("emoji", request.emoji.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_reactions_paginated(
        &self,
        id: &str,
        request: &ListReactionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .string("emoji", request.emoji.clone())
            .build();
        let options_clone = options.clone();
        let id_clone = id.to_string();

        AsyncPaginator::new(
            http_client,
            move |client, cursor_value| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();
                if let Some(cursor) = cursor_value {
                    // Add cursor parameter based on pagination configuration
                    query_params.push(("after".to_string(), cursor));
                }
                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block
                let id_for_async = id_clone.clone();

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            &format!("messages/{}/reactions", id_for_async),
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction using pagination configuration
                    let items: Vec<serde_json::Value> = response
                        .get("data")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let next_cursor: Option<String> = response
                        .get("page_info")
                        .and_then(|v| v.get("end_cursor"))
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    let has_next_page = next_cursor.is_some();

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with no cursor
        )
    }

    /// Unpins a message in its channel. Requires the channel's pin permission.
    ///
    /// # Arguments
    ///
    /// * `id` - The message to unpin.
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
    ///     client.messages.unpin(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unpin(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ChatMessage, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                &format!("messages/{}/unpin", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Remove your emoji reaction from a message. Removing a reaction you have not added changes nothing.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the message to remove your reaction from.
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
    ///         .unreact(
    ///             &"id".to_string(),
    ///             &UnreactMessagesRequest {
    ///                 emoji: "❤️".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unreact(
        &self,
        id: &str,
        request: &UnreactMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChatMessage, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                &format!("messages/{}/unreact", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Vote for one option on a message poll. Each user can vote once per poll.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the message containing the poll.
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
    ///         .vote(
    ///             &"id".to_string(),
    ///             &VoteMessagesRequest {
    ///                 poll_option_id: "2".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn vote(
        &self,
        id: &str,
        request: &VoteMessagesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChatMessage, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                &format!("messages/{}/vote", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
