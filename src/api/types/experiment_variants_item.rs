pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperimentVariantsItem {
    /// Treatment identifier. `control` is reserved — it is the implicit remainder.
    #[serde(default)]
    pub name: String,
    /// Granted bucket ranges (1% units, end-exclusive) recording this arm's allocation history. Server-managed; ranges are only ever appended, which is what keeps assignments permanent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<Vec<Vec<i64>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resource: Option<ExperimentResourceReference>,
    /// Percentage of all users assigned to this treatment, 1–100. All weights together sum to at most 100; the remainder is control.
    #[serde(default)]
    pub weight: i64,
}

impl ExperimentVariantsItem {
    pub fn builder() -> ExperimentVariantsItemBuilder {
        <ExperimentVariantsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentVariantsItemBuilder {
    name: Option<String>,
    ranges: Option<Vec<Vec<i64>>>,
    related_resource: Option<ExperimentResourceReference>,
    weight: Option<i64>,
}

impl ExperimentVariantsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn ranges(mut self, value: Vec<Vec<i64>>) -> Self {
        self.ranges = Some(value);
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

    /// Consumes the builder and constructs a [`ExperimentVariantsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ExperimentVariantsItemBuilder::name)
    /// - [`weight`](ExperimentVariantsItemBuilder::weight)
    pub fn build(self) -> Result<ExperimentVariantsItem, BuildError> {
        Ok(ExperimentVariantsItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            ranges: self.ranges,
            related_resource: self.related_resource,
            weight: self
                .weight
                .ok_or_else(|| BuildError::missing_field("weight"))?,
        })
    }
}
