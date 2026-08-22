pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserBalanceHistory {
    #[serde(default)]
    pub data: Vec<UserBalanceHistoryPoint>,
    /// Value of the most recent point, in USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub last: f64,
    /// Maximum value across the window, in USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub max: f64,
    /// Minimum value across the window, in USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub min: f64,
}

impl UserBalanceHistory {
    pub fn builder() -> UserBalanceHistoryBuilder {
        <UserBalanceHistoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBalanceHistoryBuilder {
    data: Option<Vec<UserBalanceHistoryPoint>>,
    last: Option<f64>,
    max: Option<f64>,
    min: Option<f64>,
}

impl UserBalanceHistoryBuilder {
    pub fn data(mut self, value: Vec<UserBalanceHistoryPoint>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn last(mut self, value: f64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn max(mut self, value: f64) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(mut self, value: f64) -> Self {
        self.min = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserBalanceHistory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](UserBalanceHistoryBuilder::data)
    /// - [`last`](UserBalanceHistoryBuilder::last)
    /// - [`max`](UserBalanceHistoryBuilder::max)
    /// - [`min`](UserBalanceHistoryBuilder::min)
    pub fn build(self) -> Result<UserBalanceHistory, BuildError> {
        Ok(UserBalanceHistory {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            last: self.last.ok_or_else(|| BuildError::missing_field("last"))?,
            max: self.max.ok_or_else(|| BuildError::missing_field("max"))?,
            min: self.min.ok_or_else(|| BuildError::missing_field("min"))?,
        })
    }
}
