use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod logs;
pub use logs::LogsClient;
pub mod notes;
pub use notes::NotesClient;
pub struct MembersClient {
    pub http_client: HttpClient,
    pub logs: LogsClient,
    pub notes: NotesClient,
}

impl MembersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            logs: LogsClient::new(config.clone())?,
            notes: NotesClient::new(config.clone())?,
        })
    }

    /// Lists the members of an account. A member is one buyer's relationship with the account, regardless of how many memberships they hold.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account to list members for (`biz_` tag). Defaults to the account the credential acts as.
    /// * `access_level` - Filter by what the member can reach on the account.
    /// * `status` - Filter by whether the member is still part of the account.
    /// * `user_ids` - Only return members whose users match these `user_` identifiers.
    /// * `most_recent_actions` - Only return members whose `most_recent_action` is one of these values.
    /// * `membership_status` - `active` returns members holding at least one membership that currently grants access; `inactive` returns members holding at least one membership that does not. Combined with `product_ids`, `variant_ids`, or `promo_code_ids`, one membership must match all of them.
    /// * `product_ids` - Only return members holding a membership to one of these products, prefixed `prod_`. Unknown IDs match nothing.
    /// * `variant_ids` - Only return members holding a membership on one of these variants, prefixed `plan_`. Unknown IDs match nothing.
    /// * `promo_code_ids` - Only return members holding a membership bought with one of these promo codes, prefixed `promo_`. Unknown IDs match nothing.
    /// * `query` - Search members by name or username. An exact email address also matches when the credential holds the member:email:read scope.
    /// * `created_after` - Only members who joined after this ISO 8601 timestamp.
    /// * `created_before` - Only members who joined before this ISO 8601 timestamp.
    /// * `order` - Sort field. `most_recent_action_at` and `last_activity_at` place members without one first when ascending and last when descending.
    /// * `direction` - Sort direction.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `include_total_count` - Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
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
    ///         .members
    ///         .list(
    ///             &MembersListQueryRequest {
    ///                 user_ids: vec![Some("user_xxxxxxxxxxxxxx".to_string())],
    ///                 product_ids: vec![Some("prod_xxxxxxxxxxxxx".to_string())],
    ///                 variant_ids: vec![Some("plan_xxxxxxxxxxxxx".to_string())],
    ///                 promo_code_ids: vec![Some("promo_xxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 access_level: None,
    ///                 status: None,
    ///                 most_recent_actions: vec![],
    ///                 membership_status: None,
    ///                 query: None,
    ///                 created_after: None,
    ///                 created_before: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 include_total_count: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &MembersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembersResponse, ApiError> {
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
                "members",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("access_level", request.access_level.clone())
                    .serialize("status", request.status.clone())
                    .string_array("user_ids", request.user_ids.clone())
                    .serialize_array("most_recent_actions", request.most_recent_actions.clone())
                    .serialize("membership_status", request.membership_status.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .string_array("variant_ids", request.variant_ids.clone())
                    .string_array("promo_code_ids", request.promo_code_ids.clone())
                    .structured_query("query", request.query.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .bool("include_total_count", request.include_total_count.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &MembersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .serialize("access_level", request.access_level.clone())
            .serialize("status", request.status.clone())
            .string_array("user_ids", request.user_ids.clone())
            .serialize_array("most_recent_actions", request.most_recent_actions.clone())
            .serialize("membership_status", request.membership_status.clone())
            .string_array("product_ids", request.product_ids.clone())
            .string_array("variant_ids", request.variant_ids.clone())
            .string_array("promo_code_ids", request.promo_code_ids.clone())
            .structured_query("query", request.query.clone())
            .string("created_after", request.created_after.clone())
            .string("created_before", request.created_before.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .bool("include_total_count", request.include_total_count.clone())
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
                            "members",
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

    /// Adds a person to an account as a member. If the person is already a member of the account, that member is returned unchanged.
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
    ///         .members
    ///         .create(
    ///             &CreateMembersRequest {
    ///                 email: "jane@example.com".to_string(),
    ///                 account_id: None,
    ///                 name: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateMembersRequest,
        options: Option<RequestOptions>,
    ) -> Result<Member, ApiError> {
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
                "members",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a member by ID. Accessible to the account and to the member's own user.
    ///
    /// # Arguments
    ///
    /// * `id` - Member ID (`mber_` tag).
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
    ///     client.members.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Member, ApiError> {
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
                &format!("members/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a member.
    ///
    /// # Arguments
    ///
    /// * `id` - Member ID (`mber_` tag).
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
    ///         .members
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateMembersRequest {
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
        request: &UpdateMembersRequest,
        options: Option<RequestOptions>,
    ) -> Result<Member, ApiError> {
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
                &format!("members/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Unsubscribes a member from the account's marketing communications with the token from their unsubscribe link. Inboxes post here directly for one-click unsubscribe (RFC 8058).
    ///
    /// # Arguments
    ///
    /// * `id` - The member to unsubscribe, prefixed `mber_`.
    /// * `token` - The token from the member's unsubscribe link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .members
    ///         .unsubscribe(
    ///             &"id".to_string(),
    ///             &UnsubscribeQueryRequest {
    ///                 token: "token".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unsubscribe(
        &self,
        id: &str,
        request: &UnsubscribeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
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
                &format!("members/{}/unsubscribe", id),
                None,
                QueryBuilder::new()
                    .string("token", request.token.clone())
                    .build(),
                options,
            )
            .await
    }
}
