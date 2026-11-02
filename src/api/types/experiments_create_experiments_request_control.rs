pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateExperimentsRequestControl {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
}

impl CreateExperimentsRequestControl {
    pub fn builder() -> CreateExperimentsRequestControlBuilder {
        <CreateExperimentsRequestControlBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExperimentsRequestControlBuilder {
    related_resource: Option<ExperimentResourceReference>,
}

impl CreateExperimentsRequestControlBuilder {
    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateExperimentsRequestControl`].
    pub fn build(self) -> Result<CreateExperimentsRequestControl, BuildError> {
        Ok(CreateExperimentsRequestControl {
            related_resource: self.related_resource,
        })
    }
}
