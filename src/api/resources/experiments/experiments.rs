use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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

    /// Lists experiments for one account with experiment:read permission. Omit account_id or pass internal to list internal experiments, which requires Whop internal access.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Owning account ID. Omit or pass internal for Whop internal experiments; internal access is required.
    /// * `related_resource` - Filter by related resource; requires account_id.
    /// * `status` - Only experiments with this status.
    /// * `first` - The number of experiments to return (default 20, max 100).
    /// * `after` - A cursor; returns experiments after this position.
    /// * `last` - The number of experiments to return from the end of the range.
    /// * `before` - A cursor; returns experiments before this position.
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
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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

    /// Creates a draft experiment for the specified account. Use internal for a Whop platform experiment.
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
    ///                 account_id: "internal".to_string(),
    ///                 flag_key: "checkout_redesign_v2".to_string(),
    ///                 bucket_by: None,
    ///                 control: None,
    ///                 feature_flag_only: None,
    ///                 hypothesis: None,
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
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "experiments",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Evaluates and records an exposure without requiring authentication. When credentials resolve, their authentication method, API key ID, and signed-in user ID are recorded on the exposure event. Pass subject for bucketing identity and account_id for experiment ownership.
    ///
    /// Pass `flag_key` to check a single flag, or omit it to fetch active flags in the account and related resource scope. Internal anonymous callers may use the `x-whop-anonymous-id` header or `ajs_anonymous_id` cookie; explicit `subject[anonymous_id]` takes precedence.
    ///
    /// Assignments use exactly the configured `bucket_by`: `subject[user_id]`, `subject[account_id]`, or `subject[anonymous_id]`. Internal user experiments derive identity from the signed-in session. Missing the required identity fails single evaluation and omits the experiment from batch evaluation. Subjects outside all treatment ranges receive control.
    ///
    /// Pass `subject[account_id]` to enable account-level targeting rules. Pass `properties` as a JSON object to supply the values that `property` targeting conditions match against.
    ///
    /// # Arguments
    ///
    /// * `subject` - Bucketing subject. Ownership is the top-level account_id. Account experiments accept caller-supplied subject IDs; internal experiments derive the user from the session.
    /// * `related_resource` - Restricts batch evaluation to this related resource; omitted batches contain only unbound experiments.
    /// * `flag_key` - Flag or experiment to evaluate — the flag_key handle or the `expt_` id. Omit to return all flags the caller qualifies for.
    /// * `account_id` - Owning account ID or internal. Required when evaluating by flag_key or in a batch; optional for an expt_ ID.
    /// * `properties` - JSON-encoded scalar values that property targeting conditions match against. Numeric and boolean strings are coerced. Nested query keys such as properties[plan]=pro remain accepted for existing callers. For internal experiments, is_internal_user is derived from the session and cannot be overridden.
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
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "experiments/exposures",
                None,
                QueryBuilder::new()
                    .serialize("subject", request.subject.clone())
                    .serialize("related_resource", request.related_resource.clone())
                    .string("flag_key", request.flag_key.clone())
                    .string("account_id", request.account_id.clone())
                    .string("properties", request.properties.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a single experiment or feature flag by its `expt_` id or flag_key handle. Requires the corresponding experiment permission on the owning account, or Whop internal access for internal experiments.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
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
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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

    /// Updates the targeting rules, treatment allocation, metrics, or hypothesis of an existing experiment or feature flag. Weights and metrics can only grow, so enrolled users never change arms and an existing metric is never dropped. Lifecycle moves through the transition endpoints (`activate`, `pause`, `end`), never through this update. Requires the corresponding experiment permission on the owning account, or Whop internal access for internal experiments.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
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
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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

    /// Starts (or resumes) an experiment or feature flag so evaluation begins serving it. Activating a draft stamps `started_at`; resuming a paused experiment keeps the original start. Only drafts and paused experiments can be activated. Requires the corresponding experiment permission on the owning account, or Whop internal access for internal experiments.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
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
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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

    /// Concludes the experiment and records required `findings`. Pass `winning_arm` to serve the winning treatment to everyone; omit it when control won. Ended experiments cannot restart, but may be ended again to correct the winner. Requires experiment:manage on the account, or internal access for platform experiments.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
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
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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

    /// Pauses an active experiment or feature flag: evaluation stops serving it and exposures stop flowing. Assignments are keyed on stable identity, so users return to their original arm when the experiment resumes. Requires the corresponding experiment permission on the owning account, or Whop internal access for internal experiments.
    ///
    /// # Arguments
    ///
    /// * `id` - The experiment identifier — the `expt_` id or the flag_key handle.
    /// * `account_id` - Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
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
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
