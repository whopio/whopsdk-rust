use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ResolutionCenterCasesClient {
    pub http_client: HttpClient,
}

impl ResolutionCenterCasesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists resolution center cases. Without `account_id` you get every case you can read — the ones you opened as a buyer and every account you are a team member of; the filters narrow that list.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only cases filed against this account (`biz_` tag). With read access to the account this lists its whole queue; without, only the cases you opened against it.
    /// * `user_id` - Only cases opened by this customer — a `user_` tag, or `me` for the calling user. It narrows what you can already read, so `me` lists the cases you opened without the ones on accounts you are a team member of.
    /// * `first` - The number of cases to return (default 20, max 100).
    /// * `after` - A cursor; returns cases after this position.
    /// * `last` - The number of cases to return from the end of the range.
    /// * `before` - A cursor; returns cases before this position.
    /// * `order` - The field to sort cases by.
    /// * `direction` - Sort direction.
    /// * `status` - Only cases in these statuses. Repeat the parameter to pass several — one paginated list covers all of them.
    /// * `reason` - Only cases opened for these reasons. Repeat the parameter to pass several.
    /// * `outcome` - Only closed cases that ended these ways. Repeat the parameter to pass several.
    /// * `created_before` - Only cases created before this ISO 8601 timestamp.
    /// * `created_after` - Only cases created after this ISO 8601 timestamp.
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
    ///         .resolution_center_cases
    ///         .list(
    ///             &ResolutionCenterCasesListQueryRequest {
    ///                 account_id: None,
    ///                 user_id: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 status: vec![],
    ///                 reason: vec![],
    ///                 outcome: vec![],
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
        request: &ResolutionCenterCasesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListResolutionCenterCasesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "resolution_center_cases",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize_array("status", request.status.clone())
                    .serialize_array("reason", request.reason.clone())
                    .serialize_array("outcome", request.outcome.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Opens a case, as the customer, against one of your own payments. Provide the payment (`receipt_id`), the `reason`, and a `message`.
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
    ///         .resolution_center_cases
    ///         .create(
    ///             &CreateResolutionCenterCasesRequest {
    ///                 message: "The mobile detailer never showed up for the Ceramic Coating appointment."
    ///                     .to_string(),
    ///                 reason: CreateResolutionCenterCasesRequestReason::Fraudulent,
    ///                 receipt_id: "pay_xxxxxxxxxxxxxx".to_string(),
    ///                 attachments: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateResolutionCenterCasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResolutionCenterCase, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "resolution_center_cases",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Aggregates the same cases `GET /resolution_center_cases` lists, using the same filters. Use it to build status tabs and issue filters without paging the whole list.
    ///
    /// # Arguments
    ///
    /// * `groups` - Which breakdowns to return, keyed by these names under `groups`. Repeat the parameter to ask for several; omit it for all of them.
    /// * `account_id` - The account to summarize cases for (`biz_` tag).
    /// * `user_id` - Only cases opened by this customer — a `user_` tag, or `me` for the calling user.
    /// * `status` - Only cases in these statuses.
    /// * `reason` - Only cases opened for these reasons.
    /// * `outcome` - Only closed cases that ended these ways.
    /// * `created_before` - Only count cases created before this ISO 8601 timestamp.
    /// * `created_after` - Only count cases created after this ISO 8601 timestamp.
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
    ///         .resolution_center_cases
    ///         .summary(
    ///             &ResolutionCenterCasesSummaryQueryRequest {
    ///                 groups: vec![],
    ///                 account_id: None,
    ///                 user_id: None,
    ///                 status: vec![],
    ///                 reason: vec![],
    ///                 outcome: vec![],
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
        request: &ResolutionCenterCasesSummaryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SummaryResolutionCenterCasesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "resolution_center_cases/summary",
                None,
                QueryBuilder::new()
                    .serialize_array("groups", request.groups.clone())
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .serialize_array("status", request.status.clone())
                    .serialize_array("reason", request.reason.clone())
                    .serialize_array("outcome", request.outcome.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a single resolution center case with its full event timeline.
    ///
    /// # Arguments
    ///
    /// * `id` - The resolution center case ID (`reso_` tag).
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
    ///         .resolution_center_cases
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ResolutionCenterCase, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("resolution_center_cases/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Accepts the case in the customer's favor, as the merchant: refunds the payment in full and closes the case.
    ///
    /// # Arguments
    ///
    /// * `id` - The resolution center case ID (`reso_` tag).
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
    ///         .resolution_center_cases
    ///         .accept(
    ///             &"id".to_string(),
    ///             &AcceptResolutionCenterCasesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn accept(
        &self,
        id: &str,
        request: &AcceptResolutionCenterCasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResolutionCenterCase, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("resolution_center_cases/{}/accept", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Appeals a decision, as the customer, on a case that closed in the merchant's favor. Escalates the case to Whop for platform review. A case can be appealed once.
    ///
    /// # Arguments
    ///
    /// * `id` - The resolution center case ID (`reso_` tag).
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
    ///         .resolution_center_cases
    ///         .appeal(
    ///             &"id".to_string(),
    ///             &AppealResolutionCenterCasesRequest {
    ///                 message: "The coating is already flaking on the hood two weeks later.".to_string(),
    ///                 attachments: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn appeal(
        &self,
        id: &str,
        request: &AppealResolutionCenterCasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResolutionCenterCase, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("resolution_center_cases/{}/appeal", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Denies the case, as the merchant: rejects the claim and closes the case with no refund.
    ///
    /// # Arguments
    ///
    /// * `id` - The resolution center case ID (`reso_` tag).
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
    ///         .resolution_center_cases
    ///         .deny(
    ///             &"id".to_string(),
    ///             &DenyResolutionCenterCasesRequest {
    ///                 message:
    ///                     "The ceramic coating was applied and the vehicle was collected on 2026-01-05."
    ///                         .to_string(),
    ///                 attachments: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn deny(
        &self,
        id: &str,
        request: &DenyResolutionCenterCasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResolutionCenterCase, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("resolution_center_cases/{}/deny", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the case timeline, newest first. Events the viewer is not allowed to see are omitted — a customer reads the customer-visible timeline, the merchant reads the full one.
    ///
    /// # Arguments
    ///
    /// * `id` - The resolution center case ID (`reso_` tag).
    /// * `first` - The number of events to return (default 20, max 100).
    /// * `after` - A cursor; returns events after this position.
    /// * `last` - The number of events to return from the end of the range.
    /// * `before` - A cursor; returns events before this position.
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
    ///         .resolution_center_cases
    ///         .events(
    ///             &"id".to_string(),
    ///             &EventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn events(
        &self,
        id: &str,
        request: &EventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<EventsResolutionCenterCasesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("resolution_center_cases/{}/events", id),
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Replies to an open request for information on the case. As the merchant this answers Whop's request (valid while the case awaits your information); as the customer it provides the information requested from you. The actor is resolved from the credential.
    ///
    /// # Arguments
    ///
    /// * `id` - The resolution center case ID (`reso_` tag).
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
    ///         .resolution_center_cases
    ///         .reply(
    ///             &"id".to_string(),
    ///             &ReplyResolutionCenterCasesRequest {
    ///                 message: "Here are the before and after photos from the Burnet Rd bay.".to_string(),
    ///                 attachments: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reply(
        &self,
        id: &str,
        request: &ReplyResolutionCenterCasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResolutionCenterCase, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("resolution_center_cases/{}/reply", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Asks the customer for more information, as the merchant. Allowed up to 3 times per case before you must accept or deny it.
    ///
    /// # Arguments
    ///
    /// * `id` - The resolution center case ID (`reso_` tag).
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
    ///         .resolution_center_cases
    ///         .request_info(
    ///             &"id".to_string(),
    ///             &RequestInfoResolutionCenterCasesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn request_info(
        &self,
        id: &str,
        request: &RequestInfoResolutionCenterCasesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ResolutionCenterCase, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("resolution_center_cases/{}/request_info", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Withdraws (cancels) a case you opened, as the customer. Only possible while the case is still open.
    ///
    /// # Arguments
    ///
    /// * `id` - The resolution center case ID (`reso_` tag).
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
    ///         .resolution_center_cases
    ///         .withdraw(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn withdraw(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ResolutionCenterCase, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("resolution_center_cases/{}/withdraw", id),
                None,
                None,
                options,
            )
            .await
    }
}
