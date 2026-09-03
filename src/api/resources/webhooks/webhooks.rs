use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of webhook endpoints configured for an account, ordered by most recently created.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The unique identifier of the account to list webhooks for.
    /// * `app_id` - Only return webhooks attached to this app. Omit to list the account's own webhooks.
    /// * `include_app_webhooks` - Also return webhooks attached to the account's apps, not just the account's own. Cannot be combined with `app_id`.
    /// * `has_failures` - Only return webhooks whose endpoint is currently failing — every delivery since the current failure streak began has been rejected. Clears as soon as a delivery succeeds.
    /// * `first` - The number of webhooks to return (default 20, max 100).
    /// * `after` - A cursor; returns webhooks after this position.
    /// * `last` - The number of webhooks to return from the end of the range.
    /// * `before` - A cursor; returns webhooks before this position.
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
    ///         .webhooks
    ///         .list(
    ///             &WebhooksListQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 app_id: None,
    ///                 include_app_webhooks: None,
    ///                 has_failures: None,
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
        request: &WebhooksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWebhooksResponse, ApiError> {
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
                "webhooks",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("app_id", request.app_id.clone())
                    .bool("include_app_webhooks", request.include_app_webhooks.clone())
                    .bool("has_failures", request.has_failures.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a webhook endpoint that receives event notifications via HTTP POST.
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
    ///         .webhooks
    ///         .create(
    ///             &CreateWebhooksRequest {
    ///                 url: "https://example.com/hooks".to_string(),
    ///                 api_version_date: None,
    ///                 child_resource_events: None,
    ///                 enabled: None,
    ///                 events: None,
    ///                 resource_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<Webhook, ApiError> {
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
                "webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing webhook.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook ID, prefixed `hook_`.
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
    ///     client.webhooks.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Webhook, ApiError> {
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
                &format!("webhooks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Permanently deletes a webhook endpoint. Returns `true` on success, matching the legacy proxy response.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook ID, prefixed `hook_`.
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
    ///     client.webhooks.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteWebhooksResponse, ApiError> {
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
                &format!("webhooks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a webhook endpoint's URL, subscribed events, pinned payload version, or enabled state.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook ID, prefixed `hook_`.
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
    ///         .webhooks
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateWebhooksRequest {
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
        request: &UpdateWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<Webhook, ApiError> {
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
                &format!("webhooks/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a paginated list of delivery attempts for a webhook, ordered by most recent first. Includes the request payload, response body, response code, and timing for each attempt.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook ID, prefixed `hook_`.
    /// * `first` - The number of deliveries to return (default 50, max 100).
    /// * `after` - A cursor; returns deliveries after this position.
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
    ///         .webhooks
    ///         .list_deliveries(
    ///             &"id".to_string(),
    ///             &ListDeliveriesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_deliveries(
        &self,
        id: &str,
        request: &ListDeliveriesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDeliveriesWebhooksResponse, ApiError> {
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
                &format!("webhooks/{}/deliveries", id),
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Re-sends the exact payload of a past delivery to the webhook's current URL and returns the delivery result. By default the replay keeps the original `webhook-id`, so consumers that deduplicate on it can drop events they already processed. Pass `regenerate_id` to re-send under a freshly generated `webhook-id` instead, so a deduplicating consumer processes the replay as a new message. Only available for enabled webhooks on API version v1; deliveries are retained for 30 days.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook ID, prefixed `hook_`.
    /// * `delivery_id` - Delivery ID, prefixed `whdel_`, from the List Deliveries endpoint.
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
    ///         .webhooks
    ///         .replay_delivery(
    ///             &"id".to_string(),
    ///             &"delivery_id".to_string(),
    ///             &ReplayDeliveryWebhooksRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn replay_delivery(
        &self,
        id: &str,
        delivery_id: &str,
        request: &ReplayDeliveryWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReplayDeliveryWebhooksResponse, ApiError> {
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
                &format!("webhooks/{}/deliveries/{}/replay", id, delivery_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Re-sends the webhook's past deliveries within a time window, optionally limited to specific events or to messages whose most recent delivery attempt failed. Fire and forget: nothing about the replay is stored, and each re-send appears as a new entry in the webhook's delivery log. Each matching message is re-sent once, by default with its original `webhook-id`, so consumers that deduplicate are unaffected; pass `regenerate_ids` to re-send under freshly generated ids instead. Only available for enabled webhooks on API version v1; deliveries are retained for 30 days.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook ID, prefixed `hook_`.
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
    ///         .webhooks
    ///         .replay(
    ///             &"id".to_string(),
    ///             &ReplayWebhooksRequest {
    ///                 sent_after: "2026-01-01T12:00:00.000Z".to_string(),
    ///                 events: None,
    ///                 failed_only: None,
    ///                 regenerate_ids: None,
    ///                 sent_before: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn replay(
        &self,
        id: &str,
        request: &ReplayWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReplayWebhooksResponse, ApiError> {
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
                &format!("webhooks/{}/replay", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Sends a sample payload for the given event to the webhook's URL and returns the delivery result.
    ///
    /// # Arguments
    ///
    /// * `id` - Webhook ID, prefixed `hook_`.
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
    ///         .webhooks
    ///         .test(
    ///             &"id".to_string(),
    ///             &TestWebhooksRequest {
    ///                 event: "payment.succeeded".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn test(
        &self,
        id: &str,
        request: &TestWebhooksRequest,
        options: Option<RequestOptions>,
    ) -> Result<TestWebhooksResponse, ApiError> {
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
                &format!("webhooks/{}/test", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a paginated list of delivery attempts for a webhook, ordered by most recent first. Includes the request payload, response body, response code, and timing for each attempt.
    ///
    /// Required permissions:
    /// - `developer:manage_webhook`
    ///
    /// # Arguments
    ///
    /// * `webhook_id` - The unique identifier of the webhook to list deliveries for.
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
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
    ///         .webhooks
    ///         .deliveries_webhook(
    ///             &"webhook_id".to_string(),
    ///             &DeliveriesWebhookQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn deliveries_webhook(
        &self,
        webhook_id: &str,
        request: &DeliveriesWebhookQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeliveriesWebhookResponse, ApiError> {
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
                &format!("webhooks/{}/deliveries", webhook_id),
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .build(),
                options,
            )
            .await
    }
}
