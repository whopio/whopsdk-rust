use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct SearchClient {
    pub http_client: HttpClient,
}

impl SearchClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Searches one account's records in one call. `account` rows are `account_id` itself and, when you send `include_subaccounts`, its subaccounts; every other row belongs to one of those accounts.
    ///
    /// Each `object` needs its own read permission on the record's account, and rows you can't read are left out without an error: `account` (`biz_`, `company:basic:read`), `member` (`mber_`, `member:basic:read`), `membership` (`mem_`, `member:basic:read`), `payment` (`pay_`, `payment:basic:read`), `affiliate` (`aff_`, `affiliate:basic:read`), `review` (`rev_`, `company:basic:read`), `resolution_center_case` (`reso_`, `payment:resolution_center_case:read`) and `promo_code` (`promo_`, `promo_code:basic:read`). A query containing `@` also needs `member:email:read`, and a phone match `member:phone:read`. An invoice's number finds the invoice as its `payment`.
    ///
    /// `query` is trimmed and has to be 2 to 250 characters; a full card number returns a `400`. `data` comes identifier matches first, such as a pasted ID, email address, card digits or invoice number, newest first within each kind of match. Every query of 3 or more characters is also ranked, and ranked rows follow, best match first. Each record appears once, and `match_reason` says why it matched. [Search](/api-reference/search/search) lists the query shapes and match reasons.
    ///
    /// `first` caps the rows, 20 by default and up to 100. There is no cursor yet, so the response is a single page. Search ranks; to filter one resource or page through every match, use its list endpoint, such as `GET /payments?query=`.
    ///
    /// # Arguments
    ///
    /// * `query` - Text to search for, 2 to 250 characters.
    /// * `types` - Kinds of records to search. Defaults to every kind. `account` rows are `account_id` itself and, with `include_subaccounts`, its subaccounts.
    /// * `account_id` - The account to search, prefixed `biz_`. Records other than accounts belong to it, or with `include_subaccounts` to its subaccounts. Your credential needs read access to at least one kind of record on it that search returns, or the account is a 404.
    /// * `include_subaccounts` - `true` also searches the subaccounts of a platform `account_id`: an ID, an email, card digits or a phone number finds their records, and the subaccounts themselves are found as `account` rows by name, ID or store link, or by the email of their owner, a team member or a customer. Safe to send for any account: one without subaccounts is searched as if this were false.
    /// * `first` - Number of results to return from the start of the range.
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
    ///         .search
    ///         .list(
    ///             &SearchListQueryRequest {
    ///                 query: "query".to_string(),
    ///                 account_id: "account_id".to_string(),
    ///                 types: vec![],
    ///                 include_subaccounts: None,
    ///                 first: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SearchListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSearchResponse, ApiError> {
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
                "search",
                None,
                QueryBuilder::new()
                    .structured_query("query", request.query.clone())
                    .serialize_array("types", request.types.clone())
                    .string("account_id", request.account_id.clone())
                    .bool("include_subaccounts", request.include_subaccounts.clone())
                    .int("first", request.first.clone())
                    .build(),
                options,
            )
            .await
    }
}
