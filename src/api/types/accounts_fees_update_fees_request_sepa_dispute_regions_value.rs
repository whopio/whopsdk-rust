pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestSepaDisputeRegionsValue {
    /// The new amount per event, in euros (EUR). `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestSepaDisputeRegionsValue {
    pub fn builder() -> UpdateFeesRequestSepaDisputeRegionsValueBuilder {
        <UpdateFeesRequestSepaDisputeRegionsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestSepaDisputeRegionsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestSepaDisputeRegionsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestSepaDisputeRegionsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestSepaDisputeRegionsValue, BuildError> {
        Ok(UpdateFeesRequestSepaDisputeRegionsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
