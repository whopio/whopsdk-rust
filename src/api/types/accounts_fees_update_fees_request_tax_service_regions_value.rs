pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestTaxServiceRegionsValue {
    /// The new amount per event in US dollars. `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestTaxServiceRegionsValue {
    pub fn builder() -> UpdateFeesRequestTaxServiceRegionsValueBuilder {
        <UpdateFeesRequestTaxServiceRegionsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestTaxServiceRegionsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestTaxServiceRegionsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestTaxServiceRegionsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestTaxServiceRegionsValue, BuildError> {
        Ok(UpdateFeesRequestTaxServiceRegionsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
