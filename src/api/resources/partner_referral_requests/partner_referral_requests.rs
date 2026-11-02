use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PartnerReferralRequestsClient {
    pub http_client: HttpClient,
}

impl PartnerReferralRequestsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists requests sent by an eligible partner and requests for accounts where the authenticated user currently holds the owner role. Filters narrow that combined view. Use a Whop login session or an account API key with `partner:referral_request:read`. The key must have been created by the account's current owner. Account API keys return their owner's sent requests and incoming requests for the key's account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only requests for this business ID, prefixed `biz_`.
    /// * `partner_id` - Only requests sent by this partner's user ID, prefixed `user_`.
    /// * `status` - Only requests with this approval status.
    /// * `request_type` - Only requests initiated in this way.
    /// * `order` - Field used to sort requests.
    /// * `direction` - Sort direction.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
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
    ///         .partner_referral_requests
    ///         .list(
    ///             &PartnerReferralRequestsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PartnerReferralRequestsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPartnerReferralRequestsResponse, ApiError> {
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
                "partner_referral_requests",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("partner_id", request.partner_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("request_type", request.request_type.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a pending manual request for an existing business as the authenticated, enrolled, verified Whop partner. Provide exactly one of account_id or account_url. Whop business and product links resolve to their business. A business owner must accept before attribution changes. An existing pending manual request from the same partner returns 200; a new request returns 201. Use a Whop login session or an account API key with `partner:referral_request:create`. The key must have been created by the account's current owner. Account API keys submit requests as their account owner, who must be enrolled, verified, and not suspended.
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
    ///     client.partner_referral_requests.create(&CreatePartnerReferralRequestsRequestBody::CreatePartnerReferralRequestsRequestBodyAccountID(CreatePartnerReferralRequestsRequestBodyAccountID {
    ///         account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///         ..Default::default()
    ///     }), None).await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePartnerReferralRequestsRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "partner_referral_requests",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a request visible to its eligible sender or a current owner of the receiving account. Use a Whop login session or an account API key with `partner:referral_request:read`. The key must have been created by the account's current owner. Account API keys can retrieve their owner's sent requests and incoming requests for the key's account.
    ///
    /// # Arguments
    ///
    /// * `id` - Partner referral request ID, prefixed `prfr_`.
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
    ///         .partner_referral_requests
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
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
                &format!("partner_referral_requests/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Accepts a pending manual request as a current business owner and attributes the business to the verified requesting partner. Existing active attribution blocks acceptance. Repeating acceptance returns the accepted request. Use a Whop login session or an account API key with `partner:referral_request:accept`. The key must have been created by the account's current owner. Account API keys can respond only to requests for the key's account.
    ///
    /// # Arguments
    ///
    /// * `id` - Partner referral request ID, prefixed `prfr_`.
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
    ///         .partner_referral_requests
    ///         .accept(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn accept(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("partner_referral_requests/{}/accept", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancels a pending manual request as its eligible requesting partner. Repeating cancellation returns the cancelled request. Use a Whop login session or an account API key with `partner:referral_request:cancel`. The key must have been created by the account's current owner. Account API keys cancel requests as their account owner.
    ///
    /// # Arguments
    ///
    /// * `id` - Partner referral request ID, prefixed `prfr_`.
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
    ///         .partner_referral_requests
    ///         .cancel(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("partner_referral_requests/{}/cancel", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Denies a pending manual request as a current business owner. Repeating denial returns the denied request. Use a Whop login session or an account API key with `partner:referral_request:decline`. The key must have been created by the account's current owner. Account API keys can respond only to requests for the key's account.
    ///
    /// # Arguments
    ///
    /// * `id` - Partner referral request ID, prefixed `prfr_`.
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
    ///         .partner_referral_requests
    ///         .decline(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn decline(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PartnerReferralRequest, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("partner_referral_requests/{}/decline", id),
                None,
                None,
                options,
            )
            .await
    }
}
