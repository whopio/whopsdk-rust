pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestSepaReturnRegionsValue {
    /// The new amount per event, in euros (EUR). `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestSepaReturnRegionsValue {
    pub fn builder() -> UpdateFeesRequestSepaReturnRegionsValueBuilder {
        <UpdateFeesRequestSepaReturnRegionsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestSepaReturnRegionsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestSepaReturnRegionsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestSepaReturnRegionsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestSepaReturnRegionsValue, BuildError> {
        Ok(UpdateFeesRequestSepaReturnRegionsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
