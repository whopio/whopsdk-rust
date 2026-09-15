use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LinksClient {
    pub http_client: HttpClient,
}

impl LinksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the authenticated user's standard referral URL and a page of their balance reward links, newest first. Expired and fully claimed rewards are included by default; deleted rewards are excluded. Filter status to narrow the promotion links. Users do not need to be enrolled to retrieve their links.
    ///
    /// # Arguments
    ///
    /// * `status` - Filter promotion links by availability. Repeat the status parameter for multiple values.
    /// * `first` - Number of promotion links to return from the start of the window.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - Number of promotion links to return from the end of the window.
    /// * `before` - Cursor to fetch the page before (from page_info.start_cursor).
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
    ///         .partners
    ///         .links
    ///         .list(
    ///             &PartnersLinksListQueryRequest {
    ///                 status: vec![],
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
        request: &PartnersLinksListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLinksResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-13".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "partners/links",
                None,
                QueryBuilder::new()
                    .serialize_array("status", request.status.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }
}
