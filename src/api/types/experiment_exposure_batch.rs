pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct ExperimentExposureBatch {
    pub exposures: HashMap<String, ExperimentExposure>,
}

impl ExperimentExposureBatch {
    pub fn builder() -> ExperimentExposureBatchBuilder {
        <ExperimentExposureBatchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentExposureBatchBuilder {
    exposures: Option<HashMap<String, ExperimentExposure>>,
}

impl ExperimentExposureBatchBuilder {
    pub fn exposures(mut self, value: HashMap<String, ExperimentExposure>) -> Self {
        self.exposures = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExperimentExposureBatch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`exposures`](ExperimentExposureBatchBuilder::exposures)
    pub fn build(self) -> Result<ExperimentExposureBatch, BuildError> {
        Ok(ExperimentExposureBatch {
            exposures: self
                .exposures
                .ok_or_else(|| BuildError::missing_field("exposures"))?,
        })
    }
}
