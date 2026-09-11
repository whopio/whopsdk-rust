pub use crate::prelude::*;

/// Query parameters for exposures
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExposuresQueryRequest {
    /// Bucketing subject. Ownership is the top-level account_id. Account experiments accept caller-supplied subject IDs; internal experiments derive the user from the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<ExposuresExperimentsRequestSubject>,
    /// Restricts batch evaluation to this related resource; omitted batches contain only unbound experiments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Flag or experiment to evaluate — the flag_key handle or the `expt_` id. Omit to return all flags the caller qualifies for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_key: Option<String>,
    /// Owning account ID or internal. Required when evaluating by flag_key or in a batch; optional for an expt_ ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// JSON-encoded scalar values that property targeting conditions match against. Numeric and boolean strings are coerced. Nested query keys such as properties[plan]=pro remain accepted for existing callers. For internal experiments, is_internal_user is derived from the session and cannot be overridden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
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

    /// Consumes the builder and constructs a [`ExposuresQueryRequest`].
    pub fn build(self) -> Result<ExposuresQueryRequest, BuildError> {
        Ok(ExposuresQueryRequest {
            subject: self.subject,
            related_resource: self.related_resource,
            flag_key: self.flag_key,
            account_id: self.account_id,
            properties: self.properties,
        })
    }
}
