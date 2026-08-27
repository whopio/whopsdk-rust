use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EarningsClient {
    pub http_client: HttpClient,
}

impl EarningsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the earnings Whop pays out for one referred business's activity, most recent first.
    ///
    /// # Arguments
    ///
    /// * `id` - The partner business ID (a coma_ identifier).
    /// * `status` - Filter by earning status.
    /// * `income_source` - Filter to earnings from these income sources. Repeat the parameter for each one (income_source=sales&income_source=ad_spend).
    /// * `order` - The field to sort earnings by.
    /// * `direction` - Sort direction.
    /// * `created_before` - Only return earnings created before this timestamp.
    /// * `created_after` - Only return earnings created after this timestamp.
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
    ///         .partners
    ///         .businesses
    ///         .earnings
    ///         .list(
    ///             &"id".to_string(),
    ///             &PartnersBusinessesEarningsListQueryRequest {
    ///                 status: None,
    ///                 income_source: vec![],
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        id: &str,
        request: &PartnersBusinessesEarningsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEarningsResponse, ApiError> {
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
                &format!("partners/businesses/{}/earnings", id),
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
                    .serialize_array("income_source", request.income_source.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }
}
