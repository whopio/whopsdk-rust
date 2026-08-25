use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ForumPostsClient {
    pub http_client: HttpClient,
}

impl ForumPostsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of forum posts within a specific experience, with optional filtering by parent post or pinned status.
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
    /// * `experience_id` - The unique identifier of the experience to list forum posts for.
    /// * `include_bounty_anchors` - Whether to include top-level bounty discussion anchors as rich forum items.
    /// * `parent_id` - The unique identifier of a parent post to list comments for. When set, returns replies to that post.
    /// * `pinned` - Whether to filter for only pinned posts. Set to true to return only pinned posts.
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
    ///         .forum_posts
    ///         .list(
    ///             &ForumPostsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 experience_id: "exp_xxxxxxxxxxxxxx".to_string(),
    ///                 after: None,
    ///                 before: None,
    ///                 include_bounty_anchors: None,
    ///                 parent_id: None,
    ///                 pinned: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ForumPostsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListForumPostsResponse, ApiError> {
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
                "forum_posts",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("experience_id", request.experience_id.clone())
                    .bool(
                        "include_bounty_anchors",
                        request.include_bounty_anchors.clone(),
                    )
                    .string("parent_id", request.parent_id.clone())
                    .bool("pinned", request.pinned.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create a new forum post or comment within an experience. Supports text content, attachments, polls, paywalling, and pinning. Pass experience_id 'public' with a company_id to post to a company's public forum.
    ///
    /// Required permissions:
    /// - `forum:post:create`
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
    ///         .forum_posts
    ///         .create(
    ///             &CreateForumPostsRequest {
    ///                 experience_id: "exp_xxxxxxxxxxxxxx".to_string(),
    ///                 attachments: None,
    ///                 company_id: None,
    ///                 content: None,
    ///                 is_mention: None,
    ///                 parent_id: None,
    ///                 paywall_amount: None,
    ///                 paywall_currency: None,
    ///                 pinned: None,
    ///                 poll: None,
    ///                 rich_content: None,
    ///                 title: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateForumPostsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ForumPost, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "forum_posts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing forum post.
    ///
    /// Required permissions:
    /// - `forum:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the forum post to retrieve.
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
    ///     client.forum_posts.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ForumPost, ApiError> {
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
                &format!("forum_posts/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Edit the content, attachments, pinned status, or visibility of an existing forum post or comment.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the forum post to update.
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
    ///         .forum_posts
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateForumPostsRequest {
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
        request: &UpdateForumPostsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ForumPost, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("forum_posts/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
