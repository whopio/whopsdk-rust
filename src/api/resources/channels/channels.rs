use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct ChannelsClient {
    pub http_client: HttpClient,
}

impl ChannelsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// List channels you can access.
    ///
    /// # Arguments
    ///
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `include_total_count` - Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
    /// * `kind` - Channel kinds to return. Combine `private` and `support`, or pass `community` alone. `private` includes one-to-one and group direct messages.
    /// * `support_role` - Which support channels to return. `customer` returns support chats you opened, including when `account_id` is set. `staff` returns the support inbox of the account in `account_id`, and requires permission to read that account's support chats. Applies only to `support` channels.
    /// * `account_id` - Filter channels by their owning account, prefixed `biz_`. Applies to private, community and support channels.
    /// * `query` - Return only channels whose name contains this text. Private and support channels also match a member's username or name. `%` and `_` match literally.
    /// * `status` - Return only support channels with this resolution. Requires `kind` to be `support` alone.
    /// * `awaiting_reply` - `true` returns unresolved support channels whose last message came from the customer; `false` returns the other support channels. Other channel kinds are returned either way.
    /// * `unread` - `true` returns channels with messages you have not read; `false` returns the rest. Requires authenticating as a user.
    /// * `pinned` - `true` returns private and support channels you have pinned; `false` returns those you have not. Requires authenticating as a user.
    /// * `viewed` - `true` returns community channels you have opened; `false` returns those you never have. Requires authenticating as a user.
    /// * `requested` - `true` returns your pending direct-message requests; `false` returns your accepted private channels. Requires `private` in `kind` and authenticating as a user. With `support` also in `kind`, `false` keeps support channels and `true` returns only pending requests.
    /// * `order` - The field to sort by. Defaults to `last_viewed_at` for community channels and `last_message_at` for private and support channels. Ties sort in a stable order. Cannot be supplied with updated_after.
    /// * `include_members` - Include the current members of private and support channels, including you, with their public profiles.
    /// * `include_last_message` - Include the most recent message you can see, with its author.
    /// * `direction` - The sort direction. Defaults to `desc`. Cannot be supplied with updated_after.
    /// * `include_deleted` - Include redacted entries for deleted channels and your removed memberships. Use deleted_at to remove these channels from your cache.
    /// * `updated_after` - Return channels where the channel or your membership changed at or after this ISO 8601 timestamp. Cannot be more than one hour old, including on subsequent pages. For older changes, reload channels without updated_after. Results use ascending synchronized_at order. Cannot be combined with order, direction, query, status, awaiting_reply, unread, pinned, viewed, requested, created_before, or created_after.
    /// * `created_before` - Return only channels created before this ISO 8601 timestamp.
    /// * `created_after` - Return only channels created after this ISO 8601 timestamp.
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
    ///         .channels
    ///         .list(
    ///             &ChannelsListQueryRequest {
    ///                 kind: vec![Some(ListChannelsRequestKindItem::Private)],
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 include_total_count: None,
    ///                 support_role: None,
    ///                 account_id: None,
    ///                 query: None,
    ///                 status: None,
    ///                 awaiting_reply: None,
    ///                 unread: None,
    ///                 pinned: None,
    ///                 viewed: None,
    ///                 requested: None,
    ///                 order: None,
    ///                 include_members: None,
    ///                 include_last_message: None,
    ///                 direction: None,
    ///                 include_deleted: None,
    ///                 updated_after: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ChannelsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListChannelsResponse, ApiError> {
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
                "channels",
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .bool("include_total_count", request.include_total_count.clone())
                    .serialize_array("kind", request.kind.clone())
                    .serialize("support_role", request.support_role.clone())
                    .string("account_id", request.account_id.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("status", request.status.clone())
                    .bool("awaiting_reply", request.awaiting_reply.clone())
                    .bool("unread", request.unread.clone())
                    .bool("pinned", request.pinned.clone())
                    .bool("viewed", request.viewed.clone())
                    .bool("requested", request.requested.clone())
                    .serialize("order", request.order.clone())
                    .bool("include_members", request.include_members.clone())
                    .bool("include_last_message", request.include_last_message.clone())
                    .serialize("direction", request.direction.clone())
                    .bool("include_deleted", request.include_deleted.clone())
                    .string("updated_after", request.updated_after.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &ChannelsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .bool("include_total_count", request.include_total_count.clone())
            .serialize_array("kind", request.kind.clone())
            .serialize("support_role", request.support_role.clone())
            .string("account_id", request.account_id.clone())
            .structured_query("query", request.query.clone())
            .serialize("status", request.status.clone())
            .bool("awaiting_reply", request.awaiting_reply.clone())
            .bool("unread", request.unread.clone())
            .bool("pinned", request.pinned.clone())
            .bool("viewed", request.viewed.clone())
            .bool("requested", request.requested.clone())
            .serialize("order", request.order.clone())
            .bool("include_members", request.include_members.clone())
            .bool("include_last_message", request.include_last_message.clone())
            .serialize("direction", request.direction.clone())
            .bool("include_deleted", request.include_deleted.clone())
            .string("updated_after", request.updated_after.clone())
            .string("created_before", request.created_before.clone())
            .string("created_after", request.created_after.clone())
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
                            "channels",
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

    /// Create a private, community or support channel. Returns an existing private or support channel unchanged with `200`, or a new channel with `201`.
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
    ///         .channels
    ///         .create(
    ///             &CreateChannelsRequest {
    ///                 kind: CreateChannelsRequestKind::Private,
    ///                 account_id: None,
    ///                 customer_user_id: None,
    ///                 icon: None,
    ///                 name: None,
    ///                 notifications_enabled: None,
    ///                 participant_user_ids: None,
    ///                 product_id: None,
    ///                 sender_user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Channel, ApiError> {
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
                "channels",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieve a channel.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///     client.channels.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Channel, ApiError> {
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
                &format!("channels/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a channel. Deleting a support channel requires `support_chat:create` on the account. Community channels delete their backing experience and accept account API keys with the `experience:delete` permission. Private and support channels require a user identity with channel management permission; account API keys cannot act as a channel participant.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///     client.channels.delete(&"id".to_string(), None).await;
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
                &format!("channels/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a channel. Community icon changes update the experience logo and require `experience:update`. Group private channel icon changes set the channel's own icon and require a group member. One-to-one private and support channels reject icon changes. Support rename and notification changes require `support_chat:create`. Private and support writes require a user identity with channel management permission. Account API keys can update community channels only.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///         .channels
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateChannelsRequest {
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
        request: &UpdateChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Channel, ApiError> {
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
                &format!("channels/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Accept a direct-message request.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///         .channels
    ///         .accept(
    ///             &"id".to_string(),
    ///             &AcceptChannelsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn accept(
        &self,
        id: &str,
        request: &AcceptChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChannelMember, ApiError> {
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
                &format!("channels/{}/accept", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Decline a DM request by hiding the channel from your inbox.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///         .channels
    ///         .hide(
    ///             &"id".to_string(),
    ///             &HideChannelsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn hide(
        &self,
        id: &str,
        request: &HideChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChannelMember, ApiError> {
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
                &format!("channels/{}/hide", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark a channel as read and optionally record viewed messages.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///         .channels
    ///         .mark_read(
    ///             &"id".to_string(),
    ///             &MarkReadChannelsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn mark_read(
        &self,
        id: &str,
        request: &MarkReadChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChannelMember, ApiError> {
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
                &format!("channels/{}/mark_read", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark a channel as unread for the authenticated user. Experience channels are unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///         .channels
    ///         .mark_unread(
    ///             &"id".to_string(),
    ///             &MarkUnreadChannelsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn mark_unread(
        &self,
        id: &str,
        request: &MarkUnreadChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChannelMember, ApiError> {
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
                &format!("channels/{}/mark_unread", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pin a private or support channel for yourself. Omit position to append a new pin or keep an existing pin in place. An explicit zero-based position inserts or moves the pin and shifts the rest. At most 12 channels can be pinned.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel ID, prefixed `feed_`.
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
    ///     client
    ///         .channels
    ///         .pin(
    ///             &"id".to_string(),
    ///             &PinChannelsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pin(
        &self,
        id: &str,
        request: &PinChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
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
                &format!("channels/{}/pin", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Reopen a resolved support channel.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///         .channels
    ///         .reopen(
    ///             &"id".to_string(),
    ///             &ReopenChannelsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reopen(
        &self,
        id: &str,
        request: &ReopenChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Channel, ApiError> {
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
                &format!("channels/{}/reopen", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Resolve a support channel.
    ///
    /// # Arguments
    ///
    /// * `id` - Public resource ID.
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
    ///         .channels
    ///         .resolve(
    ///             &"id".to_string(),
    ///             &ResolveChannelsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resolve(
        &self,
        id: &str,
        request: &ResolveChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Channel, ApiError> {
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
                &format!("channels/{}/resolve", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Remove a channel from your pins and compact the remaining order. Your discarded memberships and inaccessible channels can be unpinned. Does not return channel data or grant read access.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel ID, prefixed `feed_`.
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
    ///     client.channels.unpin(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unpin(&self, id: &str, options: Option<RequestOptions>) -> Result<(), ApiError> {
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
                &format!("channels/{}/unpin", id),
                None,
                None,
                options,
            )
            .await
    }
}
