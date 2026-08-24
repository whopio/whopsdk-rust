use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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
    /// * `status` - Filter by billing state. `canceling` matches active memberships set to cancel at period end; `paused` matches memberships with payment collection paused.
    /// * `product_id` - Filter to memberships of this product (`prod_` tag). Repeat as product_ids[] for several.
    /// * `plan_id` - Filter to memberships of this plan (`plan_` tag). Repeat as plan_ids[] for several.
    /// * `created_after` - Only memberships created after this ISO 8601 timestamp.
    /// * `created_before` - Only memberships created before this ISO 8601 timestamp.
    /// * `order` - Sort field.
    /// * `direction` - Sort direction.
    /// * `first` - Number of memberships to return from the start of the window.
    /// * `after` - Cursor to paginate forwards from.
    /// * `last` - Number of memberships to return from the end of the window.
    /// * `before` - Cursor to paginate backwards from.
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
    ///                 ..Default::default()
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
        self.http_client
            .execute_request(
                Method::GET,
                "memberships",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .serialize("status", request.status.clone())
                    .string("product_id", request.product_id.clone())
                    .string("plan_id", request.plan_id.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
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

    /// Sends an email inviting one recipient to join the account through a free plan. Identify the recipient by exactly one of `user_id` or `email`. The invitation is bound to that recipient; after signing in, accepting it immediately grants the membership without checkout. This Experimental endpoint is available only to accounts enabled for membership invitations.
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
    ///             &InviteMembershipsRequestBody::InviteMembershipsRequestBodyUserID(
    ///                 InviteMembershipsRequestBodyUserID {
    ///                     plan_id: "plan_xxxxxxxxxxxxxx".to_string(),
    ///                     user_id: "user_xxxxxxxxxxxxxx".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn invite(
        &self,
        request: &InviteMembershipsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<InviteMembershipsResponse, ApiError> {
        self.http_client
            .execute_request(
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
        self.http_client
            .execute_request(
                Method::GET,
                &format!("memberships/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a membership: merge metadata key-value pairs, or toggle `cancel_at_period_end` — `true` schedules the cancellation for the end of the current billing period, `false` reverses a pending one.
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
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("memberships/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Add free days to extend a membership's current billing period, expiration date, or Stripe trial.
    ///
    /// Required permissions:
    /// - `member:manage`
    /// - `member:email:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the membership.
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
    ///         .add_free_days_membership(
    ///             &"mem_xxxxxxxxxxxxxx".to_string(),
    ///             &AddFreeDaysMembershipRequest { free_days: 42 },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_free_days_membership(
        &self,
        id: &str,
        request: &AddFreeDaysMembershipRequest,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("memberships/{}/add_free_days", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancels a membership. Pass `cancel_at_period_end: true` to stop auto-renewal and keep access until the current billing period ends. Omit it (or pass `false`) to revoke access immediately. Buyers cannot cancel buy-now-pay-later (`splitit`, `sezzle`) or non-trial split-pay memberships.
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
        self.http_client
            .execute_request(
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
        self.http_client
            .execute_request(
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
        self.http_client
            .execute_request(
                Method::POST,
                &format!("memberships/{}/pause", id),
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
        self.http_client
            .execute_request(
                Method::POST,
                &format!("memberships/{}/resume", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Re-run access fulfillment for a membership. Recomputes the member's content access on Whop, re-validates their Discord link (re-adding them to the server and re-assigning roles if needed), and re-fulfills TradingView indicator access. Telegram access is invite-based and cannot be resynced here. The outcome is written to the membership's logs.
    ///
    /// Required permissions:
    /// - `membership:resync_access`
    /// - `member:email:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the membership to resync access for.
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
    ///         .resync_access_membership(&"mem_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn resync_access_membership(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
        self.http_client
            .execute_request(
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
        self.http_client
            .execute_request(
                Method::POST,
                &format!("memberships/{}/transfer", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Reverse a pending cancellation for a membership that was scheduled to cancel at period end.
    ///
    /// Required permissions:
    /// - `member:manage`
    /// - `member:email:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the membership to uncancel.
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
    ///         .uncancel_membership(&"mem_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn uncancel_membership(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Membership, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("memberships/{}/uncancel", id),
                None,
                None,
                options,
            )
            .await
    }
}
