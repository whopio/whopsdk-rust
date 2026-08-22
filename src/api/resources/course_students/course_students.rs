use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CourseStudentsClient {
    pub http_client: HttpClient,
}

impl CourseStudentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of students enrolled in a course, with optional name filtering.
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
    /// * `course_id` - The unique identifier of the course to list enrolled students for.
    /// * `keyword` - A search term to filter students by name or username.
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
    ///         .course_students
    ///         .list(
    ///             &CourseStudentsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 course_id: "cors_xxxxxxxxxxxxx".to_string(),
    ///                 after: None,
    ///                 before: None,
    ///                 keyword: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CourseStudentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCourseStudentsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "course_students",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("course_id", request.course_id.clone())
                    .string("keyword", request.keyword.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the details of an existing course student.
    ///
    /// Required permissions:
    /// - `courses:read`
    /// - `course_analytics:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the course student record to retrieve.
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
    ///         .course_students
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CourseStudent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("course_students/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
