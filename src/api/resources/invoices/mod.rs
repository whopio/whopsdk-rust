use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod sends;
pub use sends::SendsClient;
pub struct InvoicesClient {
    pub http_client: HttpClient,
    pub sends: SendsClient,
}

impl InvoicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            sends: SendsClient::new(config.clone())?,
        })
    }

    /// Lists receipt-backed invoices for accounts the caller can read. Legacy invoices are excluded.
    ///
    /// # Arguments
    ///
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `include_total_count` - Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
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
    ///         .invoices
    ///         .list(
    ///             &InvoicesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &InvoicesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListInvoicesResponse, ApiError> {
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
                "invoices",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .bool("overdue", request.overdue.clone())
                    .string("member_id", request.member_id.clone())
                    .string("product_id", request.product_id.clone())
                    .datetime("created_after", request.created_after.clone())
                    .datetime("created_before", request.created_before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .bool("include_total_count", request.include_total_count.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &InvoicesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .serialize("status", request.status.clone())
            .bool("overdue", request.overdue.clone())
            .string("member_id", request.member_id.clone())
            .string("product_id", request.product_id.clone())
            .datetime("created_after", request.created_after.clone())
            .datetime("created_before", request.created_before.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .bool("include_total_count", request.include_total_count.clone())
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
                            "invoices",
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

    /// Creates a draft invoice backed by a receipt, with saved items, quantities, prices, and recipients. Issue the draft to resolve final tax and open the invoice. The pay_ ID stays the same when paid. Needs `invoice:create` or `payment:charge`; a `payment_method_id` needs `payment:charge`.
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
    ///         .invoices
    ///         .create(
    ///             &CreateInvoicesRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 items: vec![CreateInvoicesRequestItemsItem {
    ///                     ..Default::default()
    ///                 }],
    ///                 billing_address: None,
    ///                 buyer_fee_percentage: None,
    ///                 customer_name: None,
    ///                 due_at: None,
    ///                 email_address: None,
    ///                 mailing_address_id: None,
    ///                 member_id: None,
    ///                 memo: None,
    ///                 metadata: None,
    ///                 payment_method_configuration: None,
    ///                 payment_method_id: None,
    ///                 recipient_emails: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateInvoicesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                "invoices",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a receipt-backed invoice by its pay_ ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///     client.invoices.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                &format!("invoices/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a draft or void invoice. A deleted invoice no longer appears in invoice or payment lists, and its ID returns a `404`. Deleting can't be undone. Void an open invoice before you delete it. A draft with a scheduled send or charge must be unscheduled first. A draft renewal invoice can't be deleted, and neither can a void invoice that collected money or still awaits a bank wire. The invoice's `deletable` field is `true` when this call can delete it.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///     client.invoices.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteInvoicesResponse, ApiError> {
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
                &format!("invoices/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates draft invoice items and billing fields. Updates of only `recipient_emails` and `memo` are also allowed on unpaid invoices. The customer is set when the draft is created and can't change. To bill someone else, first void the draft. Then create a new invoice. Edit customer contact details through the member API. A draft with no saved payment method and no scheduled send needs `invoice:update`, `invoice:create`, or `payment:charge`; every other update, and setting `payment_method_id`, needs `payment:charge`.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///         .invoices
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateInvoicesRequest {
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
        request: &UpdateInvoicesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                &format!("invoices/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Charges an open invoice now to one of its customer's saved payment methods, canceling any charge scheduled for later. The charge completes asynchronously: the invoice becomes `paid`, or, if declined, stays `open` and its recipients are emailed the `checkout_link`. Refused for drafts and while another payment is in progress.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///         .invoices
    ///         .charge(
    ///             &"id".to_string(),
    ///             &ChargeInvoicesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn charge(
        &self,
        id: &str,
        request: &ChargeInvoicesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                &format!("invoices/{}/charge", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generates the invoice as a PDF and returns a short-lived link to download it. A draft shows its estimated totals. The buyer's name and email need `member:basic:read` and `member:email:read`, and the Pay now link on an open invoice needs `payment:charge`, or `invoice:create` or `invoice:update` when it has no `payment_method_id`. Each call generates a new file and link, so this endpoint does not replay `Idempotency-Key` responses.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///         .invoices
    ///         .generate_pdf(
    ///             &"id".to_string(),
    ///             &GeneratePdfInvoicesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn generate_pdf(
        &self,
        id: &str,
        request: &GeneratePdfInvoicesRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoicePdf, ApiError> {
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
                &format!("invoices/{}/generate_pdf", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Resolves final tax and opens a draft invoice without emailing it. An invoice with a `payment_method_id` is charged to it off-session, as `POST /payments` charges a saved payment method: at `charge_at` if given, otherwise on `due_at` while it is still ahead, otherwise right away (see `scheduled_charge_at`). With a future `due_at` and no `charge_at`, the customer is charged on the due date without being emailed first. Issuing an open invoice again retains its totals and charges nothing.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///         .invoices
    ///         .issue(
    ///             &"id".to_string(),
    ///             &IssueInvoicesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn issue(
        &self,
        id: &str,
        request: &IssueInvoicesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                &format!("invoices/{}/issue", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Marks an open invoice as uncollectible. Its checkout link stops working.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///         .invoices
    ///         .mark_uncollectible(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn mark_uncollectible(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                &format!("invoices/{}/mark_uncollectible", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Issues a draft and emails it to its recipients, with reminders after the due date. A draft with no `payment_method_id` needs `invoice:update`, `invoice:create`, or `payment:charge`, and is never charged. A draft with a `payment_method_id` needs `payment:charge`: with a `charge_at`, or when its `due_at` is still ahead, it is emailed now and charged to that method at that time (see `scheduled_charge_at`); otherwise it is charged right away instead of emailed. Voiding the invoice, marking it uncollectible, or paying it first cancels the scheduled charge, and its recipients get the payment link if a charge fails. Sending an open invoice needs `payment:charge` and emails it again, up to three emails a day including the first send. With `send_at`, the draft is sent at that time instead, and cannot be edited, except its recipients, until it is sent or unscheduled. Each email, reminders included, is sent separately to the customer email shown on the invoice and to each address in `recipient_emails`, so recipients never see each other's addresses.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///         .invoices
    ///         .send(
    ///             &"id".to_string(),
    ///             &SendInvoicesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send(
        &self,
        id: &str,
        request: &SendInvoicesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                &format!("invoices/{}/send", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Cancels a draft's scheduled send, so it can be edited again. An invoice with no scheduled send is returned unchanged. A draft with no `payment_method_id` needs `invoice:update`, `invoice:create`, or `payment:charge`; any other invoice needs `payment:charge`.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///     client.invoices.unschedule(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unschedule(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                &format!("invoices/{}/unschedule", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Voids a draft or unpaid invoice.
    ///
    /// # Arguments
    ///
    /// * `id` - The invoice's pay_ ID.
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
    ///     client.invoices.void(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn void(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
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
                &format!("invoices/{}/void", id),
                None,
                None,
                options,
            )
            .await
    }
}
