use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod oauth_grants;
pub use oauth_grants::OauthGrantsClient;
pub mod passkeys;
pub use passkeys::PasskeysClient;
pub mod preferences;
pub use preferences::PreferencesClient2;
pub struct UsersClient {
    pub http_client: HttpClient,
    pub oauth_grants: OauthGrantsClient,
    pub passkeys: PasskeysClient,
    pub preferences: PreferencesClient2,
}

impl UsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            oauth_grants: OauthGrantsClient::new(config.clone())?,
            passkeys: PasskeysClient::new(config.clone())?,
            preferences: PreferencesClient2::new(config.clone())?,
        })
    }

    /// Search for users by name or username, ranked by social proximity to the authenticated user. Returns the user's most recently followed users when no query is given.
    ///
    /// # Arguments
    ///
    /// * `query` - A search term to filter users by name or username.
    /// * `first` - The number of users to return (max 50).
    /// * `after` - A cursor; returns users after this position.
    /// * `last` - The number of users to return from the end of the range.
    /// * `before` - A cursor; returns users before this position.
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
    ///         .users
    ///         .list(
    ///             &UsersListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &UsersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListUsersResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "users",
                None,
                QueryBuilder::new()
                    .structured_query("query", request.query.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the authenticated user — the self view of the user object. Same shape as `GET /users/{id}`, with the self-only fields populated: `email` (email-read scope), `staff` (Whop staff only, staff-read scope), `balance` and `earnings_usd` (balance-read scope), the opt-in `balance_history`, and every linked social account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - When set, returns your account-specific profile overrides for this account.
    /// * `include_balance_history` - Also compute your balance history (opt-in; runs a heavier query). Ignored for callers without balance-read scope.
    /// * `from` - Balance-history window start, ISO 8601 date or datetime. Defaults to 30 days ago. Only used with `include_balance_history`.
    /// * `to` - Balance-history window end, ISO 8601 date or datetime. Defaults to now. Only used with `include_balance_history`.
    /// * `interval` - Balance-history point granularity. Defaults to `day`. Only used with `include_balance_history`.
    /// * `time_zone` - IANA time zone the balance-history points are bucketed in. Defaults to `UTC`. Only used with `include_balance_history`.
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
    ///         .users
    ///         .me(
    ///             &MeQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn me(
        &self,
        request: &MeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "users/me",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .bool(
                        "include_balance_history",
                        request.include_balance_history.clone(),
                    )
                    .string("from", request.from.clone())
                    .string("to", request.to.clone())
                    .serialize("interval", request.interval.clone())
                    .string("time_zone", request.time_zone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates the authenticated user's global profile, or their profile override for an account when account_id is given. Not available to API keys.
    ///
    /// # Arguments
    ///
    /// * `account_id` - When set, updates the authenticated user's profile override for this account instead of their global profile.
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
    ///         .users
    ///         .update_me(
    ///             &UpdateMeUsersRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_me(
        &self,
        request: &UpdateMeUsersRequest,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                "users/me",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a user by `user_` tag or username, or the authenticated user with the reserved id `me`. Profiles include linked social accounts — reading your own profile returns every linked account, other profiles only what is public on Whop (the primary Discord and the X account). The self-only fields are populated only when the id is `me`: `email` (email-read scope), `staff` (Whop staff only, staff-read scope), `balance` and `earnings_usd` (balance-read scope), and the opt-in `balance_history`. They are always `null` when addressing a user by tag or username.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID (prefixed `user_`), username, or `me` for the authenticated user.
    /// * `account_id` - When set, returns the user's account-specific profile overrides for this account.
    /// * `include_balance_history` - Also compute your balance history (opt-in; runs a heavier query). Only applies when the id is `me`; ignored for callers without balance-read scope.
    /// * `from` - Balance-history window start, ISO 8601 date or datetime. Defaults to 30 days ago. Only used with `include_balance_history`.
    /// * `to` - Balance-history window end, ISO 8601 date or datetime. Defaults to now. Only used with `include_balance_history`.
    /// * `interval` - Balance-history point granularity. Defaults to `day`. Only used with `include_balance_history`.
    /// * `time_zone` - IANA time zone the balance-history points are bucketed in. Defaults to `UTC`. Only used with `include_balance_history`.
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
    ///         .users
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &UsersRetrieveQueryRequest {
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
        request: &UsersRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("users/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .bool(
                        "include_balance_history",
                        request.include_balance_history.clone(),
                    )
                    .string("from", request.from.clone())
                    .string("to", request.to.clone())
                    .serialize("interval", request.interval.clone())
                    .string("time_zone", request.time_zone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a user, addressed by `user_` tag, username, or the reserved id `me` for the authenticated user. A user token updates their own global profile; an API key updates the user's account-specific profile override (account_id required).
    ///
    /// # Arguments
    ///
    /// * `id` - User ID (prefixed `user_`), username, or `me` for the authenticated user.
    /// * `account_id` - The account whose profile override to update. Required for API key callers.
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
    ///         .users
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateUsersRequest {
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
        request: &UpdateUsersRequest,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("users/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Checks whether a user has access to an account, product, or experience the caller can reach.
    ///
    /// # Arguments
    ///
    /// * `id` - The user_ tag or username to check access for.
    /// * `resource_id` - An account (biz_), product (prod_), or experience (exp_) ID.
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
    ///         .users
    ///         .check_access(&"id".to_string(), &"resource_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn check_access(
        &self,
        id: &str,
        resource_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CheckAccessUsersResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("users/{}/access/{}", id, resource_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists the recommended actions computed for the user: personal suggestions (e.g. start a business or become an affiliate) pooled with the highest-impact actions across the accounts the user owns. Business actions are tagged with their `account_id`/`account_name`; personal actions leave those `null`. Self-only: `id` must be `me` or the authenticated user's own tag/username.
    ///
    /// # Arguments
    ///
    /// * `id` - `me`, or the authenticated user's own `user_` tag or username.
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
    ///         .users
    ///         .recommend_actions(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn recommend_actions(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RecommendActionsUsersResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("users/{}/recommend_actions", id),
                None,
                None,
                options,
            )
            .await
    }
}
