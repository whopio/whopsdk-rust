pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateExperimentsRequestVariantsItem {
    /// Treatment identifier. `control` is reserved for the implicit remainder.
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Percentage of all subjects assigned to this treatment. The remainder is control.
    #[serde(default)]
    pub weight: i64,
}

impl CreateExperimentsRequestVariantsItem {
    pub fn builder() -> CreateExperimentsRequestVariantsItemBuilder {
        <CreateExperimentsRequestVariantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExperimentsRequestVariantsItemBuilder {
    name: Option<String>,
    related_resource: Option<ExperimentResourceReference>,
    weight: Option<i64>,
}

impl CreateExperimentsRequestVariantsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn related_resource(mut self, value: ExperimentResourceReference) -> Self {
        self.related_resource = Some(value);
        self
    }

    pub fn weight(mut self, value: i64) -> Self {
        self.weight = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateExperimentsRequestVariantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateExperimentsRequestVariantsItemBuilder::name)
    /// - [`weight`](CreateExperimentsRequestVariantsItemBuilder::weight)
    pub fn build(self) -> Result<CreateExperimentsRequestVariantsItem, BuildError> {
        Ok(CreateExperimentsRequestVariantsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            related_resource: self.related_resource,
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
        })
    }
}
