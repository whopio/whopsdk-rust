use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CourseLessonInteractionsClient {
    pub http_client: HttpClient,
}

impl CourseLessonInteractionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of lesson interactions, filtered by lesson, course, user, or completion status.
    ///
    /// Required permissions:
    /// - `courses:read`
    /// - `course_analytics:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `user_id` - The unique identifier of the user to filter lesson interactions for.
    /// * `lesson_id` - The unique identifier of the lesson to filter interactions for.
    /// * `course_id` - The unique identifier of the course to filter interactions for.
    /// * `completed` - Whether to filter for completed or in-progress lesson interactions.
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
    ///         .course_lesson_interactions
    ///         .list(
    ///             &CourseLessonInteractionsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 user_id: Some("user_xxxxxxxxxxxxx".to_string()),
    ///                 lesson_id: Some("lesn_xxxxxxxxxxxxx".to_string()),
    ///                 course_id: Some("cors_xxxxxxxxxxxxx".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CourseLessonInteractionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCourseLessonInteractionsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "course_lesson_interactions",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("user_id", request.user_id.clone())
                    .string("lesson_id", request.lesson_id.clone())
                    .string("course_id", request.course_id.clone())
                    .bool("completed", request.completed.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the details of an existing course lesson interaction.
    ///
    /// Required permissions:
    /// - `courses:read`
    /// - `course_analytics:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the lesson interaction to retrieve.
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
    ///         .course_lesson_interactions
    ///         .retrieve(&"crsli_xxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CourseLessonInteraction, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("course_lesson_interactions/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
