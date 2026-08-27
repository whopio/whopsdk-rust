use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct OauthGrantsClient {
    pub http_client: HttpClient,
}

impl OauthGrantsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the authenticated user's own OAuth grants — one per app they have authorized, per account they authorized it for. The list is always the caller's own; there is no parameter for reading another user's grants. Requires a user session: an API key or an OAuth token is refused, so an app can never enumerate the other apps a user has authorized.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Only return grants for this app, prefixed `app_`. An app the user has never authorized returns an empty list.
    /// * `first` - The number of grants to return (default 20, max 100).
    /// * `after` - A cursor; returns grants after this position.
    /// * `last` - The number of grants to return from the end of the range.
    /// * `before` - A cursor; returns grants before this position.
    /// * `order` - The field to sort grants by.
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
    ///         .users
    ///         .oauth_grants
    ///         .list(
    ///             &UsersOauthGrantsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &UsersOauthGrantsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOauthGrantsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "users/me/oauth_grants",
                None,
                QueryBuilder::new()
                    .string("app_id", request.app_id.clone())
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

    /// Completes the OAuth authorization step for the authenticated user: records their consent for the scopes an app asked for and mints the authorization code to hand back to it. Returns the grant, plus a `redirect_url` carrying that code — the one and only time it is returned. Exchange the code at `POST /oauth/token` with the verifier for `code_challenge`. Requires a user session, because consent has to come from the account holder: an API key or an OAuth token is refused, so an app can never authorize itself. Send an `Idempotency-Key` to make a retry safe — a replay returns the original `redirect_url` and its code rather than issuing a second one.
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
    ///         .users
    ///         .oauth_grants
    ///         .create(
    ///             &CreateOauthGrantsRequest {
    ///                 client_id: "app_xxxxxxxxxxxxxx".to_string(),
    ///                 code_challenge: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
    ///                 code_challenge_method: CreateOauthGrantsRequestCodeChallengeMethod::S256,
    ///                 redirect_uri: "https://Booking.Shinetime.example:8443/oauth/Callback/".to_string(),
    ///                 requested_scopes: vec!["profile".to_string()],
    ///                 account_id: None,
    ///                 consent_shown: None,
    ///                 nonce: None,
    ///                 response_type: None,
    ///                 state: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateOauthGrantsRequest,
        options: Option<RequestOptions>,
    ) -> Result<OauthGrant, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "users/me/oauth_grants",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
