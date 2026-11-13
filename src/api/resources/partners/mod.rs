use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
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
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(base_url, Method::POST, "partners", None, None, options)
            .await
    }

    /// Lists, per income stream, the fee a business the caller refers would be charged, how low the caller may price it, and the commission the caller earns on that stream. Answers for a referral that does not exist yet, so a referral flow can price the deal before the business is created. Requires `partner:fees:update`, since the floors a partner may price down to are only for a credential that can set them.
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
    ///     client.partners.fee_options(None).await;
    /// }
    /// ```
    pub async fn fee_options(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<FeeOptionsPartnersResponse, ApiError> {
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
                "partners/fee_options",
                None,
                None,
                options,
            )
            .await
    }

    /// Ranks referrers by partner business earnings over the chosen `period`, all-time by default. Authentication is optional: authenticated callers also get their own standing, anonymous callers get the rankings alone.
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
                "partners/leaderboard",
                None,
                QueryBuilder::new()
                    .serialize("period", request.period.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Lists the users the caller referred onto Whop, newest first by default, each with the caller's total affiliate earnings from that user across all tiers. Deprecated: this list is retired and no longer appears in the API reference. It keeps responding for existing integrations and has no replacement.
    ///
    /// # Arguments
    ///
    /// * `earning_partner_id` - The selected referring partner's user_ ID. Requires user_id=global; cannot be combined with earning_partner_username.
    /// * `earning_partner_username` - The selected referring partner's exact username. Requires user_id=global; cannot be combined with earning_partner_id.
    /// * `referring_account_id` - List the users this business's team members referred, past members included, instead of the caller's own. Requires company:basic:read on that business, which every team member holds, and cannot be combined with user_id. The owner role reads every team member's referrals; anyone else reads only their own and those of the people under them in the org chart. earning_partner_id or earning_partner_username then narrows to one team member.
    /// * `query` - Search referred users by name or username. In global mode, matches the beginning of usernames only.
    /// * `has_businesses` - When true, only referred users who brought at least one business onto Whop.
    /// * `has_earning_businesses` - When true, only referred users with at least one business that has generated earnings.
    /// * `order` - The field to sort by. created_at uses the user's signup date; earnings_usd uses the current primary referrer's cached total affiliate earnings from that user.
    /// * `direction` - The direction to sort results.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
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
                "partners/referred_users",
                None,
                QueryBuilder::new()
                    .string("earning_partner_id", request.earning_partner_id.clone())
                    .string(
                        "earning_partner_username",
                        request.earning_partner_username.clone(),
                    )
                    .string("referring_account_id", request.referring_account_id.clone())
                    .structured_query("query", request.query.clone())
                    .bool("has_businesses", request.has_businesses.clone())
                    .bool(
                        "has_earning_businesses",
                        request.has_earning_businesses.clone(),
                    )
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

    pub async fn referred_users_paginated(
        &self,
        request: &ReferredUsersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("earning_partner_id", request.earning_partner_id.clone())
            .string(
                "earning_partner_username",
                request.earning_partner_username.clone(),
            )
            .string("referring_account_id", request.referring_account_id.clone())
            .structured_query("query", request.query.clone())
            .bool("has_businesses", request.has_businesses.clone())
            .bool(
                "has_earning_businesses",
                request.has_earning_businesses.clone(),
            )
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
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
                            "partners/referred_users",
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

    /// Retrieves the authenticated user's partner profile: enrollment and verification status, certification completion, active direct business referral count, and default payout rates. Other users' profiles are not accessible. To create and manage referral links, use `/partner_referral_requests`.
    ///
    /// # Arguments
    ///
    /// * `id` - The authenticated partner's user ID, prefixed user_, or me. Other users' profiles are not accessible.
    /// * `account_id` - Return `payout_rates` for this business, prefixed `biz_`, instead of the caller's own: the business's rate rows, else the platform defaults. Requires company:basic:read on that business, which every team member holds. The rest of the profile stays the caller's.
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
    ///         .retrieve(
    ///             &"me".to_string(),
    ///             &PartnersRetrieveQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        request: &PartnersRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Partner, ApiError> {
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
                &format!("partners/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
