use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct PreferencesClient {
    pub http_client: HttpClient,
}

impl PreferencesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves the account's preferences: a singleton settings document keyed by preference name.
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
    ///         .preferences
    ///         .retrieve(&"account_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        account_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrievePreferencesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("accounts/{}/preferences", account_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates the account's preferences. Each top-level key in the body replaces its stored value as a whole. Keys left out keep their current values.
    ///
    /// The required scope depends on which preferences you update.
    ///
    /// - `ad_campaign:create`: `ads_payment_methods`, `ads_reporting_currency`, `ads_scheduling_timezone`, `ads_triple_whale_integration`.
    /// - `payout:account:update`: `cards_auto_top_up`, `cards_notifications`, `covers_subaccount_negative_balances`.
    /// - `payment:dispute`: `dispute_fighter_enabled`.
    /// - `company:update`: `economic_intelligence_duration_key`, `economic_intelligence_auto_renew`, `review_request_emails`, `cancellation_emails`, `renewal_reminder_emails`, `failed_payment_emails`, `subaccount_notifications`, `subaccount_team_view_only`.
    /// - `company:manage_checkout`: `feedback_options`, `customer_tracking`.
    ///
    /// To update keys under more than one scope, the API key needs all of those scopes.
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
    ///         .preferences
    ///         .update(
    ///             &"account_id".to_string(),
    ///             &UpdatePreferencesRequest {
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
        request: &UpdatePreferencesRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdatePreferencesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::PATCH,
                &format!("accounts/{}/preferences", account_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
