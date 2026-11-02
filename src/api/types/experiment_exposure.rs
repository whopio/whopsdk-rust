pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperimentExposure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_revision: Option<i64>,
    /// Whether the subject receives a treatment or enabled feature flag.
    #[serde(default)]
    pub enabled: bool,
    /// Unique experiment ID, included for account experiments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    /// Present when evaluating a single experiment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Treatment name, control, or null for an inactive flag or excluded subject.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

impl ExperimentExposure {
    pub fn builder() -> ExperimentExposureBuilder {
        <ExperimentExposureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentExposureBuilder {
    configuration_revision: Option<i64>,
    enabled: Option<bool>,
    experiment_id: Option<String>,
    flag_key: Option<String>,
    related_resource: Option<ExperimentResourceReference>,
    variant: Option<String>,
}

impl ExperimentExposureBuilder {
    pub fn configuration_revision(mut self, value: i64) -> Self {
        self.configuration_revision = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn experiment_id(mut self, value: impl Into<String>) -> Self {
        self.experiment_id = Some(value.into());
        self
    }

    pub fn flag_key(mut self, value: impl Into<String>) -> Self {
        self.flag_key = Some(value.into());
        self
    }

    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    pub fn variant(mut self, value: impl Into<String>) -> Self {
        self.variant = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExperimentExposure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](ExperimentExposureBuilder::enabled)
    pub fn build(self) -> Result<ExperimentExposure, BuildError> {
        Ok(ExperimentExposure {
            configuration_revision: self.configuration_revision,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            experiment_id: self.experiment_id,
            flag_key: self.flag_key,
            related_resource: self.related_resource,
            variant: self.variant,
        })
    }
}
