use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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
    /// * `first` - The number of disputes to return (default 20, max 100).
    /// * `after` - A cursor; returns disputes after this position.
    /// * `last` - The number of disputes to return from the end of the range.
    /// * `before` - A cursor; returns disputes before this position.
    /// * `order` - The field to sort disputes by.
    /// * `direction` - Sort direction.
    /// * `status` - Only disputes in these statuses. Repeat the parameter to pass several — one paginated list covers all of them. Covers both chargebacks and inquiries at each stage. A `needs_response` dispute whose evidence deadline has passed reports and filters as `under_review` instead.
    /// * `currency` - Only disputes in this three-letter ISO currency.
    /// * `created_before` - Only disputes opened before this ISO 8601 timestamp.
    /// * `created_after` - Only disputes opened after this ISO 8601 timestamp.
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
        self.http_client
            .execute_request(
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
                    .build(),
                options,
            )
            .await
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
        self.http_client
            .execute_request(
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
        self.http_client
            .execute_request(
                Method::GET,
                &format!("disputes/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Edits a dispute's evidence, while it is still editable. Sending it is a separate call.
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
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("disputes/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Sends a dispute's evidence to the payment processor. This is final — it cannot be edited or sent again.
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
        self.http_client
            .execute_request(
                Method::POST,
                &format!("disputes/{}/submit", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Submit a payment dispute to the payment processor for review. Once submitted, no further edits can be made.
    ///
    /// Required permissions:
    /// - `payment:dispute`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `company:basic:read`
    /// - `payment:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the dispute to submit to the payment processor for review.
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
    ///         .submit_evidence_dispute(&"dspt_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn submit_evidence_dispute(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Dispute, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("disputes/{}/submit_evidence", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Update a dispute with evidence data to attempt to win the dispute.
    ///
    /// Required permissions:
    /// - `payment:dispute`
    /// - `plan:basic:read`
    /// - `access_pass:basic:read`
    /// - `company:basic:read`
    /// - `payment:basic:read`
    /// - `member:email:read`
    /// - `member:basic:read`
    /// - `member:phone:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the dispute to update.
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
    ///         .update_evidence_dispute(
    ///             &"dspt_xxxxxxxxxxxxx".to_string(),
    ///             &UpdateEvidenceDisputeRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_evidence_dispute(
        &self,
        id: &str,
        request: &UpdateEvidenceDisputeRequest,
        options: Option<RequestOptions>,
    ) -> Result<Dispute, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("disputes/{}/update_evidence", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replaces the full set of uploaded evidence documents on a dispute, beyond the four fixed evidence slots. Send the files as multipart file parts to upload and attach in one call, or reference files already stored by `id`/`direct_upload_id`. Send every document the packet should carry — up to 10, 10MB each and 25MB in total; an empty list removes them all. Accepted content types: application/pdf, application/json, image/jpeg, image/png, image/webp — any other type is rejected.
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
        self.http_client
            .execute_request(
                Method::POST,
                &format!("disputes/{}/upload_evidence", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
