use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SupportedMethodsClient {
    pub http_client: HttpClient,
}

impl SupportedMethodsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the payout methods an account or user is eligible to add.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The owning account ID (a biz_ identifier). Provide this or user_id.
    /// * `user_id` - The owning user ID (a user_ identifier). Provide this or account_id.
    /// * `country` - ISO 3166-1 alpha-2 country code for the bank account or wallet, such as `US`. Defaults to the country of supported_payout_method_id when one is given, otherwise the payout account's country.
    /// * `amount` - Optional withdrawal amount in whole currency units, for example `250.00`. When provided, each destination includes per-currency fee and delivery quotes.
    /// * `currency` - Currency code of the amount, for example `usd`. Only meaningful with amount.
    /// * `supported_payout_method_id` - Narrows the list to one supported payout method (a podst_ identifier) and includes the required_fields needed to save it as a payout method.
    /// * `destination_currency` - Currency the supported payout method would deliver payouts in. Only meaningful with supported_payout_method_id; required fields vary by destination currency.
    /// * `first` - Number of supported payout methods to return from the start of the window.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - Number of supported payout methods to return from the end of the window.
    /// * `before` - Cursor to fetch the page before (from page_info.start_cursor).
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
    ///         .payouts
    ///         .supported_methods
    ///         .list(
    ///             &PayoutsSupportedMethodsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PayoutsSupportedMethodsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSupportedMethodsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "payouts/supported_methods",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("country", request.country.clone())
                    .float("amount", request.amount.clone())
                    .string("currency", request.currency.clone())
                    .string(
                        "supported_payout_method_id",
                        request.supported_payout_method_id.clone(),
                    )
                    .string("destination_currency", request.destination_currency.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }
}
