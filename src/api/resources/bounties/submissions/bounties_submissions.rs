use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SubmissionsClient {
    pub http_client: HttpClient,
}

impl SubmissionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists a bounty's publicly visible work — submitted, approved, and denied submissions in the reduced public shape. Authentication is optional; a bounty that is not publicly visible returns `404`.
    ///
    /// # Arguments
    ///
    /// * `bounty_id` - The bounty whose public submissions to list (`bnty_` tag).
    /// * `status` - Filter by lifecycle state.
    /// * `created_after` - Only submissions created after this ISO 8601 timestamp.
    /// * `created_before` - Only submissions created before this ISO 8601 timestamp.
    /// * `order` - Sort field.
    /// * `direction` - Sort direction.
    /// * `first` - Number of submissions to return from the start of the window.
    /// * `after` - Cursor to paginate forwards from.
    /// * `last` - Number of submissions to return from the end of the window.
    /// * `before` - Cursor to paginate backwards from.
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
    ///         .bounties
    ///         .submissions
    ///         .list(
    ///             &"bounty_id".to_string(),
    ///             &BountiesSubmissionsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        bounty_id: &str,
        request: &BountiesSubmissionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSubmissionsResponse, ApiError> {
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
                &format!("bounties/{}/submissions", bounty_id),
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
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

    /// Retrieves one of a bounty's publicly visible submissions in the reduced public shape — the read behind a shared proof link, whose submission is usually outside the bounty page's capped preview. Authentication is optional; a bounty that is not publicly visible, and a submission that is not publicly visible work on it, both return `404`.
    ///
    /// # Arguments
    ///
    /// * `bounty_id` - The bounty the submission belongs to (`bnty_` tag).
    /// * `id` - The submission to retrieve (`btys_` tag).
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
    ///         .bounties
    ///         .submissions
    ///         .retrieve(&"bounty_id".to_string(), &"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        bounty_id: &str,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PublicBountySubmission, ApiError> {
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
                &format!("bounties/{}/submissions/{}", bounty_id, id),
                None,
                None,
                options,
            )
            .await
    }
}
