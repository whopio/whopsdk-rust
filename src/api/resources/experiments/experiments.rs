use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct ExperimentsClient {
    pub http_client: HttpClient,
}

impl ExperimentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the experiments and feature flags owned by an account. Requires `experiment:read` on the account.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Owning account ID.
    /// * `related_resource` - Filter by related resource; requires account_id.
    /// * `status` - Only experiments with this status.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `order` - The field to sort experiments by.
    /// * `direction` - Sort direction.
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
    ///         .experiments
    ///         .list(
    ///             &ExperimentsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ExperimentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListExperimentsResponse, ApiError> {
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
                "experiments",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("related_resource", request.related_resource.clone())
                    .serialize("status", request.status.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &ExperimentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .serialize("related_resource", request.related_resource.clone())
            .serialize("status", request.status.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
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
                            "experiments",
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

    /// Creates an experiment or feature flag in `draft` status for an account. Nothing is served until you activate it with `POST /experiments/:id/activate`, which needs Economic Intelligence on for the account. Requires `experiment:manage` on the account.
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
    ///         .experiments
    ///         .create(
    ///             &CreateExperimentsRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 flag_key: "checkout_redesign_v2".to_string(),
    ///                 bucket_by: None,
    ///                 control: None,
    ///                 feature_flag_only: None,
    ///                 hypothesis: None,
    ///                 metadata: None,
    ///                 name: None,
    ///                 related_resource: None,
    ///                 targeting_rules: None,
    ///                 variants: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateExperimentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experiment, ApiError> {
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
                "experiments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Evaluates experiments and feature flags for a subject and records an exposure. Pass `flag_key` to evaluate one flag, or omit it to evaluate every active flag in the account and related resource scope. Requires no authentication; when a credential resolves, its authentication method, API key ID, and signed-in user ID are recorded on the exposure event.
    ///
    /// # Arguments
    ///
    /// * `subject` - Bucketing subject: the user, account, or anonymous ID the experiment's `bucket_by` assigns on. Only the identity matching `bucket_by` is used; when it is missing, evaluating a single flag fails and a batch omits that experiment. Subjects outside every treatment range receive `control`. `subject[account_id]` also enables account-level targeting rules. Experiment ownership comes from the top-level `account_id`.
    /// * `related_resource` - Restricts batch evaluation to this related resource; omitted batches contain only unbound experiments.
    /// * `flag_key` - Flag or experiment to evaluate — the flag_key handle or the `expt_` id. Omit to return all flags the caller qualifies for.
    /// * `account_id` - Owning account ID. Required when evaluating by flag_key or in a batch; optional for an expt_ ID.
    /// * `properties` - JSON-encoded scalar values that property targeting conditions match against. Numeric and boolean strings are coerced. Nested query keys such as properties[variant]=pro remain accepted for existing callers.
    /// * `log_exposure` - Set `false` to read an assignment without recording an exposure, such as when a client caches assignments up front and records the exposure when the arm is rendered. Omitted records the exposure.
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
    ///         .experiments
    ///         .exposures(
    ///             &ExposuresQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn exposures(
        &self,
        request: &ExposuresQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExposuresExperimentsResponse, ApiError> {
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
                "experiments/exposures",
                None,
                QueryBuilder::new()
                    .serialize("subject", request.subject.clone())
                    .serialize("related_resource", request.related_resource.clone())
                    .string("flag_key", request.flag_key.clone())
                    .string("account_id", request.account_id.clone())
                    .string("properties", request.properties.clone())
                    .bool("log_exposure", request.log_exposure.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves an experiment or feature flag by its `expt_` ID or `flag_key` handle. Requires `experiment:read` on the owning account.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account ID. Required when id is a flag key; optional for an expt_ ID.
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
    ///         .experiments
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &ExperimentsRetrieveQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        request: &ExperimentsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experiment, ApiError> {
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
                &format!("experiments/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates the targeting rules, treatment allocation, or hypothesis of an existing experiment or feature flag. To change its lifecycle, use the `activate`, `pause`, and `end` endpoints instead. Requires `experiment:manage` on the owning account.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account ID. Required when id is a flag key; optional for an expt_ ID.
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
    ///         .experiments
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateExperimentsRequest {
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
        request: &UpdateExperimentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experiment, ApiError> {
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
                &format!("experiments/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Starts (or resumes) an experiment or feature flag so evaluation begins serving it. Activating a draft stamps `started_at`; resuming a paused experiment keeps the original start. Only drafts and paused experiments can be activated. Requires `experiment:manage` on the owning account. Activating needs Economic Intelligence on for the owning account: while the account's `economic_intelligence` is `false`, this returns a `403` and the experiment keeps its status. Turn it on with `PATCH /accounts/:account_id/preferences`. Creating, editing, pausing and ending an experiment don't need it.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account ID. Required when id is a flag key; optional for an expt_ ID.
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
    ///         .experiments
    ///         .activate(
    ///             &"id".to_string(),
    ///             &ActivateExperimentsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn activate(
        &self,
        id: &str,
        request: &ActivateExperimentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experiment, ApiError> {
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
                &format!("experiments/{}/activate", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Concludes the experiment and records required `findings`. Pass `winning_arm` to serve the winning treatment to everyone; omit it when control won. Ended experiments cannot restart, but may be ended again to correct the winner. Requires `experiment:manage` on the owning account.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account ID. Required when id is a flag key; optional for an expt_ ID.
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
    ///         .experiments
    ///         .end(
    ///             &"id".to_string(),
    ///             &EndExperimentsRequest {
    ///                 findings: "Treatment lifted signups 12%, shipping it to everyone.".to_string(),
    ///                 account_id: None,
    ///                 winning_arm: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn end(
        &self,
        id: &str,
        request: &EndExperimentsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experiment, ApiError> {
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
                &format!("experiments/{}/end", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Pauses an active experiment or feature flag: evaluation stops serving it and exposures stop flowing. Assignments are keyed on stable identity, so users return to their original arm when the experiment resumes. Requires `experiment:manage` on the owning account.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account ID. Required when id is a flag key; optional for an expt_ ID.
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
    ///         .experiments
    ///         .pause(
    ///             &"id".to_string(),
    ///             &PauseQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn pause(
        &self,
        id: &str,
        request: &PauseQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Experiment, ApiError> {
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
                &format!("experiments/{}/pause", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
