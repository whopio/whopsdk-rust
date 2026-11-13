pub use crate::prelude::*;

/// The new markup. Fields left out keep their current value. `null` clears the row so the markup returns to its default.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestMarkupsPaymentMethodsValue {
    /// The amount the platform adds per event, in US dollars.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub fixed: Option<f64>,
    /// The percentage of the transaction the platform adds, where `2` means 2%.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestMarkupsPaymentMethodsValue {
    pub fn builder() -> UpdateFeesRequestMarkupsPaymentMethodsValueBuilder {
        <UpdateFeesRequestMarkupsPaymentMethodsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestMarkupsPaymentMethodsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestMarkupsPaymentMethodsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestMarkupsPaymentMethodsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestMarkupsPaymentMethodsValue, BuildError> {
        Ok(UpdateFeesRequestMarkupsPaymentMethodsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
