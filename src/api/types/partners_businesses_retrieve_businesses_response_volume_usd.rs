pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponseVolumeUsd {
    /// Credited GMV (awaiting_settlement + settled); excludes canceled and reversed, in USD.
    #[serde(default)]
    pub attributed: String,
    /// GMV awaiting settlement (commission not yet computed), in USD.
    #[serde(default)]
    pub awaiting_settlement: String,
    /// GMV of pending + completed payments, in USD.
    #[serde(default)]
    pub settled: String,
}

impl RetrieveBusinessesResponseVolumeUsd {
    pub fn builder() -> RetrieveBusinessesResponseVolumeUsdBuilder {
        <RetrieveBusinessesResponseVolumeUsdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseVolumeUsdBuilder {
    attributed: Option<String>,
    awaiting_settlement: Option<String>,
    settled: Option<String>,
}

impl RetrieveBusinessesResponseVolumeUsdBuilder {
    pub fn attributed(mut self, value: impl Into<String>) -> Self {
        self.attributed = Some(value.into());
        self
    }

    pub fn awaiting_settlement(mut self, value: impl Into<String>) -> Self {
        self.awaiting_settlement = Some(value.into());
        self
    }

    pub fn settled(mut self, value: impl Into<String>) -> Self {
        self.settled = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponseVolumeUsd`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributed`](RetrieveBusinessesResponseVolumeUsdBuilder::attributed)
    /// - [`awaiting_settlement`](RetrieveBusinessesResponseVolumeUsdBuilder::awaiting_settlement)
    /// - [`settled`](RetrieveBusinessesResponseVolumeUsdBuilder::settled)
    pub fn build(self) -> Result<RetrieveBusinessesResponseVolumeUsd, BuildError> {
        Ok(RetrieveBusinessesResponseVolumeUsd {
            attributed: self
                .attributed
                .ok_or_else(|| BuildError::missing_field("attributed"))?,
            awaiting_settlement: self
                .awaiting_settlement
                .ok_or_else(|| BuildError::missing_field("awaiting_settlement"))?,
            settled: self
                .settled
                .ok_or_else(|| BuildError::missing_field("settled"))?,
        })
    }
}
