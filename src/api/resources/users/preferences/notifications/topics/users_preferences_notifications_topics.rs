use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TopicsClient2 {
    pub http_client: HttpClient,
}

impl TopicsClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the authenticated user's topic-scoped notification preferences, plus user-agnostic platform defaults. Each filter matches preferences scoped to its value or not narrowed on that dimension. Per-experience levels are listed separately, by `GET /users/me/preferences/notifications/experiences`.
    ///
    /// # Arguments
    ///
    /// * `channel` - Only return preferences for this delivery channel (or not narrowed to a channel).
    /// * `account_id` - Only return preferences scoped to this account's member notifications (`biz_` tag).
    /// * `team_account_id` - Only return preferences scoped to this account's team notifications (`biz_` tag).
    /// * `experience_id` - Only return preferences scoped to this experience (`exp_` tag).
    /// * `topic_id` - Only return preferences scoped to this notification topic (`topic_` tag).
    /// * `first` - The number of preferences to return.
    /// * `after` - A cursor; returns preferences after this position.
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
    ///         .users
    ///         .preferences
    ///         .notifications
    ///         .topics
    ///         .list(
    ///             &UsersPreferencesNotificationsTopicsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &UsersPreferencesNotificationsTopicsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTopicsResponse2, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "users/me/preferences/notifications/topics",
                None,
                QueryBuilder::new()
                    .serialize("channel", request.channel.clone())
                    .string("account_id", request.account_id.clone())
                    .string("team_account_id", request.team_account_id.clone())
                    .string("experience_id", request.experience_id.clone())
                    .string("topic_id", request.topic_id.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }
}
