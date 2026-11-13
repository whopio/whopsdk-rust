use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct DisputesClient {
    pub http_client: HttpClient,
}

impl DisputesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the disputes across the accounts you can read.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only disputes filed against this account (`biz_` tag). Omit it to cover every account you can read.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `order` - The field to sort disputes by.
    /// * `direction` - Sort direction.
    /// * `status` - Only disputes in these statuses. Repeat the parameter to pass several — one paginated list covers all of them. Inquiries match only the `warning_` statuses. A `needs_response` dispute whose evidence deadline has passed reports and filters as `under_review` instead.
    /// * `currency` - Only disputes in this three-letter ISO currency.
    /// * `created_before` - Only disputes opened before this ISO 8601 timestamp.
    /// * `created_after` - Only disputes opened after this ISO 8601 timestamp.
    /// * `query` - Search disputes by dispute ID, payment ID, user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    /// * `include_subaccounts` - Also cover the account's subaccounts. Requires `account_id`, and is a 400 without it. Without `query`, covers the disputes of the account and every subaccount, in any order. The listing is cut off after 10 seconds; add a `query` to narrow it. With `query`, matches on dispute, payment, user or membership ID, buyer email, or a card's first 6 or 8 digits and last 4 separated by a space or dash (`424242 4242`); free-text queries such as a name search only the named account. Subaccounts' disputes are returned only for subaccounts you can read disputes on.
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
    ///         .disputes
    ///         .list(
    ///             &DisputesListQueryRequest {
    ///                 account_id: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 status: vec![],
    ///                 currency: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 query: None,
    ///                 include_subaccounts: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &DisputesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListDisputesResponse, ApiError> {
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
                "disputes",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize_array("status", request.status.clone())
                    .string("currency", request.currency.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .structured_query("query", request.query.clone())
                    .bool("include_subaccounts", request.include_subaccounts.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &DisputesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .serialize_array("status", request.status.clone())
            .string("currency", request.currency.clone())
            .string("created_before", request.created_before.clone())
            .string("created_after", request.created_after.clone())
            .structured_query("query", request.query.clone())
            .bool("include_subaccounts", request.include_subaccounts.clone())
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
                            "disputes",
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

    /// Totals up the same disputes the list returns, so you can build status tabs and totals without paging through them.
    ///
    /// # Arguments
    ///
    /// * `groups` - Which breakdowns to return, keyed by these names under `groups`. Repeat the parameter to ask for several; omit it for all of them.
    /// * `account_id` - Only disputes filed against this account (`biz_` tag). Omit it to cover every account you can read.
    /// * `status` - Only disputes in these statuses. Repeat the parameter to pass several. A `needs_response` dispute whose evidence deadline has passed reports and filters as `under_review` instead.
    /// * `currency` - Only disputes in this three-letter ISO currency.
    /// * `created_before` - Only disputes opened before this ISO 8601 timestamp.
    /// * `created_after` - Only disputes opened after this ISO 8601 timestamp.
    /// * `query` - Search disputes by dispute ID, payment ID, user ID, membership ID, user email, name, or username. Email filtering requires the member:email:read permission.
    /// * `include_subaccounts` - Also cover the account's subaccounts. Requires `account_id`, and is a 400 without it. Without `query`, covers the disputes of the account and every subaccount, in any order. The listing is cut off after 10 seconds; add a `query` to narrow it. With `query`, matches on dispute, payment, user or membership ID, buyer email, or a card's first 6 or 8 digits and last 4 separated by a space or dash (`424242 4242`); free-text queries such as a name search only the named account. Subaccounts' disputes are returned only for subaccounts you can read disputes on.
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
    ///         .disputes
    ///         .summary(
    ///             &DisputesSummaryQueryRequest {
    ///                 groups: vec![],
    ///                 account_id: None,
    ///                 status: vec![],
    ///                 currency: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 query: None,
    ///                 include_subaccounts: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn summary(
        &self,
        request: &DisputesSummaryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SummaryDisputesResponse, ApiError> {
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
                "disputes/summary",
                None,
                QueryBuilder::new()
                    .serialize_array("groups", request.groups.clone())
                    .string("account_id", request.account_id.clone())
                    .serialize_array("status", request.status.clone())
                    .string("currency", request.currency.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .structured_query("query", request.query.clone())
                    .bool("include_subaccounts", request.include_subaccounts.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a single dispute.
    ///
    /// # Arguments
    ///
    /// * `id` - The dispute ID (`dspt_` tag).
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
    ///     client.disputes.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Dispute, ApiError> {
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
                &format!("disputes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Edits a dispute's evidence while it is still editable. When the evidence is ready, send it to the payment processor with `POST /disputes/:id/submit`.
    ///
    /// # Arguments
    ///
    /// * `id` - The dispute ID (`dspt_` tag).
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
    ///         .disputes
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateDisputesRequest {
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
        request: &UpdateDisputesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Dispute, ApiError> {
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
                &format!("disputes/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Accepts a chargeback, conceding it to the customer instead of contesting it. The payment processor closes the dispute as `lost`. This is final — the dispute cannot be contested afterwards. Only a dispute whose `acceptable` is true can be accepted.
    ///
    /// # Arguments
    ///
    /// * `id` - The dispute ID (`dspt_` tag).
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
    ///     client.disputes.accept(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn accept(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Dispute, ApiError> {
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
                &format!("disputes/{}/accept", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Sends a dispute's evidence to the payment processor. The packet is checked against the processor's limits first, so a file over a cap is refused here with the file and the limit named. Otherwise the dispute is returned as `generating_response` with its evidence locked and the filing completes in the background: the dispute moves to `under_review` with `evidence_submitted_at` set once the processor accepts it, or back to `needs_response` with `submission_failure_reason` set when the processor refuses it, so it can be fixed and sent again before `evidence_due_at`. Either move is sent as `dispute.updated`.
    ///
    /// # Arguments
    ///
    /// * `id` - The dispute ID (`dspt_` tag).
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
    ///     client.disputes.submit(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn submit(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Dispute, ApiError> {
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
                &format!("disputes/{}/submit", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Replaces the full set of uploaded evidence documents on a dispute, beyond the four fixed evidence slots. Prefer `PATCH /disputes/:id` with `evidence.documents`, which does the same replace alongside every other evidence field in one call.
    ///
    /// # Arguments
    ///
    /// * `id` - The dispute ID (`dspt_` tag).
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
    ///         .disputes
    ///         .upload_evidence(
    ///             &"id".to_string(),
    ///             &UploadEvidenceDisputesRequest {
    ///                 documents: vec![UploadEvidenceDisputesRequestDocumentsItem {
    ///                     direct_upload_id: None,
    ///                     document_type:
    ///                         UploadEvidenceDisputesRequestDocumentsItemDocumentType::ReturnPolicy,
    ///                     file: None,
    ///                     id: None,
    ///                 }],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload_evidence(
        &self,
        id: &str,
        request: &UploadEvidenceDisputesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Dispute, ApiError> {
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
                &format!("disputes/{}/upload_evidence", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
