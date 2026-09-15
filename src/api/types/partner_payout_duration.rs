pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartnerPayoutDuration {
    /// Unit of the earning window. Month means a calendar month; day means a day.
    pub unit: PartnerPayoutDurationUnit,
    /// Number of units in the earning window.
    #[serde(default)]
    pub value: i64,
}

impl PartnerPayoutDuration {
    pub fn builder() -> PartnerPayoutDurationBuilder {
        <PartnerPayoutDurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerPayoutDurationBuilder {
    unit: Option<PartnerPayoutDurationUnit>,
    value: Option<i64>,
}

impl PartnerPayoutDurationBuilder {
    pub fn unit(mut self, value: PartnerPayoutDurationUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn value(mut self, value: i64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerPayoutDuration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](PartnerPayoutDurationBuilder::unit)
    /// - [`value`](PartnerPayoutDurationBuilder::value)
    pub fn build(self) -> Result<PartnerPayoutDuration, BuildError> {
        Ok(PartnerPayoutDuration {
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
