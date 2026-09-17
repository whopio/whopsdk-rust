pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestPendingAutoTopupRegionsValue {
    /// The new amount per event in US dollars. `null` clears the custom amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<f64>,
    /// The new percentage, where `2` means 2%. `null` clears the custom rate so the fee returns to its default or inherited rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f64>,
}

impl UpdateFeesRequestPendingAutoTopupRegionsValue {
    pub fn builder() -> UpdateFeesRequestPendingAutoTopupRegionsValueBuilder {
        <UpdateFeesRequestPendingAutoTopupRegionsValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestPendingAutoTopupRegionsValueBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestPendingAutoTopupRegionsValueBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestPendingAutoTopupRegionsValue`].
    pub fn build(self) -> Result<UpdateFeesRequestPendingAutoTopupRegionsValue, BuildError> {
        Ok(UpdateFeesRequestPendingAutoTopupRegionsValue {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
