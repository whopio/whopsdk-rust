pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateExperimentsRequestVariantsItem {
    /// Treatment name. Pass every existing treatment (weights equal or higher); append new names to add arms. `control` is reserved.
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Percentage assigned to treatment; allocation may only grow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

impl UpdateExperimentsRequestVariantsItem {
    pub fn builder() -> UpdateExperimentsRequestVariantsItemBuilder {
        <UpdateExperimentsRequestVariantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExperimentsRequestVariantsItemBuilder {
    name: Option<String>,
    related_resource: Option<ExperimentResourceReference>,
    weight: Option<i64>,
}

impl UpdateExperimentsRequestVariantsItemBuilder {
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

    /// Consumes the builder and constructs a [`UpdateExperimentsRequestVariantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateExperimentsRequestVariantsItemBuilder::name)
    pub fn build(self) -> Result<UpdateExperimentsRequestVariantsItem, BuildError> {
        Ok(UpdateExperimentsRequestVariantsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            related_resource: self.related_resource,
            weight: self.weight,
        })
    }
}
