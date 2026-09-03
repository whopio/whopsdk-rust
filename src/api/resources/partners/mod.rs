use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod businesses;
pub use businesses::BusinessesClient;
pub struct PartnersClient {
    pub http_client: HttpClient,
    pub businesses: BusinessesClient,
}

impl PartnersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            businesses: BusinessesClient::new(config.clone())?,
        })
    }

    /// Enrolls the calling user in the Whop partner program, making their partner businesses eligible for earnings. Idempotent — enrolling again keeps the original enrollment time.
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
    ///     client.partners.create(None).await;
    /// }
    /// ```
    pub async fn create(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<CreatePartnersResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::POST, "partners", None, None, options)
            .await
    }

    /// Ranks referrers by partner business earnings — all-time by default, or over the current day, month, year, or trailing 30 days. Authentication is optional: authenticated callers also get their own standing, anonymous callers get the rankings alone.
    ///
    /// # Arguments
    ///
    /// * `period` - Time window for the rankings. `day`, `month`, and `year` count earnings since the start of the current calendar day, month, or year; `last_30_days` counts earnings over the trailing 30 days; `all_time` ranks lifetime earnings.
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
    ///         .leaderboard(
    ///             &LeaderboardQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn leaderboard(
        &self,
        request: &LeaderboardQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LeaderboardPartnersResponse, ApiError> {
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
                "partners/leaderboard",
                None,
                QueryBuilder::new()
                    .serialize("period", request.period.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Lists the users the caller referred onto Whop (newest first), each with the second-tier earnings the caller has made from that user's businesses.
    ///
    /// # Arguments
    ///
    /// * `has_businesses` - When true, only referred users who brought at least one business onto Whop.
    /// * `has_earning_businesses` - When true, only referred users with at least one business that has generated earnings.
    /// * `first` - Number of referred users to return from the start of the window.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - Number of referred users to return from the end of the window.
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
    ///         .referred_users(
    ///             &ReferredUsersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn referred_users(
        &self,
        request: &ReferredUsersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReferredUsersPartnersResponse, ApiError> {
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
                "partners/referred_users",
                None,
                QueryBuilder::new()
                    .bool("has_businesses", request.has_businesses.clone())
                    .bool(
                        "has_earning_businesses",
                        request.has_earning_businesses.clone(),
                    )
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
