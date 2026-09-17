use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct FeesClient {
    pub http_client: HttpClient,
}

impl FeesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves every fee the account is charged, as a document keyed by fee: Whop's fees, resolved the way they are charged, and any markups the platform the account is connected to adds on top. The account's own team, the Whop Verified Partner who referred it, and the platform it is connected to all read the same document; `adjustable` on each fee says what the caller may change through `PATCH`.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account ID, prefixed `biz_`.
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
    ///         .fees
    ///         .retrieve(&"account_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        account_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AccountFees, ApiError> {
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
                &format!("accounts/{}/fees", account_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Changes fees on the account. The body mirrors the document: send only the keys to change, and each is replaced while the rest stay as they are. A platform sets `markups` on an account connected to it, or `child_markups` on itself for every connected account. A Whop Verified Partner edits the fee schedule of a business they referred, with `notes`, from a first-party Whop session. Every change is validated against the document before anything is written, and a rejected request names the key. Returns the full document.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account ID, prefixed `biz_`.
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
    ///         .fees
    ///         .update(
    ///             &"account_id".to_string(),
    ///             &UpdateFeesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        account_id: &str,
        request: &UpdateFeesRequest,
        options: Option<RequestOptions>,
    ) -> Result<AccountFees, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("accounts/{}/fees", account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
