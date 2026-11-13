pub use crate::prelude::*;

/// Query parameters for exposures
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExposuresQueryRequest {
    /// Bucketing subject: the user, account, or anonymous ID the experiment's `bucket_by` assigns on. Only the identity matching `bucket_by` is used; when it is missing, evaluating a single flag fails and a batch omits that experiment. Subjects outside every treatment range receive `control`. `subject[account_id]` also enables account-level targeting rules. Experiment ownership comes from the top-level `account_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<ExposuresExperimentsRequestSubject>,
    /// Restricts batch evaluation to this related resource; omitted batches contain only unbound experiments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Flag or experiment to evaluate — the flag_key handle or the `expt_` id. Omit to return all flags the caller qualifies for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_key: Option<String>,
    /// Owning account ID. Required when evaluating by flag_key or in a batch; optional for an expt_ ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// JSON-encoded scalar values that property targeting conditions match against. Numeric and boolean strings are coerced. Nested query keys such as properties[variant]=pro remain accepted for existing callers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
    /// Set `false` to read an assignment without recording an exposure, such as when a client caches assignments up front and records the exposure when the arm is rendered. Omitted records the exposure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_exposure: Option<bool>,
}

impl ExposuresQueryRequest {
    pub fn builder() -> ExposuresQueryRequestBuilder {
        <ExposuresQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExposuresQueryRequestBuilder {
    subject: Option<ExposuresExperimentsRequestSubject>,
    related_resource: Option<ExperimentResourceReference>,
    flag_key: Option<String>,
    account_id: Option<String>,
    properties: Option<String>,
    log_exposure: Option<bool>,
}

impl ExposuresQueryRequestBuilder {
    pub fn subject(mut self, value: ExposuresExperimentsRequestSubject) -> Self {
        self.subject = Some(value);
        self
    }

    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    pub fn flag_key(mut self, value: impl Into<String>) -> Self {
        self.flag_key = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn properties(mut self, value: impl Into<String>) -> Self {
        self.properties = Some(value.into());
        self
    }

    pub fn log_exposure(mut self, value: bool) -> Self {
        self.log_exposure = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExposuresQueryRequest`].
    pub fn build(self) -> Result<ExposuresQueryRequest, BuildError> {
        Ok(ExposuresQueryRequest {
            subject: self.subject,
            related_resource: self.related_resource,
            flag_key: self.flag_key,
            account_id: self.account_id,
            properties: self.properties,
            log_exposure: self.log_exposure,
        })
    }
}
