pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExperimentResourceReference {
    /// Referenced resource tag, belonging to the experiment owner.
    #[serde(default)]
    pub id: String,
    pub object: ExperimentResourceReferenceObject,
}

impl ExperimentResourceReference {
    pub fn builder() -> ExperimentResourceReferenceBuilder {
        <ExperimentResourceReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentResourceReferenceBuilder {
    id: Option<String>,
    object: Option<ExperimentResourceReferenceObject>,
}

impl ExperimentResourceReferenceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: ExperimentResourceReferenceObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExperimentResourceReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExperimentResourceReferenceBuilder::id)
    /// - [`object`](ExperimentResourceReferenceBuilder::object)
    pub fn build(self) -> Result<ExperimentResourceReference, BuildError> {
        Ok(ExperimentResourceReference {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
