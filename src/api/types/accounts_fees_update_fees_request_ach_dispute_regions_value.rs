pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestAchDisputeRegionsValue {
    /// The new amount per event, in US dollars (USD). `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestAchDisputeRegionsValue {
    pub fn builder() -> UpdateFeesRequestAchDisputeRegionsValueBuilder {
        <UpdateFeesRequestAchDisputeRegionsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestAchDisputeRegionsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestAchDisputeRegionsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestAchDisputeRegionsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestAchDisputeRegionsValue, BuildError> {
        Ok(UpdateFeesRequestAchDisputeRegionsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
