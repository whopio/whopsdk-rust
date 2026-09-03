use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct LedgerAccountsClient {
    pub http_client: HttpClient,
}

impl LedgerAccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves the details of an existing ledger account.
    ///
    /// Required permissions:
    /// - `company:balance:read`
    /// - `payout:account:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The identifier to look up the ledger account. Accepts a user ID ('user_xxx'), company ID ('biz_xxx'), or ledger account ID ('ldgr_xxx').
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
    ///         .ledger_accounts
    ///         .retrieve(&"ldgr_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<LedgerAccount, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("ledger_accounts/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
