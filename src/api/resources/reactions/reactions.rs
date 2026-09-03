use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ReactionsClient {
    pub http_client: HttpClient,
}

impl ReactionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of emoji reactions on a specific message or forum post, sorted by most recent.
    ///
    /// Required permissions (one of):
    /// - `chat:read`
    /// - `dms:read`
    /// - `forum:read`
    /// - `livestream:chat:read`
    /// - `support_chat:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `resource_id` - The unique identifier of the message or forum post to list reactions for.
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
    ///         .reactions
    ///         .list(
    ///             &ReactionsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 resource_id: "resource_id".to_string(),
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
        request: &ReactionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListReactionsResponse, ApiError> {
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
                "reactions",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("resource_id", request.resource_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add an emoji reaction or poll vote to a message or forum post. In forums, the reaction is always a like.
    ///
    /// Required permissions (one of):
    /// - `chat:read`
    /// - `dms:read`
    /// - `forum:read`
    /// - `livestream:chat:read`
    /// - `support_chat:read`
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
    ///         .reactions
    ///         .create(
    ///             &CreateReactionsRequest {
    ///                 resource_id: "resource_id".to_string(),
    ///                 emoji: None,
    ///                 poll_option_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateReactionsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Reaction, ApiError> {
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
                "reactions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing reaction.
    ///
    /// Required permissions (one of):
    /// - `chat:read`
    /// - `dms:read`
    /// - `forum:read`
    /// - `livestream:chat:read`
    /// - `support_chat:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the reaction to retrieve.
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
    ///         .reactions
    ///         .retrieve(&"reac_xxxxxxxxxxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Reaction, ApiError> {
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
                &format!("reactions/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Remove an emoji reaction from a message or forum post. Only the reaction author or a channel admin can remove a reaction.
    ///
    /// Required permissions (one of):
    /// - `chat:read`
    /// - `dms:read`
    /// - `forum:read`
    /// - `livestream:chat:read`
    /// - `support_chat:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the reaction to remove, or the identifier of the message or forum post to remove a reaction from. When passing a message or post ID, you must also provide the emoji argument.
    /// * `emoji` - The emoji to remove, in shortcode or unicode format. For example, ':heart:' or a unicode emoji. Required when the id refers to a message or post instead of a reaction.
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
    ///         .reactions
    ///         .delete(
    ///             &"reac_xxxxxxxxxxxxxxxxxxxxxx".to_string(),
    ///             &ReactionsDeleteQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        request: &ReactionsDeleteQueryRequest,
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
                &format!("reactions/{}", id),
                None,
                QueryBuilder::new()
                    .string("emoji", request.emoji.clone())
                    .build(),
                options,
            )
            .await
    }
}
