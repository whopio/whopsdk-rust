pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperimentControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
}

impl ExperimentControl {
    pub fn builder() -> ExperimentControlBuilder {
        <ExperimentControlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentControlBuilder {
    related_resource: Option<ExperimentResourceReference>,
}

impl ExperimentControlBuilder {
    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExperimentControl`].
    pub fn build(self) -> Result<ExperimentControl, BuildError> {
        Ok(ExperimentControl {
            related_resource: self.related_resource,
        })
    }
}
