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

    /// Lists all recommended actions Economic Intelligence has generated for the account, newest first. Filter with `status=ready` for actions that are current.
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
                .or_insert_with(|| "2026-09-13".to_string());
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

    /// Harnesses Economic Intelligence to generate recommended actions that lead the business down the most optimal path to the next dollar. Returns a `queued` recommendation right away. Poll the list endpoint until it is `ready` or `failed`.
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
    ///     client.economic_intelligence.create(&CreateEconomicIntelligenceRequest {
    ///         input: "I sell $79 customized gym straps. The number of purchases per day fell from 84 to 66 since June and my ads cost per signup doubled to $38. Half the leads never open the checkout. I want to win back churned visitors and lift conversion without cutting the price, and I can spend up to $500 this month on it.".to_string(),
    ///         account_id: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateEconomicIntelligenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<EconomicIntelligence, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-13".to_string());
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

    /// Records approval with `executed`, or retires an unwanted recommendation with `superseded`. Both replenish the ready inventory. Supplying a rejection reason also allows retiring an executed recommendation.
    ///
    /// # Arguments
    ///
    /// * `id` - Recommendation ID, prefixed `reca_`.
    /// * `account_id` - Account ID, prefixed `biz_`. Defaults to the API key's own account.
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
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateEconomicIntelligenceRequest {
    ///                 status: UpdateEconomicIntelligenceRequestStatus::Executed,
    ///                 account_id: None,
    ///                 reason: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateEconomicIntelligenceRequest,
        options: Option<RequestOptions>,
    ) -> Result<EconomicIntelligence, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-13".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("economic_intelligence/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
