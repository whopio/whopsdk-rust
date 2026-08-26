use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AuthorizedUsersClient {
    pub http_client: HttpClient,
}

impl AuthorizedUsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of authorized team members for a company, with optional filtering by user, role, and creation date.
    ///
    /// Required permissions:
    /// - `company:authorized_user:read`
    /// - `member:email:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list authorized users for.
    /// * `user_id` - Filter results to a specific user to check if they are an authorized team member.
    /// * `created_before` - Only return authorized users created before this timestamp.
    /// * `created_after` - Only return authorized users created after this timestamp.
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
    ///         .authorized_users
    ///         .list(
    ///             &AuthorizedUsersListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 user_id: Some("user_xxxxxxxxxxxxx".to_string()),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AuthorizedUsersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAuthorizedUsersResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "authorized_users",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .string("user_id", request.user_id.clone())
                    .serialize("role", request.role.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Add a new authorized user to a company.
    ///
    /// Required permissions:
    /// - `authorized_user:create`
    /// - `member:email:read`
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
    ///         .authorized_users
    ///         .create(
    ///             &CreateAuthorizedUsersRequest {
    ///                 company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 role: GrantableAuthorizedUserRoles::Owner,
    ///                 user_id: "user_xxxxxxxxxxxxx".to_string(),
    ///                 elevation: None,
    ///                 send_emails: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAuthorizedUsersRequest,
        options: Option<RequestOptions>,
    ) -> Result<AuthorizedUser, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "authorized_users",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing authorized user.
    ///
    /// Required permissions:
    /// - `company:authorized_user:read`
    /// - `member:email:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the authorized user to retrieve.
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
    ///         .authorized_users
    ///         .retrieve(&"ausr_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AuthorizedUser, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("authorized_users/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Remove an authorized user from a company.
    ///
    /// Required permissions:
    /// - `authorized_user:delete`
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the authorized user or user to remove.
    /// * `company_id` - The ID of the company the authorized user belongs to. Optional if the authorized user ID is provided.
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
    ///         .authorized_users
    ///         .delete(
    ///             &"ausr_xxxxxxxxxxxxx".to_string(),
    ///             &AuthorizedUsersDeleteQueryRequest {
    ///                 company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        request: &AuthorizedUsersDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("authorized_users/{}", id),
                None,
                QueryBuilder::new()
                    .string("company_id", request.company_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
