use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EconomicIntelligenceClient {
    pub http_client: HttpClient,
}

impl EconomicIntelligenceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists every recommendation the account has been given, newest first: requests the engine is still answering, cards ready to run, cards already run, and cards a newer one replaced. `status=ready` is what the dashboard shows; an account with nothing ready gets `generation_pending` true while cards are being generated, so poll until it clears.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account ID, prefixed `biz_`. Defaults to the API key's own account.
    /// * `status` - Only recommendations in this state. `ready` for the cards the owner can run now.
    /// * `first` - The number of recommendations to return (default 20, max 100).
    /// * `after` - A cursor; returns recommendations after this position.
    /// * `last` - The number of recommendations to return from the end of the range.
    /// * `before` - A cursor; returns recommendations before this position.
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
    ///         .economic_intelligence
    ///         .list(
    ///             &EconomicIntelligenceListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &EconomicIntelligenceListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEconomicIntelligenceResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-11".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "economic_intelligence",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Runs the engine toward what the owner wants, in their own words, such as "get more repeat buyers for my taurine supplement". The recommendation comes back right away with status `queued` and only the owner's `input` filled in; the engine moves it to `pending` while it works and then to `ready`, with the title and brief written, or to `failed`. Watch it in the list.
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
    ///         .economic_intelligence
    ///         .run(
    ///             &RunEconomicIntelligenceRequest {
    ///                 input: "get more repeat buyers for my taurine supplement".to_string(),
    ///                 account_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn run(
        &self,
        request: &RunEconomicIntelligenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<EconomicIntelligence, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-11".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "economic_intelligence",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
