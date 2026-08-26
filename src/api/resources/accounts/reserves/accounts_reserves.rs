use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ReservesClient {
    pub http_client: HttpClient,
}

impl ReservesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists what the account's held balance is made of, one entry per currency: the total held, why each part is held, and the days it unlocks.
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
    ///         .reserves
    ///         .list(&"account_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        account_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ListReservesResponse, ApiError> {
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
                &format!("accounts/{}/reserves", account_id),
                None,
                None,
                options,
            )
            .await
    }
}
