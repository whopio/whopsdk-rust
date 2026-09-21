use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct WaitlistEntriesClient {
    pub http_client: HttpClient,
}

impl WaitlistEntriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists visible waitlist signups. waitlist_entry:read grants the user's own signups; plan:waitlist:read grants signups for authorized seller accounts. With both permissions, returns their union. Account credentials are limited to their account. Filters narrow this set.
    ///
    /// # Arguments
    ///
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `plan_id` - Only return signups for this plan, prefixed `plan_`.
    /// * `account_id` - Only return signups submitted to this seller account, prefixed `biz_`.
    /// * `product_id` - Only return signups for plans on this product, prefixed `prod_`.
    /// * `status` - Only return signups in this state. Canceled signups are returned only when `status` is `canceled`.
    /// * `created_before` - Only return signups submitted at or before this ISO 8601 timestamp.
    /// * `created_after` - Only return signups submitted at or after this ISO 8601 timestamp.
    /// * `order` - The field to sort results by. Defaults to `created_at`.
    /// * `direction` - The sort direction for results. Defaults to descending.
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
    ///         .waitlist_entries
    ///         .list(
    ///             &WaitlistEntriesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WaitlistEntriesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWaitlistEntriesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "waitlist_entries",
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .string("plan_id", request.plan_id.clone())
                    .string("account_id", request.account_id.clone())
                    .string("product_id", request.product_id.clone())
                    .serialize("status", request.status.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Joins a free waitlist plan as the authenticated user. Requires waitlist_entry:create. Repeated joins return the existing pending entry, or an approved entry with a valid membership. Paid plans are rejected; no payment method is collected and no membership is granted.
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
    ///         .waitlist_entries
    ///         .create(
    ///             &CreateWaitlistEntriesRequest {
    ///                 plan_id: "plan_xxxxxxxxxxxxxx".to_string(),
    ///                 custom_field_responses: None,
    ///                 metadata: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateWaitlistEntriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "waitlist_entries",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Queues approval of every pending signup for an account, optionally narrowed to a plan. Requires plan:waitlist:manage. Paid signups may charge saved payment methods. Approval runs asynchronously: list signups with `status` set to `pending` to follow progress, and retrieve a signup to read its outcome. Signups created after this request are excluded.
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
    ///         .waitlist_entries
    ///         .approve_all(
    ///             &ApproveAllWaitlistEntriesRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 plan_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn approve_all(
        &self,
        request: &ApproveAllWaitlistEntriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApproveAllWaitlistEntriesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "waitlist_entries/approve_all",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a signup owned by the caller with waitlist_entry:read, or submitted to an account they can read with plan:waitlist:read.
    ///
    /// # Arguments
    ///
    /// * `id` - Waitlist signup ID, prefixed `entry_`.
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
    ///         .waitlist_entries
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("waitlist_entries/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Queues approval of a pending signup. Requires plan:waitlist:manage on its seller account. Paid signups may charge their saved payment method. Returns the signup's current state; retrieve it to read `status` and `approval_failure_reason` after processing.
    ///
    /// # Arguments
    ///
    /// * `id` - Waitlist signup ID, prefixed `entry_`.
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
    ///         .waitlist_entries
    ///         .approve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn approve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("waitlist_entries/{}/approve", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Withdraws the caller's pending personal signup. Requires waitlist_entry:cancel. Does not cancel an approved membership.
    ///
    /// # Arguments
    ///
    /// * `id` - Waitlist signup ID, prefixed `entry_`.
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
    ///         .waitlist_entries
    ///         .cancel(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("waitlist_entries/{}/cancel", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Denies a pending signup. Requires plan:waitlist:manage on its seller account.
    ///
    /// # Arguments
    ///
    /// * `id` - Waitlist signup ID, prefixed `entry_`.
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
    ///     client.waitlist_entries.deny(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn deny(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("waitlist_entries/{}/deny", id),
                None,
                None,
                options,
            )
            .await
    }
}
