pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBusinessesResponseDataItemVolumeUsd {
    /// Credited GMV (awaiting_settlement + settled); excludes canceled and reversed, in USD.
    #[serde(default)]
    pub attributed: String,
    /// GMV awaiting settlement (commission not yet computed), in USD.
    #[serde(default)]
    pub awaiting_settlement: String,
    /// Credited GMV from the trailing 30 days (awaiting_settlement + settled), in USD.
    #[serde(rename = "last_30d")]
    #[serde(default)]
    pub last30d: String,
    /// GMV of pending + completed payments, in USD.
    #[serde(default)]
    pub settled: String,
}

impl ListBusinessesResponseDataItemVolumeUsd {
    pub fn builder() -> ListBusinessesResponseDataItemVolumeUsdBuilder {
        <ListBusinessesResponseDataItemVolumeUsdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBusinessesResponseDataItemVolumeUsdBuilder {
    attributed: Option<String>,
    awaiting_settlement: Option<String>,
    last30d: Option<String>,
    settled: Option<String>,
}

impl ListBusinessesResponseDataItemVolumeUsdBuilder {
    pub fn attributed(mut self, value: impl Into<String>) -> Self {
        self.attributed = Some(value.into());
        self
    }

    pub fn awaiting_settlement(mut self, value: impl Into<String>) -> Self {
        self.awaiting_settlement = Some(value.into());
        self
    }

    pub fn last30d(mut self, value: impl Into<String>) -> Self {
        self.last30d = Some(value.into());
        self
    }

    pub fn settled(mut self, value: impl Into<String>) -> Self {
        self.settled = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBusinessesResponseDataItemVolumeUsd`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributed`](ListBusinessesResponseDataItemVolumeUsdBuilder::attributed)
    /// - [`awaiting_settlement`](ListBusinessesResponseDataItemVolumeUsdBuilder::awaiting_settlement)
    /// - [`last30d`](ListBusinessesResponseDataItemVolumeUsdBuilder::last30d)
    /// - [`settled`](ListBusinessesResponseDataItemVolumeUsdBuilder::settled)
    pub fn build(self) -> Result<ListBusinessesResponseDataItemVolumeUsd, BuildError> {
        Ok(ListBusinessesResponseDataItemVolumeUsd {
            attributed: self
                .attributed
                .ok_or_else(|| BuildError::missing_field("attributed"))?,
            awaiting_settlement: self
                .awaiting_settlement
                .ok_or_else(|| BuildError::missing_field("awaiting_settlement"))?,
            last30d: self
                .last30d
                .ok_or_else(|| BuildError::missing_field("last30d"))?,
            settled: self
                .settled
                .ok_or_else(|| BuildError::missing_field("settled"))?,
        })
    }
}
