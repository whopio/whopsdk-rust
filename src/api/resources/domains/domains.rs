use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct DomainsClient {
    pub http_client: HttpClient,
}

impl DomainsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists your domains. Filter by account, app, status, hostname, or the state of a capability.
    ///
    /// Pass `search` to find domains to buy instead: the exact domain first, even when taken, then your name on popular extensions, then suggestions. Pass `tlds` to check only the extensions you choose. Results aren't reserved.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only domains belonging to this account, prefixed biz_.
    /// * `app_id` - Only domains whose website serves this app, prefixed app_.
    /// * `status` - Only domains with this status: the most pressing state of their capabilities, or `idle` when they want nothing.
    /// * `order` - Field to sort by. Domains with the same value keep their creation order in the same direction.
    /// * `direction` - Sort direction.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `search` - A name or domain to find domains to buy, such as `example` or `example.com`; a subdomain or URL searches its registrable domain. Returns domains with `availability` instead of your domains, without other filters or pagination.
    /// * `tlds` - With `search`, check only these extensions, such as `com` or `co.uk`, returned in this order. Repeat for several, up to 100.
    /// * `domain` - Only your domain with this hostname, such as `example.com`.
    /// * `verification` - Only domains with verification in this state, such as `ready`, or `any` for every domain that has verification.
    /// * `registration` - Only domains with registration in this state, such as `ready`, or `any` for every domain that has registration.
    /// * `platform` - Only domains with platform in this state, such as `ready`, or `any` for every domain that has platform.
    /// * `subdomain` - Only domains with subdomain in this state, such as `ready`, or `any` for every domain that has subdomain.
    /// * `website` - Only domains with website in this state, such as `ready`, or `any` for every domain that has website.
    /// * `apple_pay` - Only domains with apple_pay in this state, such as `ready`, or `any` for every domain that has apple_pay.
    /// * `pixel` - Only domains with pixel in this state, such as `ready`, or `any` for every domain that has pixel.
    /// * `email` - Only domains with email in this state, such as `ready`, or `any` for every domain that has email.
    /// * `transfer` - Only domains with transfer in this state, such as `ready`, or `any` for every domain that has transfer.
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
    ///         .domains
    ///         .list(
    ///             &DomainsListQueryRequest {
    ///                 tlds: vec![Some("com".to_string())],
    ///                 account_id: None,
    ///                 app_id: None,
    ///                 status: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 search: None,
    ///                 domain: None,
    ///                 verification: None,
    ///                 registration: None,
    ///                 platform: None,
    ///                 subdomain: None,
    ///                 website: None,
    ///                 apple_pay: None,
    ///                 pixel: None,
    ///                 email: None,
    ///                 transfer: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &DomainsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDomainsResponse, ApiError> {
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
                "domains",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("app_id", request.app_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .string("search", request.search.clone())
                    .string_array("tlds", request.tlds.clone())
                    .string("domain", request.domain.clone())
                    .serialize("verification", request.verification.clone())
                    .serialize("registration", request.registration.clone())
                    .serialize("platform", request.platform.clone())
                    .serialize("subdomain", request.subdomain.clone())
                    .serialize("website", request.website.clone())
                    .serialize("apple_pay", request.apple_pay.clone())
                    .serialize("pixel", request.pixel.clone())
                    .serialize("email", request.email.clone())
                    .serialize("transfer", request.transfer.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &DomainsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .string("app_id", request.app_id.clone())
            .serialize("status", request.status.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .string("search", request.search.clone())
            .string_array("tlds", request.tlds.clone())
            .string("domain", request.domain.clone())
            .serialize("verification", request.verification.clone())
            .serialize("registration", request.registration.clone())
            .serialize("platform", request.platform.clone())
            .serialize("subdomain", request.subdomain.clone())
            .serialize("website", request.website.clone())
            .serialize("apple_pay", request.apple_pay.clone())
            .serialize("pixel", request.pixel.clone())
            .serialize("email", request.email.clone())
            .serialize("transfer", request.transfer.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, cursor_value| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();
                if let Some(cursor) = cursor_value {
                    // Add cursor parameter based on pagination configuration
                    query_params.push(("after".to_string(), cursor));
                }
                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "domains",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction using pagination configuration
                    let items: Vec<serde_json::Value> = response
                        .get("data")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let next_cursor: Option<String> = response
                        .get("page_info")
                        .and_then(|v| v.get("end_cursor"))
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    let has_next_page = next_cursor.is_some();

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with no cursor
        )
    }

    /// Adds a domain to your account with the capabilities you want.
    ///
    /// Pass `registration` to buy the domain through Whop; it's the default when you pass no capability. Pay its `amount_due` at `purchase_url`, or pass `registration.payment_method_id` to charge a saved card. Whop then registers it, runs its DNS, and renews it every year while `auto_renew` is on.
    ///
    /// Pass `registration.auth_code` to transfer in a domain you own at another registrar. The transfer starts once its `amount_due` is paid, or at once when `amount_due` is `null`. While `registration.phase` is `transferring`, its `issues` say what the transfer needs from you.
    ///
    /// Pass `verification` to connect a domain you registered elsewhere: its `issues` list the TXT and routing records to publish. A subdomain of a domain this account bought through Whop takes `subdomain` by default, and Whop publishes its DNS records itself. Pass `website` with an `app_id` to serve that app on the domain.
    ///
    /// To change a domain you already have, update it instead. Adding a domain this account deleted revives it under its original ID.
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
    ///         .domains
    ///         .create(
    ///             &CreateDomainsRequest {
    ///                 domain: "www.bought.com".to_string(),
    ///                 account_id: None,
    ///                 apple_pay: None,
    ///                 email: None,
    ///                 metadata: None,
    ///                 pixel: None,
    ///                 registration: None,
    ///                 subdomain: None,
    ///                 transfer: None,
    ///                 verification: None,
    ///                 website: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateDomainsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
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
                Method::POST,
                "domains",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a domain by ID or hostname. Both return the same domain, shown as fully as you can see it: everything for your own accounts, and only who has it and what it serves for anyone else.
    ///
    /// A hostname no domain on Whop has comes back with its `availability` instead.
    ///
    /// # Arguments
    ///
    /// * `id` - Domain ID, prefixed `dom_`, or a hostname such as `example.com`; a bare name looks up `.com`.
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
    ///     client.domains.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
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
                &format!("domains/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Removes the domain from your account and releases its capabilities in the background. Deleting an unpaid purchase cancels it, and deleting a transfer cancels it and refunds it while the current registrar can still stop it. A registered domain can't be deleted; turn off `auto_renew` and it's released after it expires. Adding the domain to this account again revives it under the same ID.
    ///
    /// # Arguments
    ///
    /// * `id` - Domain ID, prefixed `dom_`, or a hostname such as `example.com`; a bare name looks up `.com`.
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
    ///     client.domains.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
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
                Method::DELETE,
                &format!("domains/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Changes a domain's capabilities or metadata. Pass a capability to add it or change its settings, or `null` to release it; capabilities you leave out don't change. Passing a capability that needs action again retries it. Releasing every capability keeps the domain, `idle`; delete it to remove it.
    ///
    /// # Arguments
    ///
    /// * `id` - Domain ID, prefixed `dom_`, or a hostname such as `example.com`; a bare name looks up `.com`.
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
    ///         .domains
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateDomainsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateDomainsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
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
                &format!("domains/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Checks the domain's DNS, payment, and provider state again now instead of at its next scheduled check. Returns the domain as saved; retrieve it again to see the result.
    ///
    /// # Arguments
    ///
    /// * `id` - Domain ID, prefixed `dom_`.
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
    ///     client.domains.check(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn check(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Domain, ApiError> {
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
                Method::POST,
                &format!("domains/{}/check", id),
                None,
                None,
                options,
            )
            .await
    }
}
