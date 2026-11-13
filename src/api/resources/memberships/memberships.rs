use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct MembershipsClient {
    pub http_client: HttpClient,
}

impl MembershipsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists every membership the caller can read: an account API key its account's; a user credential their own plus those of every account they manage. `account_id` and `user_id` only narrow that list — values outside the caller's reach return fewer results, not an error.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Narrow to one account (`biz_` tag). With read access to the account this lists all of its memberships; without, only the caller's own memberships in it.
    /// * `user_id` - Narrow to one user's memberships (`user_` tag, or `me` for the caller). A user outside the caller's visible set returns an empty list.
    /// * `member_id` - Narrow to one member's memberships (`mber_` tag), including memberships bought by another business. A member outside the caller's visible set returns an empty list.
    /// * `external_account` - Narrow to memberships linked to one Discord or Telegram account, given as the platform's user ID prefixed with `discord_` or `telegram_`. Discord matches include memberships imported with that Discord ID. Any other format is a 400.
    /// * `status` - Filter by billing state. `trialing`, `active`, `past_due`, `completed`, `canceled`, and `expired` match the membership's `status`; `active`, `trialing`, and `past_due` exclude memberships set to cancel at period end. The other values are derived buckets that never appear as a `status`: `canceling` matches memberships set to cancel at period end; `paused` matches memberships with payment collection paused; `expiring` matches one-time purchases that have not yet expired; `one_time` matches one-time purchases that never expire; `unclaimed` matches memberships with no buyer user, including memberships bought by another business; `banned` matches memberships whose buyer is banned from the account.
    /// * `statuses` - Filter to memberships matching any of these billing states, with the same meanings as `status`. Repeat the parameter to pass several. Combined with `status` when both are sent.
    /// * `product_id` - Filter to memberships of this product (`prod_` tag).
    /// * `product_ids` - Filter to memberships of any of these products, each prefixed `prod_`. Repeat the parameter to pass several.
    /// * `variant_id` - Filter to memberships of this variant (`plan_` tag).
    /// * `variant_ids` - Filter to memberships of any of these variants, each prefixed `plan_`. Repeat the parameter to pass several.
    /// * `promo_code_ids` - Filter to memberships discounted by any of these promo codes, each prefixed `promo_`. Repeat the parameter to pass several.
    /// * `created_after` - Only memberships created after this ISO 8601 timestamp.
    /// * `created_before` - Only memberships created before this ISO 8601 timestamp.
    /// * `order` - Sort field.
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
    ///         .memberships
    ///         .list(
    ///             &MembershipsListQueryRequest {
    ///                 external_account: Some("discord_123456789012345678".to_string()),
    ///                 product_ids: vec![Some("prod_xxxxxxxxxxxxx".to_string())],
    ///                 variant_ids: vec![Some("plan_xxxxxxxxxxxxx".to_string())],
    ///                 promo_code_ids: vec![Some("promo_xxxxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 user_id: None,
    ///                 member_id: None,
    ///                 status: None,
    ///                 statuses: vec![],
    ///                 product_id: None,
    ///                 variant_id: None,
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
        request: &MembershipsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListMembershipsResponse, ApiError> {
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
                "memberships",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("member_id", request.member_id.clone())
                    .string("external_account", request.external_account.clone())
                    .serialize("status", request.status.clone())
                    .serialize_array("statuses", request.statuses.clone())
                    .string("product_id", request.product_id.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .string("variant_id", request.variant_id.clone())
                    .string_array("variant_ids", request.variant_ids.clone())
                    .string_array("promo_code_ids", request.promo_code_ids.clone())
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
        request: &MembershipsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .string("user_id", request.user_id.clone())
            .string("member_id", request.member_id.clone())
            .string("external_account", request.external_account.clone())
            .serialize("status", request.status.clone())
            .serialize_array("statuses", request.statuses.clone())
            .string("product_id", request.product_id.clone())
            .string_array("product_ids", request.product_ids.clone())
            .string("variant_id", request.variant_id.clone())
            .string_array("variant_ids", request.variant_ids.clone())
            .string_array("promo_code_ids", request.promo_code_ids.clone())
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
                            "memberships",
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

    /// Emails one recipient an invitation to a free variant's membership. The invitation is bound to that recipient; after signing in, accepting it immediately grants the membership without checkout. This Experimental endpoint is available only to accounts enabled for membership invitations.
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
    ///         .memberships
    ///         .invite(
    ///             &InviteMembershipsRequest {
    ///                 variant_id: "plan_xxxxxxxxxxxxxx".to_string(),
    ///                 email: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn invite(
        &self,
        request: &InviteMembershipsRequest,
        options: Option<RequestOptions>,
    ) -> Result<InviteMembershipsResponse, ApiError> {
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
                "memberships/invite",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a membership by ID or license key. Accessible to the account and to the membership's own user.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag), or a software license key.
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
    ///     client.memberships.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a membership's metadata, scheduled cancellation, renewal payment method, renewal cadence, next renewal date, or renewal price.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag), or a software license key.
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
    ///         .memberships
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateMembershipsRequest {
    ///                 billing_period_days: Some(45),
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
        request: &UpdateMembershipsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Applies a promo code to an `active` or `trialing` membership that does not already have one and has exactly one recurring item, for Stripe-billed memberships and memberships billed by Whop's billing engine. The discount lands on the next invoice and follows the code's duration (`once`, `repeating`, or `forever`).
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///         .memberships
    ///         .apply_promo_code(
    ///             &"id".to_string(),
    ///             &ApplyPromoCodeMembershipsRequest {
    ///                 promo_code: "SAVE20".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn apply_promo_code(
        &self,
        id: &str,
        request: &ApplyPromoCodeMembershipsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}/apply_promo_code", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Assigns an affiliate to a membership and pays them the commission you set on its future payments; past payments are not recalculated. A user who is not yet an affiliate of your account becomes one, which also requires `affiliate:create`. Send a new `commission_type` or `commission_value` for the membership's current affiliate to change their commission; a membership that already has a different affiliate returns a conflict. Works for active or trialing memberships with one recurring variant that bill through Stripe or Whop's billing engine, and not for marketplace memberships, paused payments, or a scheduled cancellation. You cannot assign yourself.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///         .memberships
    ///         .assign_affiliate(
    ///             &"id".to_string(),
    ///             &AssignAffiliateMembershipsRequest {
    ///                 commission_type: AssignAffiliateMembershipsRequestCommissionType::FlatFee,
    ///                 commission_value: 5.0,
    ///                 email: Some("affiliate@example.com".to_string()),
    ///                 user_id: None,
    ///                 username: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn assign_affiliate(
        &self,
        id: &str,
        request: &AssignAffiliateMembershipsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}/assign_affiliate", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancels a membership, either immediately or at the end of the current billing period. Buyers cannot cancel buy-now-pay-later (`splitit`, `sezzle`) or non-trial split-pay memberships.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///         .memberships
    ///         .cancel(
    ///             &"id".to_string(),
    ///             &CancelMembershipsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        request: &CancelMembershipsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}/cancel", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds free days to a membership, extending its current billing period, expiration date, or trial depending on the plan type.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///         .memberships
    ///         .extend(
    ///             &"id".to_string(),
    ///             &ExtendMembershipsRequest { days: 7 },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn extend(
        &self,
        id: &str,
        request: &ExtendMembershipsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}/extend", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pauses a membership's recurring payment collection. The customer keeps access but is not charged until the membership is resumed.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///         .memberships
    ///         .pause(
    ///             &"id".to_string(),
    ///             &PauseMembershipsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pause(
        &self,
        id: &str,
        request: &PauseMembershipsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}/pause", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Restores access to a `canceled` or `expired` membership that contains only one-time purchases and sets its `status` to `completed`. Lifetime memberships regain lifetime access; for memberships with an expiration, `days` sets the new `current_period_end`. Active and recurring memberships cannot be reactivated.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///         .memberships
    ///         .reactivate(
    ///             &"id".to_string(),
    ///             &ReactivateMembershipsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reactivate(
        &self,
        id: &str,
        request: &ReactivateMembershipsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}/reactivate", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Resumes a previously paused membership's recurring payment collection. Billing resumes on the next cycle.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///     client.memberships.resume(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn resume(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}/resume", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Re-runs access fulfillment for a membership: recomputes the member's content access on Whop, re-validates their Discord link (re-adding them to the server and re-assigning roles if needed), and re-fulfills TradingView indicator access. Telegram access is invite-based and is not resynced. The work runs in the background and the outcome is written to the membership's logs.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///         .memberships
    ///         .resync_access(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn resync_access(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
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
                &format!("memberships/{}/resync_access", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a one-use transfer URL for a membership. Opening the URL while logged into a different Whop account claims the membership onto that account. The membership's buyer can generate a link for their own membership with `membership:transfer` when the product allows transfers and the membership is `trialing`, `active`, or `completed`. An account credential with `membership:update` bypasses both restrictions.
    ///
    /// # Arguments
    ///
    /// * `id` - Membership ID (`mem_` tag).
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
    ///     client.memberships.transfer(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn transfer(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<TransferMembershipsResponse, ApiError> {
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
                &format!("memberships/{}/transfer", id),
                None,
                None,
                options,
            )
            .await
    }
}
