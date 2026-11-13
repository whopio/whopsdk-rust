use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod earnings;
pub use earnings::EarningsClient;
pub struct BusinessesClient {
    pub http_client: HttpClient,
    pub earnings: EarningsClient,
}

impl BusinessesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            earnings: EarningsClient::new(config.clone())?,
        })
    }

    /// Lists the businesses the authenticated user referred onto Whop, most recent first.
    ///
    /// # Arguments
    ///
    /// * `referring_account_id` - List the referrals this business's team members made, past members included, instead of the authenticated user's own. Requires company:basic:read on that business, which every team member holds, and cannot be combined with user_id. The owner role reads every referral; anyone else reads only the referrals made by themselves and by the people under them in the org chart.
    /// * `advance_status` - Filter the authenticated partner's CPA offers by status, or with referring_account_id, the CPA terms of the reps the caller manages. Use offered for every state after qualification.
    /// * `status` - Filter by referral status.
    /// * `has_earnings` - When true, only businesses with pending or completed earnings paid to the caller.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `include_total_count` - Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
    /// * `order` - The field to sort partner businesses by.
    /// * `direction` - Sort direction.
    /// * `created_before` - Only return partner businesses created before this timestamp.
    /// * `created_after` - Only return partner businesses created after this timestamp.
    /// * `first_sale_before` - Only return partner businesses whose first paid sale above zero happened before this timestamp.
    /// * `first_sale_after` - Only return partner businesses whose first paid sale above zero happened after this timestamp. Combined with first_sale_before, this is the net new set: businesses that processed their first $1 inside the window.
    /// * `referred_user_id` - Filter to referrals attributed to this user. For first-tier referrals, this is the referred account owner; for second-tier referrals, this is the partner you recruited.
    /// * `referred_username` - Filter by the referred user's exact username. Ignored when `referred_user_id` is present.
    /// * `tier` - Filter to referrals from a single tier: first, second, or blueprint.
    /// * `business_prefix_query` - Case-insensitive business-name prefix, or an exact `biz_` account ID. Surrounding whitespace is ignored; blank values apply no filter.
    /// * `earning_partner_id` - Only the referrals this team member made, plus the ones they were added to as a deal participant. User ID, prefixed `user_`. Requires `referring_account_id`.
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
    ///         .businesses
    ///         .list(
    ///             &PartnersBusinessesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PartnersBusinessesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBusinessesResponse, ApiError> {
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
                "partners/businesses",
                None,
                QueryBuilder::new()
                    .string("referring_account_id", request.referring_account_id.clone())
                    .serialize("advance_status", request.advance_status.clone())
                    .serialize("status", request.status.clone())
                    .bool("has_earnings", request.has_earnings.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .bool("include_total_count", request.include_total_count.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .string("first_sale_before", request.first_sale_before.clone())
                    .string("first_sale_after", request.first_sale_after.clone())
                    .string("referred_user_id", request.referred_user_id.clone())
                    .string("referred_username", request.referred_username.clone())
                    .serialize("tier", request.tier.clone())
                    .string(
                        "business_prefix_query",
                        request.business_prefix_query.clone(),
                    )
                    .string("earning_partner_id", request.earning_partner_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &PartnersBusinessesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("referring_account_id", request.referring_account_id.clone())
            .serialize("advance_status", request.advance_status.clone())
            .serialize("status", request.status.clone())
            .bool("has_earnings", request.has_earnings.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .bool("include_total_count", request.include_total_count.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .string("created_before", request.created_before.clone())
            .string("created_after", request.created_after.clone())
            .string("first_sale_before", request.first_sale_before.clone())
            .string("first_sale_after", request.first_sale_after.clone())
            .string("referred_user_id", request.referred_user_id.clone())
            .string("referred_username", request.referred_username.clone())
            .serialize("tier", request.tier.clone())
            .string(
                "business_prefix_query",
                request.business_prefix_query.clone(),
            )
            .string("earning_partner_id", request.earning_partner_id.clone())
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
                            "partners/businesses",
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

    /// Retrieves a single referred business and its referral terms. The original referrer or a caller with company:basic:read on the business credited for the referral can read it.
    ///
    /// # Arguments
    ///
    /// * `id` - The partner business ID (a coma_ identifier).
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
    ///         .businesses
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveBusinessesResponse, ApiError> {
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
                &format!("partners/businesses/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Save annual payment projections for a business referred by the authenticated partner. Unaccepted projections can be replaced with updated annual card and BNPL payment volume. To qualify, payment volume over the last 30 days must reach 5% of the annual payment projection. Ads are excluded from CPA; ordinary ad commissions continue. Pass withdraw_projections: true without projection amounts to remove unaccepted projections. Accepted projections are immutable. Public API access requires a partner OAuth token; business API keys are not supported.
    ///
    /// # Arguments
    ///
    /// * `id` - The partner business ID (a coma_ identifier).
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
    ///         .businesses
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateBusinessesRequest {
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
        request: &UpdateBusinessesRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateBusinessesResponse, ApiError> {
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
                Method::PATCH,
                &format!("partners/businesses/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Accept the authenticated partner's CPA offer with the reviewed terms and signed offer token. Public API access requires a partner OAuth token; business API keys are not supported.
    ///
    /// # Arguments
    ///
    /// * `id` - The referred business relationship ID.
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
    ///     client.partners.businesses.accept_commission_advance(&"id".to_string(), &AcceptCommissionAdvanceBusinessesRequest {
    ///         acknowledgments: vec![AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem::CommissionReplacement, AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem::Clawback, AcceptCommissionAdvanceBusinessesRequestAcknowledgmentsItem::Term],
    ///         expected_amount_usd: "1079.99".to_string(),
    ///         offer_token: "eyJfcmFpbHMiOnsiZGF0YSI6MSwicHVyIjoiYmxvYl9pZCJ9fQ==--xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
    ///         terms_version: "2026-01-01".to_string()
    ///     }, None).await;
    /// }
    /// ```
    pub async fn accept_commission_advance(
        &self,
        id: &str,
        request: &AcceptCommissionAdvanceBusinessesRequest,
        options: Option<RequestOptions>,
    ) -> Result<AcceptCommissionAdvanceBusinessesResponse, ApiError> {
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
                Method::POST,
                &format!("partners/businesses/{}/accept_commission_advance", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Re-emails an outstanding invite to claim ownership of a referred business. Find the invite in the business's `pending_owner_invites`. Only the first-tier partner can resend, from a whop.com session or the Whop CLI or MCP with `partner:handoff`. Rate limited to one resend per invite every five minutes.
    ///
    /// # Arguments
    ///
    /// * `id` - The partner business ID (a coma_ identifier).
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
    ///         .businesses
    ///         .resend_invite(
    ///             &"id".to_string(),
    ///             &ResendInviteBusinessesRequest {
    ///                 invite_id: "ausri_xxxxxxxxxxxxxx".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn resend_invite(
        &self,
        id: &str,
        request: &ResendInviteBusinessesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResendInviteBusinessesResponse, ApiError> {
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
                Method::POST,
                &format!("partners/businesses/{}/resend_invite", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
