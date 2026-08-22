use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod logs;
pub use logs::LogsClient;
pub struct MembersClient {
    pub http_client: HttpClient,
    pub logs: LogsClient,
}

impl MembersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            logs: LogsClient::new(config.clone())?,
        })
    }

    /// Lists the members of an account. A member is one buyer's relationship with the account, regardless of how many memberships they hold.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account to list members for (`biz_` tag). Defaults to the account the credential acts as.
    /// * `access_level` - Filter by what the member can reach on the account.
    /// * `status` - Filter by whether the member is still part of the account.
    /// * `query` - Search members by name or username. An exact email address also matches when the credential holds the member:email:read scope.
    /// * `created_after` - Only members who joined after this ISO 8601 timestamp.
    /// * `created_before` - Only members who joined before this ISO 8601 timestamp.
    /// * `order` - Sort field.
    /// * `direction` - Sort direction.
    /// * `first` - Number of members to return from the start of the window.
    /// * `after` - Cursor to paginate forwards from.
    /// * `last` - Number of members to return from the end of the window.
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
    ///         .members
    ///         .list(
    ///             &MembersListQueryRequest {
    ///                 ..Default::default()
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
        self.http_client
            .execute_request(
                Method::GET,
                "members",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("access_level", request.access_level.clone())
                    .serialize("status", request.status.clone())
                    .structured_query("query", request.query.clone())
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
        self.http_client
            .execute_request(Method::GET, &format!("members/{}", id), None, None, options)
            .await
    }
}
