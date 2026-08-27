use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub mod earnings;
pub use earnings::EarningsClient;
pub struct BusinessesClient {
    pub http_client: HttpClient,
    pub earnings: EarningsClient,
}

impl BusinessesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            earnings: EarningsClient::new(config.clone())?,
        })
    }

    /// Lists the businesses the authenticated user referred onto Whop, most recent first.
    ///
    /// # Arguments
    ///
    /// * `status` - Filter by referral status.
    /// * `has_earnings` - When true, only businesses with pending or completed earnings paid to the caller.
    /// * `first` - Number of partner businesses to return from the start of the window.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - Number of partner businesses to return from the end of the window.
    /// * `before` - Cursor to fetch the page before (from page_info.start_cursor).
    /// * `order` - The field to sort partner businesses by.
    /// * `direction` - Sort direction.
    /// * `created_before` - Only return partner businesses created before this timestamp.
    /// * `created_after` - Only return partner businesses created after this timestamp.
    /// * `referred_user_id` - Filter to referrals attributed to this user. For first-tier referrals, this is the referred account owner; for second-tier referrals, this is the partner you recruited.
    /// * `referred_username` - Filter by the referred user's exact username. Ignored when `referred_user_id` is present.
    /// * `tier` - Filter to referrals from a single tier: first, second, or blueprint.
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
    ///         .list(
    ///             &PartnersBusinessesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PartnersBusinessesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBusinessesResponse, ApiError> {
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
                "partners/businesses",
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
                    .bool("has_earnings", request.has_earnings.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .string("referred_user_id", request.referred_user_id.clone())
                    .string("referred_username", request.referred_username.clone())
                    .serialize("tier", request.tier.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a single referred business and its referral terms.
    ///
    /// # Arguments
    ///
    /// * `id` - The partner business ID (a coma_ identifier).
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
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveBusinessesResponse, ApiError> {
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
                &format!("partners/businesses/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
