pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ExposuresExperimentsResponse {
    ExperimentExposure(ExperimentExposure),

    ExperimentExposureBatch(ExperimentExposureBatch),
}

impl ExposuresExperimentsResponse {
    pub fn is_experiment_exposure(&self) -> bool {
        matches!(self, Self::ExperimentExposure(_))
    }

    pub fn is_experiment_exposure_batch(&self) -> bool {
        matches!(self, Self::ExperimentExposureBatch(_))
    }

    pub fn as_experiment_exposure(&self) -> Option<&ExperimentExposure> {
        match self {
            Self::ExperimentExposure(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_experiment_exposure(self) -> Option<ExperimentExposure> {
        match self {
            Self::ExperimentExposure(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_experiment_exposure_batch(&self) -> Option<&ExperimentExposureBatch> {
        match self {
            Self::ExperimentExposureBatch(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_experiment_exposure_batch(self) -> Option<ExperimentExposureBatch> {
        match self {
            Self::ExperimentExposureBatch(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ExposuresExperimentsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExperimentExposure(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ExperimentExposureBatch(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
