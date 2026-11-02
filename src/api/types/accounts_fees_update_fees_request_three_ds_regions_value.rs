pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestThreeDsRegionsValue {
    /// The new amount per event in US dollars. `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestThreeDsRegionsValue {
    pub fn builder() -> UpdateFeesRequestThreeDsRegionsValueBuilder {
        <UpdateFeesRequestThreeDsRegionsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestThreeDsRegionsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestThreeDsRegionsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestThreeDsRegionsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestThreeDsRegionsValue, BuildError> {
        Ok(UpdateFeesRequestThreeDsRegionsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
