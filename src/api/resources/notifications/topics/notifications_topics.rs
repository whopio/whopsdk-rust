use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TopicsClient {
    pub http_client: HttpClient,
}

impl TopicsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the platform's visible notification topics — the categories users can set notification preferences on. App-created topics are internal and not returned.
    ///
    /// # Arguments
    ///
    /// * `topic_type` - Only return topics of this scope: `user` (member notifications) or `account_team` (team notifications).
    /// * `first` - The number of topics to return (default 20, max 100).
    /// * `after` - A cursor; returns topics after this position.
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
    ///         .topics
    ///         .list(
    ///             &NotificationsTopicsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &NotificationsTopicsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTopicsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "notifications/topics",
                None,
                QueryBuilder::new()
                    .serialize("topic_type", request.topic_type.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }
}
