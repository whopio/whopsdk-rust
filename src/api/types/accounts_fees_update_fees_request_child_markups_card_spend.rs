pub use crate::prelude::*;

/// The markup on card purchases settled by the subaccount. `null` clears the custom markup.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequestChildMarkupsCardSpend {
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

impl UpdateFeesRequestChildMarkupsCardSpend {
    pub fn builder() -> UpdateFeesRequestChildMarkupsCardSpendBuilder {
        <UpdateFeesRequestChildMarkupsCardSpendBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestChildMarkupsCardSpendBuilder {
    fixed: Option<f64>,
    percentage: Option<f64>,
}

impl UpdateFeesRequestChildMarkupsCardSpendBuilder {
    pub fn fixed(mut self, value: f64) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn percentage(mut self, value: f64) -> Self {
        self.percentage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequestChildMarkupsCardSpend`].
    pub fn build(self) -> Result<UpdateFeesRequestChildMarkupsCardSpend, BuildError> {
        Ok(UpdateFeesRequestChildMarkupsCardSpend {
            fixed: self.fixed,
            percentage: self.percentage,
        })
    }
}
