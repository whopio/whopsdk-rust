use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod topics;
pub use topics::TopicsClient;
pub struct NotificationsClient {
    pub http_client: HttpClient,
    pub topics: TopicsClient,
}

impl NotificationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            topics: TopicsClient::new(config.clone())?,
        })
    }

    /// Lists the authenticated user's notifications, newest first. Requires a user credential — an account API key has no notification feed. Without filters the feed spans every experience the user belongs to plus the teams they are a member of.
    ///
    /// # Arguments
    ///
    /// * `unread` - Only return notifications created since the user last viewed their source.
    /// * `experience_id` - Only return notifications from this experience (`exp_` tag).
    /// * `account_id` - Only return team notifications for this account (`biz_` tag).
    /// * `mentions` - Only return notifications that mention the user directly.
    /// * `first` - The number of notifications to return (default 20, max 100).
    /// * `after` - A cursor (a notification `id` from a previous page); returns notifications older than it.
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
    ///         .notifications
    ///         .list(
    ///             &NotificationsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &NotificationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListNotificationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "notifications",
                None,
                QueryBuilder::new()
                    .bool("unread", request.unread.clone())
                    .string("experience_id", request.experience_id.clone())
                    .string("account_id", request.account_id.clone())
                    .bool("mentions", request.mentions.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Queues a notification to every user of an experience or to an account's team, processed asynchronously. Every send is attributed to an app: use an app API key, or a credential acting on behalf of an app. Narrow the audience with `user_ids` to send a mention.
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
    ///         .notifications
    ///         .create(
    ///             &CreateNotificationsRequest {
    ///                 content: "Drop off at 4180 Burnet Rd. Plan on two days for the full coating."
    ///                     .to_string(),
    ///                 title: "Your ceramic coating is booked".to_string(),
    ///                 account_id: None,
    ///                 experience_id: None,
    ///                 icon_user_id: None,
    ///                 rest_path: None,
    ///                 subtitle: None,
    ///                 user_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateNotificationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateNotificationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "notifications",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the authenticated user's per-experience unread badge state. Requires a user credential. Returns one row per experience the user belongs to (or per requested experience).
    ///
    /// # Arguments
    ///
    /// * `experience_ids` - Only return badges for these experiences (`exp_` tags).
    /// * `last_fetched_at` - The client's last fetched-at ISO 8601 timestamp, used to partially refresh badges after a websocket message.
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
    ///         .notifications
    ///         .badges(
    ///             &BadgesQueryRequest {
    ///                 experience_ids: vec![Some("exp_xxxxxxxxxxxxxx".to_string())],
    ///                 last_fetched_at: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn badges(
        &self,
        request: &BadgesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BadgesNotificationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "notifications/badges",
                None,
                QueryBuilder::new()
                    .string_array("experience_ids", request.experience_ids.clone())
                    .string("last_fetched_at", request.last_fetched_at.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Marks the authenticated user's notifications as read: one experience's (`experience_id`) or everything (`all: true`) — exactly one of the two. Requires a user credential. Responds with the refreshed badge rows for the affected scope.
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
    ///         .notifications
    ///         .mark_read(
    ///             &MarkReadNotificationsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn mark_read(
        &self,
        request: &MarkReadNotificationsRequest,
        options: Option<RequestOptions>,
    ) -> Result<MarkReadNotificationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "notifications/mark_read",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a single notification by id — either an `id` returned by List Notifications, or the ephemeral id delivered with a push/websocket event. Requires a user credential.
    ///
    /// # Arguments
    ///
    /// * `id` - A notification `id` from List Notifications, or the id delivered with a push/websocket event.
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
    ///     client.notifications.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Notification, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("notifications/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
