pub use crate::prelude::*;

/// Available deposit methods for destination.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDepositsResponseMethods {
    /// Bank deposit details. Only present when bank deposits are active for the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<CreateDepositsResponseMethodsBank>,
    /// Crypto networks available for this deposit, each with its on-chain deposit address and the tokens accepted on that network.
    #[serde(default)]
    pub crypto: Vec<CreateDepositsResponseMethodsCryptoItem>,
}

impl CreateDepositsResponseMethods {
    pub fn builder() -> CreateDepositsResponseMethodsBuilder {
        <CreateDepositsResponseMethodsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsResponseMethodsBuilder {
    bank: Option<CreateDepositsResponseMethodsBank>,
    crypto: Option<Vec<CreateDepositsResponseMethodsCryptoItem>>,
}

impl CreateDepositsResponseMethodsBuilder {
    pub fn bank(mut self, value: CreateDepositsResponseMethodsBank) -> Self {
        self.bank = Some(value);
        self
    }

    pub fn crypto(mut self, value: Vec<CreateDepositsResponseMethodsCryptoItem>) -> Self {
        self.crypto = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsResponseMethods`].
    /// This method will fail if any of the following fields are not set:
    /// - [`crypto`](CreateDepositsResponseMethodsBuilder::crypto)
    pub fn build(self) -> Result<CreateDepositsResponseMethods, BuildError> {
        Ok(CreateDepositsResponseMethods {
            bank: self.bank,
            crypto: self
                .crypto
                .ok_or_else(|| BuildError::missing_field("crypto"))?,
        })
    }
}
