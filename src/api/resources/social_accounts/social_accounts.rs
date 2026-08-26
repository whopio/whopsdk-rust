use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SocialAccountsClient {
    pub http_client: HttpClient,
}

impl SocialAccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the social accounts linked to an account or user.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The Account that the social accounts are connected to. Provide either this or user_id.
    /// * `user_id` - The User that the social accounts are connected to. Provide either this or account_id.
    /// * `platform` - Only return social accounts for the platform that is specified.
    /// * `verified` - Only return social accounts that are verified on the platform.
    /// * `scopes` - Only return social accounts that have these scopes.
    /// * `first` - The number of social accounts to return.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - The number of social accounts to return from the end of the range.
    /// * `before` - Cursor to fetch the page before (from page_info.start_cursor).
    /// * `order` - The field to sort social accounts by.
    /// * `direction` - Sort direction.
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
    ///         .social_accounts
    ///         .list(
    ///             &SocialAccountsListQueryRequest {
    ///                 account_id: None,
    ///                 user_id: None,
    ///                 platform: None,
    ///                 verified: None,
    ///                 scopes: vec![],
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 order: None,
    ///                 direction: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SocialAccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSocialAccountsResponse, ApiError> {
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
                "social_accounts",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .serialize("platform", request.platform.clone())
                    .bool("verified", request.verified.clone())
                    .serialize_array("scopes", request.scopes.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates or returns a Whop-managed Facebook page for an account.
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
    ///         .social_accounts
    ///         .create(
    ///             &CreateSocialAccountsRequest {
    ///                 platform: CreateSocialAccountsRequestPlatform::Facebook,
    ///                 account_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateSocialAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SocialAccount, ApiError> {
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
                "social_accounts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Starts an OAuth connection flow and returns an authorize_url where the user can connect a social account.
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
    ///         .social_accounts
    ///         .connect(
    ///             &ConnectSocialAccountsRequest {
    ///                 platform: ConnectSocialAccountsRequestPlatform::MetaBusiness,
    ///                 account_id: None,
    ///                 redirect_url: None,
    ///                 scopes: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connect(
        &self,
        request: &ConnectSocialAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConnectSocialAccountsResponse, ApiError> {
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
                "social_accounts/connect",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Disconnects a social account from an account or user without deleting the underlying platform account.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the social account to disconnect.
    /// * `account_id` - The Account that the social account is connected to. Provide either this or user_id.
    /// * `user_id` - The User that the social account is connected to. Provide either this or account_id.
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
    ///         .social_accounts
    ///         .delete(
    ///             &"id".to_string(),
    ///             &SocialAccountsDeleteQueryRequest {
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
        request: &SocialAccountsDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DeleteSocialAccountsResponse, ApiError> {
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
                &format!("social_accounts/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Lists the active lead (instant) forms that already exist on a connected Facebook page, so an ad can reuse one as its `lead_gen_form_id` instead of authoring a new form. Every active form comes back in a single response — the list is not paginated.
    ///
    /// # Arguments
    ///
    /// * `id` - The social account (a sacc_ identifier) whose lead forms to list.
    /// * `account_id` - The Account (a biz_ identifier) the social account is connected to.
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
    ///         .social_accounts
    ///         .lead_forms(
    ///             &"id".to_string(),
    ///             &LeadFormsQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn lead_forms(
        &self,
        id: &str,
        request: &LeadFormsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<LeadFormsSocialAccountsResponse, ApiError> {
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
                &format!("social_accounts/{}/lead_forms", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Lists the existing posts of a connected Facebook page, Instagram account, or TikTok account.
    ///
    /// # Arguments
    ///
    /// * `id` - The social account (a sacc_ identifier) whose posts to list.
    /// * `account_id` - The Account (a biz_ identifier) the social account is connected to.
    /// * `post_id` - Return only the single post with this platform id, instead of the full list.
    /// * `first` - The number of posts to return.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
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
    ///         .social_accounts
    ///         .posts(
    ///             &"id".to_string(),
    ///             &PostsQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 post_id: None,
    ///                 first: None,
    ///                 after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn posts(
        &self,
        id: &str,
        request: &PostsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostsSocialAccountsResponse, ApiError> {
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
                &format!("social_accounts/{}/posts", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("post_id", request.post_id.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }
}
