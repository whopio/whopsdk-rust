use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct IdentityProfilesClient {
    pub http_client: HttpClient,
}

impl IdentityProfilesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of identity profiles. When company_id is provided, lists IPs currently linked to that company's ledger. When omitted, lists IPs linked to any ledger the actor can read (including child companies under a parent).
    ///
    /// Required permissions:
    /// - `identity:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to filter to. When omitted, returns IPs across all ledgers the actor can read.
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
    ///         .identity_profiles
    ///         .list_identity_profile(
    ///             &ListIdentityProfileQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_identity_profile(
        &self,
        request: &ListIdentityProfileQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListIdentityProfileResponse, ApiError> {
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
                "identity_profiles",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .serialize("profile_type", request.profile_type.clone())
                    .serialize("status", request.status.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the details of an existing identity profile.
    ///
    /// Required permissions:
    /// - `identity:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the identity profile (idpf_xxx).
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
    ///         .identity_profiles
    ///         .retrieve_identity_profile(&"idpf_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve_identity_profile(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<IdentityProfile, ApiError> {
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
                &format!("identity_profiles/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Unlinks an IdentityProfile from a LedgerAccount (flips the matching link to is_current=false).
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the IdentityProfile to unlink.
    /// * `ledger_account_id` - The ID of the LedgerAccount to unlink the identity profile from.
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
    ///         .identity_profiles
    ///         .unlink_identity_profile(
    ///             &"idpf_xxxxxxxxxxxxx".to_string(),
    ///             &UnlinkIdentityProfileQueryRequest {
    ///                 ledger_account_id: "ldgr_xxxxxxxxxxxxx".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn unlink_identity_profile(
        &self,
        id: &str,
        request: &UnlinkIdentityProfileQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<IdentityProfile, ApiError> {
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
                &format!("identity_profiles/{}", id),
                None,
                QueryBuilder::new()
                    .string("ledger_account_id", request.ledger_account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a list of verifications attached to an identity profile, ordered by most recent first.
    ///
    /// Required permissions:
    /// - `identity:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the identity profile (idpf_xxx).
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
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
    ///         .identity_profiles
    ///         .list_verifications_identity_profile(
    ///             &"idpf_xxxxxxxxxxxxx".to_string(),
    ///             &ListVerificationsIdentityProfileQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_verifications_identity_profile(
        &self,
        id: &str,
        request: &ListVerificationsIdentityProfileQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListVerificationsIdentityProfileResponse, ApiError> {
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
                &format!("identity_profiles/{}/verifications", id),
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .build(),
                options,
            )
            .await
    }
}
