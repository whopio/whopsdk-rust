use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct TeamMembersClient {
    pub http_client: HttpClient,
}

impl TeamMembersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists an account's team members, including pending invites. A user credential with `company:basic:read` may list only its own joined membership by passing its own `user_id` and `status=joined`. Listing `role=workforce` is also allowed with the `bounty:create` scope. A signed-in Partner, or a member others report to, without `company:authorized_user:read` lists only their own joined membership and the members who report up to them, never pending invites. Members with the view-only Partner role are listed after all other members.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account ID, prefixed `biz_`.
    /// * `status` - Only return members with this status: `joined` (accepted members) or `pending` (pending invites). Both are returned by default.
    /// * `user_id` - Only return the membership for this user ID, prefixed `user_`.
    /// * `user_ids` - Only return the memberships of these users, prefixed `user_`, at most 100. IDs that match no user are ignored. With `account_id=global`, a user can have one in each partner account. Can't be combined with `user_id`.
    /// * `role` - Only return members with this role. `custom` matches members on a dashboard-managed custom role.
    /// * `query` - Only return members whose name or username contains this text, and invites whose email contains it.
    /// * `reports_to_id` - Only return the direct reports of this team member, whose ID may start with `ausr_` or `mber_`. Pass the account ID, prefixed `biz_`, for the members who report to the business directly. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    /// * `direct_reports_min` - Only return members with at least this many direct reports. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    /// * `direct_reports_max` - Only return members with at most this many direct reports; `0` returns the people who manage no one. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    /// * `team_lead_id` - Only return this user, prefixed `user_`, and everyone under them in today's org chart: the same team the stats API's `team_lead_id` reads. With `account_id=global`, their line in every partner account where they hold a role. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    /// * `has_referred_businesses` - When true, only members who referred at least one business for their account; when false, only members who referred none. Pending invites are excluded, and the list pages in `created_at` order without moving Partner-role members last.
    /// * `created_before` - Only return members added before this ISO 8601 timestamp.
    /// * `created_after` - Only return members added after this ISO 8601 timestamp.
    /// * `order` - Field used to sort members. Members with the view-only Partner role are listed after all other members whichever way the list is sorted.
    /// * `direction` - Sort direction. Defaults to `desc`.
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
    ///         .team_members
    ///         .list(
    ///             &TeamMembersListQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 status: None,
    ///                 user_id: None,
    ///                 user_ids: vec![],
    ///                 role: None,
    ///                 query: None,
    ///                 reports_to_id: None,
    ///                 direct_reports_min: None,
    ///                 direct_reports_max: None,
    ///                 team_lead_id: None,
    ///                 has_referred_businesses: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 order: None,
    ///                 direction: None,
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
        request: &TeamMembersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTeamMembersResponse, ApiError> {
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
                "team_members",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .string("user_id", request.user_id.clone())
                    .string_array("user_ids", request.user_ids.clone())
                    .serialize("role", request.role.clone())
                    .structured_query("query", request.query.clone())
                    .string("reports_to_id", request.reports_to_id.clone())
                    .int("direct_reports_min", request.direct_reports_min.clone())
                    .int("direct_reports_max", request.direct_reports_max.clone())
                    .string("team_lead_id", request.team_lead_id.clone())
                    .bool(
                        "has_referred_businesses",
                        request.has_referred_businesses.clone(),
                    )
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
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
        request: &TeamMembersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .serialize("status", request.status.clone())
            .string("user_id", request.user_id.clone())
            .string_array("user_ids", request.user_ids.clone())
            .serialize("role", request.role.clone())
            .structured_query("query", request.query.clone())
            .string("reports_to_id", request.reports_to_id.clone())
            .int("direct_reports_min", request.direct_reports_min.clone())
            .int("direct_reports_max", request.direct_reports_max.clone())
            .string("team_lead_id", request.team_lead_id.clone())
            .bool(
                "has_referred_businesses",
                request.has_referred_businesses.clone(),
            )
            .string("created_before", request.created_before.clone())
            .string("created_after", request.created_after.clone())
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
                            "team_members",
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

    /// Adds a member to an account's team with a system role. Identify them by exactly one of `user_id` or `email`. If the person has not yet accepted, or the email does not belong to a Whop account yet, an invitation is sent instead and the response is `202` with the pending team member, whose `status` is `pending` and whose id is prefixed `ausri_`. If they already have a pending invite, the request fails with a `400`. Granting the `workforce` role is also allowed with the `bounty:create` scope.
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
    ///         .team_members
    ///         .create(
    ///             &CreateTeamMembersRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 role: CreateTeamMembersRequestRole::SalesManager,
    ///                 user_id: Some("user_xxxxxxxxxxxxxx".to_string()),
    ///                 email: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateTeamMembersRequest,
        options: Option<RequestOptions>,
    ) -> Result<TeamMember, ApiError> {
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
                "team_members",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a team member or pending invite by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Team member ID. Accepted members have an ID starting with `ausr_` or `mber_`, and pending invites one starting with `ausri_`. Treat it as opaque.
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
    ///     client.team_members.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TeamMember, ApiError> {
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
                &format!("team_members/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes a team member from the account, or revokes a pending invite when given an `ausri_` ID. A user session may delete its own membership to leave the team without the delete scope. Removing a member on the `workforce` role is also allowed with the `bounty:create` scope. The account owner cannot be removed.
    ///
    /// # Arguments
    ///
    /// * `id` - Team member ID. Accepted members have an ID starting with `ausr_` or `mber_`, and pending invites one starting with `ausri_`. Treat it as opaque.
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
    ///     client.team_members.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteTeamMembersResponse, ApiError> {
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
                Method::DELETE,
                &format!("team_members/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Changes a team member's system role. Requires a user session — account API keys cannot change member roles. The account owner's role cannot be changed, and you cannot change your own role.
    ///
    /// # Arguments
    ///
    /// * `id` - Team member ID. Accepted members have an ID starting with `ausr_` or `mber_`, and pending invites one starting with `ausri_`. Treat it as opaque.
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
    ///         .team_members
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateTeamMembersRequest {
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
        request: &UpdateTeamMembersRequest,
        options: Option<RequestOptions>,
    ) -> Result<TeamMember, ApiError> {
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
                &format!("team_members/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
