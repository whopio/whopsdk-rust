use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct PartnerReferralRequestsClient {
    pub http_client: HttpClient,
}

impl PartnerReferralRequestsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists your referral links and attribution requests, including incoming requests for you or businesses you own, with filters for recipient, partner, type, and status.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only requests for this business ID, prefixed `biz_`.
    /// * `target_user_id` - Only requests received by this user ID, prefixed `user_`.
    /// * `partner_id` - Only requests sent by this partner's user ID, prefixed `user_`.
    /// * `referring_account_id` - Only the referral links created for memberships on this business's team, prefixed `biz_`. Needs read access to the business: its owner sees every member's links, a manager the links of the people reporting to them, and other members only their own.
    /// * `status` - Only requests with this approval status.
    /// * `request_type` - Only requests initiated in this way.
    /// * `authorized_user_id` - Only requests created for this team membership, prefixed `ausr_`.
    /// * `member_id` - Only requests that credit this member record, prefixed `mber_`. A request created for a team membership also credits its holder's member record.
    /// * `personal` - When true, only personal requests, created for neither a team membership nor a member record. When false, only requests created for one.
    /// * `order` - Field used to sort requests.
    /// * `direction` - Sort direction.
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
    ///         .partner_referral_requests
    ///         .list(
    ///             &PartnerReferralRequestsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PartnerReferralRequestsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPartnerReferralRequestsResponse, ApiError> {
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
                "partner_referral_requests",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("target_user_id", request.target_user_id.clone())
                    .string("partner_id", request.partner_id.clone())
                    .string("referring_account_id", request.referring_account_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("request_type", request.request_type.clone())
                    .string("authorized_user_id", request.authorized_user_id.clone())
                    .string("member_id", request.member_id.clone())
                    .bool("personal", request.personal.clone())
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

    pub async fn list_paginated(
        &self,
        request: &PartnerReferralRequestsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .string("target_user_id", request.target_user_id.clone())
            .string("partner_id", request.partner_id.clone())
            .string("referring_account_id", request.referring_account_id.clone())
            .serialize("status", request.status.clone())
            .serialize("request_type", request.request_type.clone())
            .string("authorized_user_id", request.authorized_user_id.clone())
            .string("member_id", request.member_id.clone())
            .bool("personal", request.personal.clone())
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
                            "partner_referral_requests",
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

    /// Creates a referral link, or sends an attribution request to an existing business or user for approval. Links require an enrolled partner who is not suspended; attribution requests require an enrolled, verified partner. Recipients do not need to join the partner program.
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
    ///     client.partner_referral_requests.create(&CreatePartnerReferralRequestsRequestBody::CreatePartnerReferralRequestsRequestBodyAccountId(CreatePartnerReferralRequestsRequestBodyAccountId {
    ///         account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///         ..Default::default()
    ///     }), None).await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePartnerReferralRequestsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
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
                "partner_referral_requests",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a referral link or attribution request by ID, including its partner, recipient, approval status, and referral code when present.
    ///
    /// # Arguments
    ///
    /// * `id` - Partner referral request ID, prefixed `prfr_`.
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
    ///         .partner_referral_requests
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
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
                &format!("partner_referral_requests/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Accepts a pending attribution request as the receiving user or business owner, assigning the requesting partner as that user's or business's referrer.
    ///
    /// # Arguments
    ///
    /// * `id` - Partner referral request ID, prefixed `prfr_`.
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
    ///         .partner_referral_requests
    ///         .accept(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn accept(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
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
                &format!("partner_referral_requests/{}/accept", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancels a pending attribution request you sent so the recipient can no longer accept it, and returns the cancelled request.
    ///
    /// # Arguments
    ///
    /// * `id` - Partner referral request ID, prefixed `prfr_`.
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
    ///         .partner_referral_requests
    ///         .cancel(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
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
                &format!("partner_referral_requests/{}/cancel", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Declines a pending attribution request for you or a business you own, marking it as denied without assigning the requesting partner as a referrer.
    ///
    /// # Arguments
    ///
    /// * `id` - Partner referral request ID, prefixed `prfr_`.
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
    ///         .partner_referral_requests
    ///         .decline(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn decline(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
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
                &format!("partner_referral_requests/{}/decline", id),
                None,
                None,
                options,
            )
            .await
    }
}
