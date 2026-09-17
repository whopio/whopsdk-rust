pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestPaymentMethodsValueRegionsValue {
    /// The new amount per event in US dollars. `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestPaymentMethodsValueRegionsValue {
    pub fn builder() -> UpdateFeesRequestPaymentMethodsValueRegionsValueBuilder {
        <UpdateFeesRequestPaymentMethodsValueRegionsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestPaymentMethodsValueRegionsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestPaymentMethodsValueRegionsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestPaymentMethodsValueRegionsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestPaymentMethodsValueRegionsValue, BuildError> {
        Ok(UpdateFeesRequestPaymentMethodsValueRegionsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
