pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestAchReturnRegionsValue {
    /// The new amount per event, in US dollars (USD). `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestAchReturnRegionsValue {
    pub fn builder() -> UpdateFeesRequestAchReturnRegionsValueBuilder {
        <UpdateFeesRequestAchReturnRegionsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestAchReturnRegionsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestAchReturnRegionsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestAchReturnRegionsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestAchReturnRegionsValue, BuildError> {
        Ok(UpdateFeesRequestAchReturnRegionsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
