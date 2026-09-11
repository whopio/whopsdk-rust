pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateExperimentsRequestControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
}

impl UpdateExperimentsRequestControl {
    pub fn builder() -> UpdateExperimentsRequestControlBuilder {
        <UpdateExperimentsRequestControlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExperimentsRequestControlBuilder {
    related_resource: Option<ExperimentResourceReference>,
}

impl UpdateExperimentsRequestControlBuilder {
    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateExperimentsRequestControl`].
    pub fn build(self) -> Result<UpdateExperimentsRequestControl, BuildError> {
        Ok(UpdateExperimentsRequestControl {
            related_resource: self.related_resource,
        })
    }
}
