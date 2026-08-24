use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod preferences;
pub use preferences::PreferencesClient;
pub mod reserves;
pub use reserves::ReservesClient;
pub struct AccountsClient {
    pub http_client: HttpClient,
    pub preferences: PreferencesClient,
    pub reserves: ReservesClient,
}

impl AccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            preferences: PreferencesClient::new(config.clone())?,
            reserves: ReservesClient::new(config.clone())?,
        })
    }

    /// Lists accounts visible to the credential. User tokens return the user's business accounts; Account API keys return the requesting account and its connected accounts. Pass `parent_account_id` to return only that parent account's connected accounts.
    ///
    /// # Arguments
    ///
    /// * `first` - The number of accounts to return (default 10, max 50).
    /// * `after` - A cursor; returns accounts after this position.
    /// * `last` - The number of accounts to return from the end of the range.
    /// * `before` - A cursor; returns accounts before this position.
    /// * `order` - The field to sort accounts by. `volume` requires `stats:read` on the parent account.
    /// * `direction` - Sort direction.
    /// * `status` - Return only accounts with this status: `active` (includes accounts that have not entered payments review) or `suspended`.
    /// * `query` - Free-text filter on account title or ID. `%` and `_` match literally.
    /// * `created_after` - Return only accounts created after this ISO 8601 timestamp.
    /// * `created_before` - Return only accounts created before this ISO 8601 timestamp.
    /// * `volume_min` - Return only accounts whose lifetime USD volume is at least this value. Requires `stats:read` on the parent account.
    /// * `volume_max` - Return only accounts whose lifetime USD volume is at most this value. Requires `stats:read` on the parent account.
    /// * `parent_account_id` - For platforms: the parent account ID whose direct connected accounts to return. Requires `payout:account:read` on the parent account.
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
    ///         .accounts
    ///         .list(
    ///             &AccountsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "accounts",
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("status", request.status.clone())
                    .structured_query("query", request.query.clone())
                    .datetime("created_after", request.created_after.clone())
                    .datetime("created_before", request.created_before.clone())
                    .float("volume_min", request.volume_min.clone())
                    .float("volume_max", request.volume_max.clone())
                    .string("parent_account_id", request.parent_account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an account. User tokens create business accounts; Account API keys create connected accounts. Tax fields (`tax_remitted_by`, `tax_type`, `product_tax_code_id`, `business_address`, `tax_identifiers`) are configured with Update Account, not at creation.
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
    ///         .accounts
    ///         .create(
    ///             &CreateAccountsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "accounts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the account associated with the current Account API key.
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
    ///     client.accounts.me(None).await;
    /// }
    /// ```
    pub async fn me(&self, options: Option<RequestOptions>) -> Result<Account, ApiError> {
        self.http_client
            .execute_request(Method::GET, "accounts/me", None, None, options)
            .await
    }

    /// Retrieves a single account by ID or public route when it is visible to the credential, including its crypto wallet. The reserved id `me` retrieves the account associated with the current Account API key; user tokens have no single account, so they must address one by ID or route.
    ///
    /// # Arguments
    ///
    /// * `id` - Account ID, prefixed `biz_`, its public route, or `me` for the account associated with the current API key.
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
    ///     client.accounts.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("accounts/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an account. User tokens can update business accounts; Account API keys can update connected accounts. The reserved id `me` — accepted on Retrieve Account — resolves to the requesting account, which an Account API key cannot edit, so updates must name the connected account by its `biz_` id.
    ///
    /// # Arguments
    ///
    /// * `id` - Account ID, prefixed `biz_`.
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
    ///         .accounts
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAccountsRequest {
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
        request: &UpdateAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("accounts/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Starts an LLC or C-Corp formation for a business account. Defaults to an LLC; set `entity_type` to `c_corp` to form a C-Corp, which additionally requires `share_structure` and officer `roles` on every founder. On submission, the application is validated and the response returns a hosted checkout URL. Once paid, the filing is submitted. Track progress through the account's [`company_formation`](/api-reference/beta/accounts/retrieve-account) field on Retrieve Account.
    ///
    /// # Arguments
    ///
    /// * `id` - Account ID, prefixed `biz_`.
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
    ///         .accounts
    ///         .form_company(
    ///             &"id".to_string(),
    ///             &FormCompanyAccountsRequest {
    ///                 business_address: Some(FormCompanyAccountsRequestBusinessAddress {
    ///                     city: "Austin".to_string(),
    ///                     country: "US".to_string(),
    ///                     line1: "4180 Burnet Rd".to_string(),
    ///                     line2: Some("Suite 2".to_string()),
    ///                     postal_code: "78756".to_string(),
    ///                     state: "TX".to_string(),
    ///                     ..Default::default()
    ///                 }),
    ///                 business_name: "Shine Time Auto Detailing".to_string(),
    ///                 business_phone: Some("+15125550142".to_string()),
    ///                 business_type: "brick_and_mortar".to_string(),
    ///                 business_website: Some("https://shinetime.example".to_string()),
    ///                 entity_suffix: Some(FormCompanyAccountsRequestEntitySuffix::Llc),
    ///                 entity_type: Some(FormCompanyAccountsRequestEntityType::Llc),
    ///                 expedite_ein: Some(true),
    ///                 formation_state: FormCompanyAccountsRequestFormationState::Tx,
    ///                 founders: vec![FormCompanyAccountsRequestFoundersItem {
    ///                     address: FormCompanyAccountsRequestFoundersItemAddress {
    ///                         city: "Austin".to_string(),
    ///                         country: "US".to_string(),
    ///                         line1: "907 Ridgemont Dr".to_string(),
    ///                         line2: Some("Apt 4".to_string()),
    ///                         postal_code: "78704".to_string(),
    ///                         state: "TX".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     date_of_birth: Some("1988-03-14".to_string()),
    ///                     email: "marcus@shinetime.example".to_string(),
    ///                     first_name: "Marcus".to_string(),
    ///                     is_primary: true,
    ///                     last_name: "Webb".to_string(),
    ///                     ownership_percentage: Some(100.0),
    ///                     phone: "+15125550142".to_string(),
    ///                     roles: Some(vec![
    ///                         FormCompanyAccountsRequestFoundersItemRolesItem::President,
    ///                     ]),
    ///                     ssn: Some("123-45-6789".to_string()),
    ///                     ..Default::default()
    ///                 }],
    ///                 industry_group: "automotive".to_string(),
    ///                 industry_type: "car_wash".to_string(),
    ///                 share_structure: Some(FormCompanyAccountsRequestShareStructure {
    ///                     number_of_shares: 123,
    ///                     value: 123.0,
    ///                     ..Default::default()
    ///                 }),
    ///                 use_registered_agent: Some(true),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn form_company(
        &self,
        id: &str,
        request: &FormCompanyAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<FormCompanyAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("accounts/{}/form_company", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Transfers ownership of the account to another user, identified by user ID or email address. If the recipient already holds the owner role, ownership moves immediately; otherwise they get an invite and ownership moves when they accept.
    ///
    /// # Arguments
    ///
    /// * `id` - Account ID, prefixed `biz_`.
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
    ///         .accounts
    ///         .transfer_ownership(
    ///             &"id".to_string(),
    ///             &TransferOwnershipAccountsRequest {
    ///                 identifier: "marcus@shinetime.example".to_string(),
    ///                 as_partner: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn transfer_ownership(
        &self,
        id: &str,
        request: &TransferOwnershipAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransferOwnershipAccountsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("accounts/{}/transfer_ownership", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
