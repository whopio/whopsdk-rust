use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
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

    pub async fn list_paginated(
        &self,
        request: &CourseStudentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("before", request.before.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("course_id", request.course_id.clone())
            .string("keyword", request.keyword.clone())
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
                            "course_students",
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
                &format!("course_students/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
