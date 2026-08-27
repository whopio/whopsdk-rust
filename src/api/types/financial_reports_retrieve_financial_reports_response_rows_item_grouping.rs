pub use crate::prelude::*;

/// The family the row's `line_category` rolls up into. Balance summary rows are always `balance`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveFinancialReportsResponseRowsItemGrouping {
    Advertising,
    AffiliatesAndRevshare,
    Airdrops,
    BadDebt,
    Balance,
    Clawbacks,
    ConnectedAccounts,
    Crypto,
    Disputes,
    Fees,
    Fx,
    Legacy,
    Misc,
    Other,
    Payments,
    Refunds,
    Reserves,
    Transfers,
    WalletsAndBalance,
    Withdrawals,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveFinancialReportsResponseRowsItemGrouping {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Advertising => serializer.serialize_str("advertising"),
            Self::AffiliatesAndRevshare => serializer.serialize_str("affiliates_and_revshare"),
            Self::Airdrops => serializer.serialize_str("airdrops"),
            Self::BadDebt => serializer.serialize_str("bad_debt"),
            Self::Balance => serializer.serialize_str("balance"),
            Self::Clawbacks => serializer.serialize_str("clawbacks"),
            Self::ConnectedAccounts => serializer.serialize_str("connected_accounts"),
            Self::Crypto => serializer.serialize_str("crypto"),
            Self::Disputes => serializer.serialize_str("disputes"),
            Self::Fees => serializer.serialize_str("fees"),
            Self::Fx => serializer.serialize_str("fx"),
            Self::Legacy => serializer.serialize_str("legacy"),
            Self::Misc => serializer.serialize_str("misc"),
            Self::Other => serializer.serialize_str("other"),
            Self::Payments => serializer.serialize_str("payments"),
            Self::Refunds => serializer.serialize_str("refunds"),
            Self::Reserves => serializer.serialize_str("reserves"),
            Self::Transfers => serializer.serialize_str("transfers"),
            Self::WalletsAndBalance => serializer.serialize_str("wallets_and_balance"),
            Self::Withdrawals => serializer.serialize_str("withdrawals"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveFinancialReportsResponseRowsItemGrouping {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "advertising" => Ok(Self::Advertising),
            "affiliates_and_revshare" => Ok(Self::AffiliatesAndRevshare),
            "airdrops" => Ok(Self::Airdrops),
            "bad_debt" => Ok(Self::BadDebt),
            "balance" => Ok(Self::Balance),
            "clawbacks" => Ok(Self::Clawbacks),
            "connected_accounts" => Ok(Self::ConnectedAccounts),
            "crypto" => Ok(Self::Crypto),
            "disputes" => Ok(Self::Disputes),
            "fees" => Ok(Self::Fees),
            "fx" => Ok(Self::Fx),
            "legacy" => Ok(Self::Legacy),
            "misc" => Ok(Self::Misc),
            "other" => Ok(Self::Other),
            "payments" => Ok(Self::Payments),
            "refunds" => Ok(Self::Refunds),
            "reserves" => Ok(Self::Reserves),
            "transfers" => Ok(Self::Transfers),
            "wallets_and_balance" => Ok(Self::WalletsAndBalance),
            "withdrawals" => Ok(Self::Withdrawals),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveFinancialReportsResponseRowsItemGrouping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Advertising => write!(f, "advertising"),
            Self::AffiliatesAndRevshare => write!(f, "affiliates_and_revshare"),
            Self::Airdrops => write!(f, "airdrops"),
            Self::BadDebt => write!(f, "bad_debt"),
            Self::Balance => write!(f, "balance"),
            Self::Clawbacks => write!(f, "clawbacks"),
            Self::ConnectedAccounts => write!(f, "connected_accounts"),
            Self::Crypto => write!(f, "crypto"),
            Self::Disputes => write!(f, "disputes"),
            Self::Fees => write!(f, "fees"),
            Self::Fx => write!(f, "fx"),
            Self::Legacy => write!(f, "legacy"),
            Self::Misc => write!(f, "misc"),
            Self::Other => write!(f, "other"),
            Self::Payments => write!(f, "payments"),
            Self::Refunds => write!(f, "refunds"),
            Self::Reserves => write!(f, "reserves"),
            Self::Transfers => write!(f, "transfers"),
            Self::WalletsAndBalance => write!(f, "wallets_and_balance"),
            Self::Withdrawals => write!(f, "withdrawals"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
