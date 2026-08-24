pub use crate::prelude::*;

/// The kind of money movement, which decides what comes back. Defaults to ledger. `ledger` moves credit between two Whop balances and returns a `transfer`; `wallet_send` sends USDT from the origin account's Ethereum wallet and returns a `send`; `claim_link` funds a shareable link anyone with the URL can redeem and returns a `claim_link`. A `ledger` transfer from a stablecoin-rails account settles on-chain when covered, and still returns a `transfer`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateTransfersRequestType {
    Ledger,
    WalletSend,
    ClaimLink,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateTransfersRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ledger => serializer.serialize_str("ledger"),
            Self::WalletSend => serializer.serialize_str("wallet_send"),
            Self::ClaimLink => serializer.serialize_str("claim_link"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateTransfersRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ledger" => Ok(Self::Ledger),
            "wallet_send" => Ok(Self::WalletSend),
            "claim_link" => Ok(Self::ClaimLink),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateTransfersRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ledger => write!(f, "ledger"),
            Self::WalletSend => write!(f, "wallet_send"),
            Self::ClaimLink => write!(f, "claim_link"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
