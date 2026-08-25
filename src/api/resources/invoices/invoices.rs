use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InvoicesClient {
    pub http_client: HttpClient,
}

impl InvoicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of invoices for a company, with optional filtering by product, status, collection method, and creation date.
    ///
    /// Required permissions:
    /// - `invoice:basic:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `company_id` - The unique identifier of the company to list invoices for.
    /// * `product_ids` - Filter invoices to only those associated with these specific product identifiers.
    /// * `collection_methods` - Filter invoices by their collection method.
    /// * `statuses` - Filter invoices by their current status.
    /// * `created_before` - Only return invoices created before this timestamp.
    /// * `created_after` - Only return invoices created after this timestamp.
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
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 after: None,
    ///                 before: None,
    ///                 direction: None,
    ///                 product_ids: vec![],
    ///                 collection_methods: vec![],
    ///                 statuses: vec![],
    ///                 order: None,
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "invoices",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .string("company_id", request.company_id.clone())
                    .serialize("direction", request.direction.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .serialize_array("collection_methods", request.collection_methods.clone())
                    .serialize_array("statuses", request.statuses.clone())
                    .serialize("order", request.order.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Create an invoice for a customer. The invoice can be charged automatically using a stored payment method, or sent to the customer for manual payment.
    ///
    /// Required permissions:
    /// - `invoice:create`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `payment:basic:read`
    ///
    /// # Arguments
    ///
    /// * `request` - Parameters for CreateInvoice
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
    ///             &CreateInvoicesRequestBody::CreateInvoicesRequestBodyProduct(
    ///                 CreateInvoicesRequestBodyProduct {
    ///                     automatically_finalizes_at: None,
    ///                     billing_address: None,
    ///                     charge_buyer_fee: None,
    ///                     collection_method: InvoiceCollectionMethods::SendInvoice,
    ///                     company_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                     customer_name: None,
    ///                     due_date: None,
    ///                     email_address: None,
    ///                     line_items: None,
    ///                     mailing_address_id: None,
    ///                     member_id: None,
    ///                     payment_method_id: None,
    ///                     payment_token_id: None,
    ///                     plan: CreateInvoicesRequestBodyProductPlan {
    ///                         ..Default::default()
    ///                     },
    ///                     product: CreateInvoicesRequestBodyProductProduct {
    ///                         title: "title".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     save_as_draft: None,
    ///                     subscription_billing_anchor_at: None,
    ///                 },
    ///             ),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateInvoicesRequestBody,
        options: Option<RequestOptions>,
    ) -> Result<Invoice, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "invoices",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing invoice.
    ///
    /// Required permissions:
    /// - `invoice:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `payment:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the invoice, or a secure token.
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
    ///         .retrieve(&"inv_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("invoices/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Delete a draft invoice.
    ///
    /// Required permissions:
    /// - `invoice:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the draft invoice to delete.
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
    ///         .delete(&"inv_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("invoices/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a draft invoice's details.
    ///
    /// Required permissions:
    /// - `invoice:update`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `payment:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the invoice to update.
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
    ///             &"inv_xxxxxxxxxxxxxx".to_string(),
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("invoices/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Mark an open invoice as paid when payment was collected outside of Whop.
    ///
    /// Required permissions:
    /// - `invoice:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the invoice to mark as paid.
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
    ///         .mark_paid(&"inv_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn mark_paid(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("invoices/{}/mark_paid", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Mark an open invoice as uncollectible when payment is not expected.
    ///
    /// Required permissions:
    /// - `invoice:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the invoice to mark as uncollectible.
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
    ///         .mark_uncollectible(&"inv_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn mark_uncollectible(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("invoices/{}/mark_uncollectible", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resend the notification email for an existing invoice to the customer.
    ///
    /// Required permissions:
    /// - `invoice:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the invoice to resend.
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
    ///         .resend(&"inv_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn resend(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("invoices/{}/resend", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Void an open invoice so it can no longer be paid. Voiding is permanent and cannot be undone.
    ///
    /// Required permissions:
    /// - `invoice:update`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the invoice to void.
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
    ///         .void(&"inv_xxxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn void(&self, id: &str, options: Option<RequestOptions>) -> Result<bool, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("invoices/{}/void", id),
                None,
                None,
                options,
            )
            .await
    }
}
