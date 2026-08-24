pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Transfer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TransferOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<TransferDestination>,
}

impl Transfer {
    pub fn builder() -> TransferBuilder {
        <TransferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferBuilder {
    origin: Option<TransferOrigin>,
    destination: Option<TransferDestination>,
}

impl TransferBuilder {
    pub fn origin(mut self, value: TransferOrigin) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn destination(mut self, value: TransferDestination) -> Self {
        self.destination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Transfer`].
    pub fn build(self) -> Result<Transfer, BuildError> {
        Ok(Transfer {
            origin: self.origin,
            destination: self.destination,
        })
    }
}
