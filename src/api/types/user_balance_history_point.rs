pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserBalanceHistoryPoint {
    /// Point timestamp, in Unix seconds.
    #[serde(default)]
    pub t: i64,
    /// Cumulative wallet balance at this point, in USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub v: f64,
}

impl UserBalanceHistoryPoint {
    pub fn builder() -> UserBalanceHistoryPointBuilder {
        <UserBalanceHistoryPointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBalanceHistoryPointBuilder {
    t: Option<i64>,
    v: Option<f64>,
}

impl UserBalanceHistoryPointBuilder {
    pub fn t(mut self, value: i64) -> Self {
        self.t = Some(value);
        self
    }

    pub fn v(mut self, value: f64) -> Self {
        self.v = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserBalanceHistoryPoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`t`](UserBalanceHistoryPointBuilder::t)
    /// - [`v`](UserBalanceHistoryPointBuilder::v)
    pub fn build(self) -> Result<UserBalanceHistoryPoint, BuildError> {
        Ok(UserBalanceHistoryPoint {
            t: self.t.ok_or_else(|| BuildError::missing_field("t"))?,
            v: self.v.ok_or_else(|| BuildError::missing_field("v"))?,
        })
    }
}
